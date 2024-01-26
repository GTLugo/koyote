use std::{collections::HashSet, env, fs::File, io::{Read, Write}, path::Path, path::PathBuf};
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context, Result};
use ash::vk;
use strum::IntoEnumIterator;
use tracing::{debug, error, trace};

use crate::graphics::shader::lang::Source;
use crate::graphics::shader::stage::Stage;

use super::context::RenderContext;

pub mod stage;
pub mod lang;
pub mod builder;

#[derive(Default, Clone)]
pub struct ShaderCreateInfo {
  pub path: PathBuf,
  pub stages: HashSet<Stage>,
}

#[derive(Default, Clone)]
pub struct ShaderSource {
  pub path: PathBuf,
  pub stages: HashMap<Stage, Source>,
}

// loading should first query if shader is already loaded in memory, but how?
impl ShaderSource {
  pub fn has_stage(&self, stage: Stage) -> bool {
    self.stages.contains_key(&stage)
  }

  pub fn compile(self) -> Shader {
    todo!()
  }
}

pub struct Shader {
  pub device: Arc<ash::Device>,
  pub modules: HashMap<Stage, vk::ShaderModule>,
}

impl Shader {
  pub fn new(
    create_info: ShaderCreateInfo,
  ) -> Result<Self> {
    // load modules
    debug!("[{:?}] Loading shader...", &create_info.path);
    match Self::fetch_shader_bytecode(&create_info) {
      Ok(result) => {
        trace!("[{:?}] Building modules...", &create_info.path);
        let shader_modules = Self::build_shader_modules(context, &create_info, result)?;
        Ok(Self {
          device: context.device(),
          shader_modules,
        })
      }
      Err(err) => Err(err),
    }
  }

  pub fn has_stage(&self, stage: Stage) -> bool {
    self.shader_modules.contains_key(&stage)
  }

  pub fn shader_modules(&self) -> &HashMap<Stage, vk::ShaderModule> {
    &self.shader_modules
  }

  pub fn pipeline_shader_info(&self) -> Vec<vk::PipelineShaderStageCreateInfo> {
    let mut create_info: Vec<vk::PipelineShaderStageCreateInfo> = Default::default();
    for (stage, module) in self.shader_modules.iter() {
      create_info.push(
        vk::PipelineShaderStageCreateInfo {
          stage: (*stage).into(),
          module: *module,
          p_name: stage.entry_point().as_ptr(),
          ..Default::default()
        }
      )
    }
    create_info
  }

  pub fn compile(self, context: &RenderContext) -> Self {
    if let Shader::Source { lang, path, source, stages } = &self {
      Self::Compiled {
        device: context.device(),
        shader_modules: Default::default(),
      }
    } else {
      self
    }
  }

  // pub fn entry_point(&self, stage: ShaderStage) -> Option<EntryPoint> {
  //   match self.shader_modules.get(&stage) {
  //     Some(module) => module.entry_point(stage.entry_point().as_str()),
  //     None => None
  //   }
  // }
}

impl Shader {
  unsafe fn free(&mut self) {
    for module in self.modules.values() {
      self.device.destroy_shader_module(*module, None);
    }
  }

  fn fetch_shader_bytecode(info: &ShaderCreateInfo) -> Result<HashMap<Stage, Vec<u8>>> {
    let shader_cache_dir = Self::shader_cache_dir(info)?;
    let mut stage_bytecodes: HashMap<Stage, Vec<u8>> = Default::default();

    for stage in Stage::iter().filter(|s| info.has_stage(*s)) {
      match Self::fetch_stage_bytecode(info, stage, &shader_cache_dir) {
        Ok(bytecode) => {
          stage_bytecodes.insert(stage, bytecode);
        }
        Err(err) => Err(err)?
      }
    }

    Ok(stage_bytecodes)
  }

  fn fetch_stage_bytecode(info: &ShaderCreateInfo, stage: Stage, shader_cache_dir: &Path) -> Result<Vec<u8>> {
    let cached_stage_file_path = shader_cache_dir.join(stage.to_string()).with_extension("spv");

    if cached_stage_file_path.exists() && Self::cached_file_younger_than_exe(&cached_stage_file_path)? {
      trace!("[{stage:?}] Reading cached stage...");
      match File::open(&cached_stage_file_path) {
        Ok(file) => Ok(file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>()),
        Err(err) => Err(err)?
      }
    } else {
      trace!("[{stage:?}] Recompiling stage...");
      Self::compile_shader_type(&info.source, stage, info.lang, &cached_stage_file_path)
    }
  }

