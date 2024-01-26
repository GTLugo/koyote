#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::sync::Arc;
use koyote::prelude::*;
use koyote::graphics::pipeline::{RenderPipelineBuilder, RenderPipelineConfig};
use koyote::graphics::shader::builder::ShaderBuilder;
use koyote::graphics::shader::ShaderCreateInfo;
use koyote::graphics::shader::stage::Stage;

fn main() {
  log::init_debug(Some(Level::Trace));

  Koyote::builder()
    .with_title("Koyote")
    .with_size(800, 450)
    .with_centered(true)
    .with_tick_rate(128.)
    .run::<App>();
}

struct App {}

impl Runnable for App {
  #[allow(unused)]
  fn setup(koyote: &mut Koyote) -> Self {
    Self::initialize_render_data(koyote)?;

    let static_mesh_data = (
      Arc::new([
        [-0.5, -0.5, 1.0],
        [0.5, -0.5, 1.0],
        [0.5, 0.5, 1.0],
        [-0.5, 0.5, 1.0],
      ]),
      Arc::new([
        0, 1, 2,
        2, 3, 0
      ])
    );

    Self {}
  }

  fn update(&mut self, koyote: &mut Koyote) {
    self.intentional_crash(koyote);
  }
}

impl App {
  fn initialize_render_data(koyote: &mut Koyote) {
    // koyote::register_shader!(koyote.graphics, "simple/simple", "../res/shaders/simple.hlsl");

    koyote.graphics().set_render_pipeline(
      RenderPipelineBuilder::new(&koyote.graphics)
        .with_shader(
          ShaderCreateInfo {
            path: "../res/shaders/simple.hlsl".into(),
            stages: [Stage::Vertex, Stage::Fragment].into(),
          }
        )
        .with_shader(
          ShaderBuilder::new("../res/shaders/simple.hlsl".into())
            .with_stages(&[Stage::Vertex, Stage::Fragment])
        )
        .with_config(RenderPipelineConfig::new(koyote.graphics().window().size()))
        .build()?
    );
  }

  // fn intentional_crash(&mut self, koyote: &mut Koyote) {
  //   if koyote.input().key_down(KeyCode::F) && koyote.input().modifiers_down(Modifiers::Ctrl | Modifiers::Alt) {
  //     koyote.exit(AppError::fatal_str("Not fox, I am coyote!").into())
  //   }
  // }
}