use std::{
  env,
  fs,
  fs::File,
  io::Read,
  path::PathBuf,
};

use anyhow::{Context, Result};
use strum::{Display, EnumIter};
use tracing::{error, trace};

use crate::graphics::shader::stage::Stage;

#[derive(EnumIter, Display, Clone, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Source {
  GLSL(String),
  HLSL(String),
  SPIRV(String),
}

// TODO: If shader is SPIRV, it should skip cache check since there won't be anything there
impl Source {
  pub(crate) fn read_glsl() -> Result<Self> {
    unimplemented!()
  }

  pub(crate) fn read_hlsl(stage: Stage, path: &PathBuf) -> Result<Self> {
    match Self::find_in_cache(stage, path) {
      Ok(source) => Ok(Self::HLSL(source)),
      Err(err) => {
        error!("{err}");
        todo!("read file from original source")
      }
    }
  }

  pub fn as_str(&self) -> &str {
    match self {
      Source::GLSL(source) => source.as_str(),
      Source::HLSL(source) => source.as_str(),
      Source::SPIRV(source) => source.as_str(),
    }
  }

  pub fn into_bytes(self) -> Vec<u8> {
    match self {
      Source::GLSL(source) => source.into_bytes(),
      Source::HLSL(source) => source.into_bytes(),
      Source::SPIRV(source) => source.into_bytes(),
    }
  }
}

impl Source {
  fn find_in_cache(stage: Stage, path: &PathBuf) -> Result<String> {
    let shader_cache_dir = Self::shader_cache_dir(path)?;
    let cached_stage_file_path = shader_cache_dir.join(stage.to_string()).with_extension("spv");

    if cached_stage_file_path.exists() && Self::cached_file_younger_than_exe(&cached_stage_file_path)? {
      trace!("[{stage:?}] Reading cached stage...");
      match File::open(&cached_stage_file_path) {
        Ok(mut file) => {
          let mut string = String::new();
          file.read_to_string(&mut string).context("failed to read stage to string")?;
          Ok(string)
        }
        Err(err) => Err(err.into())
      }
    } else {
      Err(anyhow::Error::msg(format!("failed to find cached stage: {}", stage)))
    }
  }

  fn shader_cache_dir(path: &PathBuf) -> Result<PathBuf> {
    let runtime_dir = env::current_exe()?
      .parent().context("failed to access current_exe parent dir")?
      .join("tmp/res/shaders");
    let shader_cache_dir = runtime_dir.join(
      path.file_stem().context("failed to read shader dir")?
    );

    // create cache
    if !shader_cache_dir.exists() {
      fs::create_dir_all(&shader_cache_dir).context("failed to create shader cache")?;
    }

    Ok(shader_cache_dir)
  }

  fn cached_file_younger_than_exe(cached_file: &PathBuf) -> Result<bool> {
    let file_age = cached_file
      .metadata().context("failed to access cached file metadata")?
      .modified().context("failed to access cached file metadata")?;
    let exe_age = env::current_exe()?
      .metadata().context("failed to access current_exe metadata")?
      .modified().context("failed to access current_exe metadata")?;
    // debug!("File age: [{file_age:?}], Exe age: [{exe_age:?}]");
    Ok(file_age >= exe_age)
  }

  // for stage in stages {
  //   match Self::read_stage_bytecode(&shader_cache_dir, stage) {
  //     Ok(bytecode) => {
  //       stage_bytecodes.insert(stage.clone(), bytecode.into());
  //     }
  //     Err(err) => Err(err)?
  //   }
  // }

  // fn read_stage_bytecode(shader_cache_dir: &PathBuf, stage: &Stage) -> Result<String> {
  //   let cached_stage_file_path = shader_cache_dir.join(stage.to_string()).with_extension("spv");
  //
  //   if cached_stage_file_path.exists() && Self::cached_file_younger_than_exe(&cached_stage_file_path)? {
  //     trace!("[{stage:?}] Reading cached stage...");
  //     match File::open(&cached_stage_file_path) {
  //       Ok(mut file) => {
  //         let mut string = String::new();
  //         file.read_to_string(&mut string).context("failed to read stage to string")?;
  //         Ok(string)
  //       },
  //       Err(err) => Err(err.into())
  //     }
  //   } else {
  //     trace!("[{stage:?}] Recompiling stage...");
  //     Err(anyhow::Error::msg(format!("failed to find cached stage: {}", stage)))
  //   }
  // }
}

impl Default for Source {
  fn default() -> Self {
    Self::HLSL("".to_string())
  }
}