  fn cached_file_younger_than_exe(cached_file: &Path) -> Result<bool> {
    let file_age = cached_file
      .metadata().context("failed to access cached file metadata")?
      .modified().context("failed to access cached file metadata")?;
    let exe_age = env::current_exe()?
      .metadata().context("failed to access current_exe metadata")?
      .modified().context("failed to access current_exe metadata")?;
    // debug!("File age: [{file_age:?}], Exe age: [{exe_age:?}]");
    Ok(file_age >= exe_age)
  }

  fn compile_shader_type(source: &str, stage: Stage, lang: ShaderLang, cached_stage_file_path: &Path) -> Result<Vec<u8>> {
    let compiler = shaderc::Compiler::new().context("failed to initialize shaderc compiler")?;
    let mut options = shaderc::CompileOptions::new().context("failed to initialize shaderc compiler options")?;

    options.set_source_language(match lang {
      ShaderLang::GLSL => shaderc::SourceLanguage::GLSL,
      ShaderLang::HLSL => shaderc::SourceLanguage::HLSL,
    });

    /* TODO:
        So this kills the release mode apparently LMAO. definitely something to look into...
        I should allow reading file directory as list of multiple individual shader files.
        This will allow for optimizations only if the shader is a multi-file shader.
    */
    //
    if cfg!(not(debug_assertions)) {
      options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    }

    match compiler.compile_into_spirv(
      source,
      stage.into(),
      cached_stage_file_path
        .file_name().context("failed to access file_name")?
        .to_str().context("failed to convert file_name to str")?,
      stage.to_entry_point_string().as_str(),
      Some(&options),
    ) {
      Ok(result) => {
        trace!("[{stage:?}] Compiled stage.");

        match File::create(cached_stage_file_path) {
          Ok(mut file) => {
            match file.write_all(result.as_binary_u8()) {
              Ok(_) => trace!("[{stage:?}] Cached stage."),
              Err(_) => error!("[{stage:?}] Failed to write stage to shader cache.")
            }
          }
          Err(_) => error!("[{stage:?}] Failed to write stage to shader cache.")
        }

        Ok(result.as_binary_u8().into())
      }
      Err(err) => Err(err.into())
    }
  }

  fn build_shader_modules(
    context: &RenderContext,
    info: &ShaderCreateInfo,
    mut stage_bytecodes: HashMap<Stage, Vec<u8>>,
  ) -> Result<HashMap<Stage, vk::ShaderModule>> {
    let mut shader_modules: HashMap<Stage, vk::ShaderModule> = Default::default();

    for stage in Stage::iter().filter(|s| info.has_stage(*s)) {
      if let Some(bytecode) = stage_bytecodes.get_mut(&stage) {
        let shader_module = unsafe {
          let mut attempt = 0;

          loop {
            let shader_module_create_info = vk::ShaderModuleCreateInfo {
              code_size: bytecode.len(),
              p_code: match bytemuck::try_cast_slice(bytecode) {
                Ok(value) => value.as_ptr(),
                Err(_) => anyhow::bail!("failed to cast bytes"),
              },
              ..Default::default()
            };

            // ShaderModule::from_bytes(device.device(), bytecode)
            match context.device().create_shader_module(&shader_module_create_info, None) {
              Ok(module) => break module,
              Err(err) => {
                if attempt >= 2 {
                  anyhow::bail!("Could not recover from shader module creation failure ({err})");
                }

                error!("Shader module creation failure, attempting to recompile ({err})");
                let shader_cache_dir = Self::shader_cache_dir(info).unwrap();
                let cached_stage_file_path = shader_cache_dir.with_file_name(stage.to_string()).with_extension("spv");
                if let Ok(code) = Self::compile_shader_type(&info.source, stage, info.lang, &cached_stage_file_path) {
                  bytecode.clear();
                  bytecode.extend(code);
                };

                attempt += 1;
              }
            }
          }
        };

        shader_modules.insert(stage, shader_module);
      }
    }

    debug!("[{:?}] Loaded shader.", &info.path);
    Ok(shader_modules)
  }
}

impl Drop for Shader {
  fn drop(&mut self) {
    unsafe {
      self.free();
    }
  }
}


// #[macro_export]
// macro_rules! shader_builder {
//   ($file_name:literal) => {{
//     let source = include_str![$file_name];
//     $crate::graphics::shader::ShaderBuilder::new(std::path::PathBuf::from($file_name), source.into())
//   }};
// }
//
// #[macro_export]
// macro_rules! include_shader {
//   ($graphics:expr, $file_name:literal) => {{
//     $crate::shader_builder![$file_name]
//       .detect_stages()
//       .build($graphics.context())
//   }};
// }
//
// #[macro_export]
// macro_rules! register_shader {
//   ($graphics:expr, $key:literal, $file_name:literal) => {{
//     $graphics.register_shader_result(
//       $key,
//       $crate::include_shader!($graphics, $file_name)
//     )
//   }};
// }
