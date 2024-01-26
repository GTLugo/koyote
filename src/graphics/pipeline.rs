use std::marker::PhantomData;
use std::sync::Arc;
use anyhow::{Context, Result};
use ash::vk;
use crate::{
  graphics::{
    context::RenderContext,
    shader::Shader,
  }
};
use crate::graphics::Graphics;
use crate::graphics::shader::builder::{ShaderBuilder, ShaderStagesSpecified};
use crate::graphics::shader::ShaderCreateInfo;

pub struct RenderPipeline {
  device: Arc<ash::Device>,
  pipeline: vk::Pipeline,
  config: RenderPipelineConfig,
  shader: Shader,
}

impl RenderPipeline {
  pub fn new(context: &RenderContext, shader_info: ShaderCreateInfo, config: RenderPipelineConfig) -> Result<Self> {
    let shader = shader_info.compile();
    let pipeline = Self::create_graphics_pipeline(context, &shader, &config)?;

    Ok(Self {
      device: context.device(),
      pipeline,
      config,
      shader,
    })
  }
}

impl RenderPipeline {
  unsafe fn free(&mut self) {
    self.device.destroy_pipeline(self.pipeline, None);
  }

  fn create_graphics_pipeline(context: &RenderContext, shader: &Shader, config: &RenderPipelineConfig) -> Result<vk::Pipeline> {
    if config.pipeline_layout == vk::PipelineLayout::null() {
      anyhow::bail!("pipeline layout is null") // TODO: Add to builder
    }

    if config.render_pass == vk::RenderPass::null() {
      anyhow::bail!("render pass is null") // TODO: Add to builder
    }

    // TODO: Overhaul pipeline creation to make it more type-driven

    let shader_stage_create_infos = shader.pipeline_shader_info();

    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo {
      vertex_binding_description_count: 0,
      // p_vertex_binding_descriptions: (),
      vertex_attribute_description_count: 0,
      // p_vertex_attribute_descriptions: (),
      ..Default::default()
    };

    let pipeline_create_info = vk::GraphicsPipelineCreateInfo {
      stage_count: shader_stage_create_infos.len() as u32,
      p_stages: shader_stage_create_infos.as_ptr(),
      p_vertex_input_state: &vertex_input_info,
      p_input_assembly_state: &config.input_assembly_info,
      // p_tessellation_state: &config.input_assembly_info,
      p_viewport_state: &config.viewport_info,
      p_rasterization_state: &config.rasterization_info,
      p_multisample_state: &config.multisample_info,
      p_depth_stencil_state: &config.depth_stencil_info,
      p_color_blend_state: &config.color_blend_info,
      // p_dynamic_state: (),
      layout: config.pipeline_layout,
      render_pass: config.render_pass,
      subpass: config.subpass,
      // base_pipeline_handle: Default::default(),
      // base_pipeline_index: 0,
      ..Default::default()
    };

    unsafe {
      context.device().create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_create_info], None)
        .map(|pipelines| pipelines[0])
        .map_err(|err| anyhow::anyhow!("failed to create graphics pipelines: {err:?}"))
    }
  }
}

impl Drop for RenderPipeline {
  fn drop(&mut self) {
    unsafe {
      self.free();
    }
  }
}

#[derive(Default)]
pub struct RenderPipelineConfig {
  pub viewport: vk::Viewport,
  pub scissor: vk::Rect2D,
  pub viewport_info: vk::PipelineViewportStateCreateInfo,
  pub input_assembly_info: vk::PipelineInputAssemblyStateCreateInfo,
  pub rasterization_info: vk::PipelineRasterizationStateCreateInfo,
  pub multisample_info: vk::PipelineMultisampleStateCreateInfo,
  pub color_blend_attachment: vk::PipelineColorBlendAttachmentState,
  pub color_blend_info: vk::PipelineColorBlendStateCreateInfo,
  pub depth_stencil_info: vk::PipelineDepthStencilStateCreateInfo,
  pub pipeline_layout: vk::PipelineLayout,
  pub render_pass: vk::RenderPass,
  pub subpass: u32,
}

unsafe impl Send for RenderPipelineConfig {}

unsafe impl Sync for RenderPipelineConfig {}

impl RenderPipelineConfig {
  pub fn new((width, height): (u32, u32)) -> Self {
    let viewport = vk::Viewport {
      x: 0.0,
      y: 0.0,
      width: width as f32,
      height: height as f32,
      min_depth: 0.0,
      max_depth: 1.0,
    };

    let scissor = vk::Rect2D {
      offset: vk::Offset2D {
        x: 0,
        y: 0,
      },
      extent: vk::Extent2D {
        width,
        height,
      },
    };

    let viewport_info = vk::PipelineViewportStateCreateInfo {
      viewport_count: 1,
      p_viewports: &viewport,
      scissor_count: 1,
      p_scissors: &scissor,
      ..Default::default()
    };

    let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo {
      topology: vk::PrimitiveTopology::TRIANGLE_LIST,
      primitive_restart_enable: vk::FALSE,
      ..Default::default()
    };

    let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
      depth_clamp_enable: vk::FALSE,
      rasterizer_discard_enable: vk::FALSE,
      polygon_mode: vk::PolygonMode::FILL,
      cull_mode: vk::CullModeFlags::BACK,
      front_face: vk::FrontFace::COUNTER_CLOCKWISE,
      depth_bias_enable: vk::FALSE,
      depth_bias_constant_factor: 0.0,
      depth_bias_clamp: 0.0,
      depth_bias_slope_factor: 0.0,
      line_width: 1.0,
      ..Default::default()
    };

