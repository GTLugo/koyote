mod debug;
pub mod window;
pub mod context;
pub mod shader;
pub mod buffer;
pub mod image;
pub mod pipeline;
mod swapchain;

use anyhow::{Result};
use bevy_ecs::prelude::Resource;
use tracing::{trace};
use winit::event_loop::EventLoop;
use crate::graphics::pipeline::RenderPipeline;

use self::{window::Window, context::RenderContext};

#[allow(unused)]
#[derive(Resource)]
pub struct Graphics {
  pipeline: Option<RenderPipeline>,
  window: Window,
  context: RenderContext,
}

pub struct GraphicsCreateInfo<'e> {
  pub event_loop: &'e EventLoop<()>,
  pub title: &'static str,
  pub width: u32,
  pub height: u32,
  pub centered: bool,
}

impl Graphics {
  pub fn new(create_info: GraphicsCreateInfo) -> Result<Self> {
    trace!("Initializing Graphics...");

    let mut window = Window::new(
      create_info.event_loop,
      create_info.title,
      create_info.width,
      create_info.height,
    )?;
    if create_info.centered {
      window.center_on_monitor();
    }

    let context = RenderContext::new(&mut window)?;
    window.set_visible(true);

    trace!("Graphics ready!");

    Ok(Self {
      pipeline: None,
      window,
      context,
    })
  }

  pub fn window(&self) -> &Window {
    &self.window
  }

  pub fn window_mut(&mut self) -> &mut Window {
    &mut self.window
  }

  // pub fn create_render_pass<RP: RenderPass + 'static>(&mut self, label: Option<&'static str>, shader: Arc<Shader>) -> RP {
  //   RP::new(label, shader, self)
  // }

  pub fn set_render_pipeline(&mut self, render_pipeline: RenderPipeline) {
    self.pipeline = Some(render_pipeline);
  }

  // pub fn set_render_pipeline_2(&mut self, pipeline_config: RenderPipelineConfiguration) -> Result<()> {
  //   self.pipeline = RenderPipeline::new(&self.context, pipeline_config)
  //     .context("failed to create render pipeline")
  //     .ok();
  //   Ok(())
  // }

  pub(crate) fn render_frame(&mut self) {}

  pub(crate) fn reset_frame(&mut self) {}

  pub fn context(&self) -> &RenderContext {
    &self.context
  }
}

impl Graphics {
  unsafe fn free(&mut self) {
    trace!("Cleaning up Graphics");
  }
}

impl Drop for Graphics {
  fn drop(&mut self) {
    unsafe {
      self.free();
    }
  }
}