    let multisample_info = vk::PipelineMultisampleStateCreateInfo {
      rasterization_samples: vk::SampleCountFlags::TYPE_1,
      sample_shading_enable: vk::FALSE,
      min_sample_shading: 1.0,
      alpha_to_coverage_enable: vk::FALSE,
      alpha_to_one_enable: vk::FALSE,
      ..Default::default()
    };

    let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
      blend_enable: vk::TRUE,
      src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
      dst_color_blend_factor: vk::BlendFactor::DST_COLOR,
      color_blend_op: vk::BlendOp::ADD,
      src_alpha_blend_factor: vk::BlendFactor::SRC_ALPHA,
      dst_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
      alpha_blend_op: vk::BlendOp::ADD,
      color_write_mask: vk::ColorComponentFlags::RGBA,
    };

    let color_blend_info = vk::PipelineColorBlendStateCreateInfo {
      logic_op_enable: vk::FALSE,
      logic_op: vk::LogicOp::COPY,
      attachment_count: 1,
      p_attachments: &color_blend_attachment,
      blend_constants: [
        0.0,
        0.0,
        0.0,
        0.0,
      ],
      ..Default::default()
    };

    let depth_stencil_info = vk::PipelineDepthStencilStateCreateInfo {
      depth_test_enable: vk::TRUE,
      depth_write_enable: vk::TRUE,
      depth_compare_op: vk::CompareOp::LESS,
      depth_bounds_test_enable: vk::FALSE,
      stencil_test_enable: vk::FALSE,
      min_depth_bounds: 0.0,
      max_depth_bounds: 1.0,
      ..Default::default()
    };

    let pipeline_layout = vk::PipelineLayout::null();

    let render_pass = vk::RenderPass::null();

    let subpass: u32 = 0;

    Self {
      viewport,
      scissor,
      viewport_info,
      input_assembly_info,
      rasterization_info,
      multisample_info,
      color_blend_attachment,
      color_blend_info,
      depth_stencil_info,
      pipeline_layout,
      render_pass,
      subpass,
    }
  }
}

#[derive(Default, Clone)]
pub struct ShaderMissing;

#[derive(Default, Clone)]
pub struct ShaderSpecified;

#[derive(Default, Clone)]
pub struct ConfigMissing;

#[derive(Default, Clone)]
pub struct ConfigSpecified;

pub struct RenderPipelineBuilder<'c, S, C> {
  context: &'c RenderContext,
  shader_specified: PhantomData<S>,
  shader: Option<ShaderCreateInfo>,
  config_specified: PhantomData<C>,
  config: Option<RenderPipelineConfig>,
}

impl<'c> RenderPipelineBuilder<'c, ShaderMissing, ConfigMissing> {
  pub fn new(graphics: &'c Graphics) -> Self {
    Self {
      context: graphics.context(),
      shader_specified: PhantomData,
      shader: None,
      config_specified: PhantomData,
      config: None,
    }
  }
}

impl<'c, C> RenderPipelineBuilder<'c, ShaderMissing, C> {
  pub fn with_shader(self, shader_info: ShaderCreateInfo) -> RenderPipelineBuilder<'c, ShaderSpecified, C> {
    if let Ok(shader_info) = shader.build() {
      RenderPipelineBuilder {
        context: self.context,
        shader_specified: PhantomData,
        shader: Some(shader_info),
        config_specified: PhantomData,
        config: self.config,
      }
    } else {
      todo!("must return default shader")
    }
  }

  // pub fn with_shader_builder(self, shader_builder: ShaderBuilder<ShaderStagesSpecified>) -> RenderPipelineBuilder<'c, ShaderSpecified, C> {
  //   if let Ok(shader_info) = shader_builder.build_hlsl() {
  //     RenderPipelineBuilder {
  //       context: self.context,
  //       shader_specified: PhantomData,
  //       shader: Some(shader_info),
  //       config_specified: PhantomData,
  //       config: self.config,
  //     }
  //   } else {
  //     todo!("must return default shader")
  //   }
  // }
}

impl<'c, S> RenderPipelineBuilder<'c, S, ConfigMissing> {
  pub fn with_config(self, config: RenderPipelineConfig) -> RenderPipelineBuilder<'c, S, ConfigSpecified> {
    RenderPipelineBuilder {
      context: self.context,
      shader_specified: PhantomData,
      shader: self.shader,
      config_specified: PhantomData,
      config: Some(config),
    }
  }
}

impl<'c> RenderPipelineBuilder<'c, ShaderSpecified, ConfigSpecified> {
  pub fn build(self) -> Result<RenderPipeline> {
    RenderPipeline::new(
      self.context,
      self.shader.unwrap(),
      self.config.unwrap(),
    ).context("failed to create render pipeline")
  }
}