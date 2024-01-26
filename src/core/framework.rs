use std::sync::OnceLock;
use bevy_ecs::prelude::*;
use tracing::{error, info, trace};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{
  core::{
    event::{WindowEvent, InputEvent},
    time::Time,
    runnable::Runnable,
  },
  graphics::Graphics,
  input::Input,
  log,
};
use crate::core::flow::Flow;
use crate::graphics::GraphicsCreateInfo;
use crate::log::Level;

pub struct Koyote {
  pub world: World,
  flow: Flow,
}

static KOYOTE: OnceLock<Koyote> = OnceLock::new();

impl Koyote {
  pub fn builder() -> FrameworkBuilder {
    FrameworkBuilder::default()
  }

  // pub fn framework() -> &'static Self {
  //   KOYOTE.get().unwrap()
  // }

  pub fn exit(&mut self) {
    self.exit_with(None, None);
  }

  pub fn exit_with(&mut self, exit_code: Option<i32>, exit_error: Option<anyhow::Error>) {
    if let Some(error) = exit_error {
      error!("FATAL | RUNTIME | {:#}", error);
    }

    self.flow = if let Some(code) = exit_code {
      Flow::Exit(code)
    } else {
      Flow::SUCCESS
    }
  }

  fn run<App: 'static + Runnable>(mut self, event_loop: EventLoop<()>) {
    trace!("Beginning app setup.");
    let mut app = App::setup(&mut self);

    trace!("Entering Koyote Framework loop.");
    info!("Kon-Koyo!");

    app.start(&mut self);
    event_loop.run(move |event, _, control_flow| {
      if let Flow::Exit(code) = self.flow {
        *control_flow = ControlFlow::ExitWithCode(code);
      }

      match event {
        winit::event::Event::WindowEvent { window_id: _, event } => {
          match event {
            winit::event::WindowEvent::CloseRequested => {
              match app.stop(&mut self) {
                Flow::Exit(code) => *control_flow = ControlFlow::ExitWithCode(code),
                Flow::Continue => *control_flow = ControlFlow::Poll,
              }
            }
            winit::event::WindowEvent::Resized(_) => {
              app.window(WindowEvent::Resized, &mut self);
            }
            winit::event::WindowEvent::Moved(_) => {
              app.window(WindowEvent::Moved, &mut self);
            }
            winit::event::WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
              if let Some(keycode) = input.virtual_keycode {
                let state = self.world.resource_mut::<Input>().update_key_state(keycode, input.state);
                app.input(InputEvent::Keyboard(keycode.into(), state), &mut self);
              }
            }
            winit::event::WindowEvent::ModifiersChanged(mods) => {
              let mods = self.world.resource_mut::<Input>().update_modifiers_state(mods);
              app.input(InputEvent::Modifiers(mods), &mut self);
            }
            winit::event::WindowEvent::CursorMoved { device_id: _, position: _, .. } => {
              app.input(InputEvent::Cursor, &mut self);
            }
            winit::event::WindowEvent::MouseWheel { device_id: _, delta: _, phase: _, .. } => {
              app.input(InputEvent::Scroll, &mut self);
            }
            winit::event::WindowEvent::MouseInput { device_id: _, state, button, .. } => {
              let state = self.world.resource_mut::<Input>().update_mouse_button_state(button, state);
              app.input(InputEvent::Mouse(button.into(), state), &mut self);
            }
            _ => {}
          }
        }
        winit::event::Event::MainEventsCleared => {
          self.update(&mut app);
        }
        winit::event::Event::RedrawRequested(_) => {
          self.graphics_mut().render_frame();
        }
        winit::event::Event::RedrawEventsCleared => {
          self.graphics_mut().reset_frame();
        }
        winit::event::Event::LoopDestroyed => {
          info!("Otsu-Koyo!");
          app.shutdown(&mut self);
          trace!("Exiting Koyote Framework loop.");
        }
        _ => {}
      };
    });
  }

  // Note: Any errors will cause the entire frame to skip
  fn update<App: 'static + Runnable>(&mut self, app: &mut App) {
    self.world.resource_mut::<Time>().update();
    while self.world.resource::<Time>().should_do_tick() {
      self.world.resource_mut::<Time>().tick();
      app.fixed_update(self);
    }
    app.update(self);
    app.late_update(self);
    self.graphics().window().request_redraw();
  }

  pub fn time(&self) -> &Time {
    self.world.resource::<Time>()
  }

  pub fn input(&self) -> &Input {
    self.world.resource::<Input>()
  }

  pub fn graphics(&self) -> &Graphics {
    self.world.resource::<Graphics>()
  }

  pub fn graphics_mut(&mut self) -> &mut Graphics {
    self.world.resource_mut::<Graphics>().into_inner()
  }
}

pub struct FrameworkBuilder {
  pub title: &'static str,
  pub width: u32,
  pub height: u32,
  pub centered: bool,
  pub tick_rate: f64,
}

impl FrameworkBuilder {
  pub fn with_title(mut self, title: &'static str) -> Self {
    self.title = title;
    self
  }

  pub fn with_size(mut self, width: u32, height: u32) -> Self {
    self.width = width;
    self.height = height;
    self
  }
  pub fn with_centered(mut self, centered: bool) -> Self {
    self.centered = centered;
    self
  }

  pub fn with_tick_rate(mut self, tick_rate: f64) -> Self {
    self.tick_rate = tick_rate;
    self
  }

  pub fn log_init(self, framework_logging_level: Option<Level>) -> Self {
    log::init(framework_logging_level);
    self
  }

  pub fn run<App: 'static + Runnable>(self) {
    let mut world = World::new();

    world.insert_resource(Time::new(self.tick_rate, 1024));
    world.insert_resource(Input::default());

    let event_loop = EventLoop::new();
    let graphics = match Graphics::new(GraphicsCreateInfo {
      event_loop: &event_loop,
      title: self.title,
      width: self.width,
      height: self.height,
      centered: self.centered,
    }) {
      Ok(value) => value,
      Err(err) => {
        error!("FATAL | GRAPHICS SETUP | {err:#}");
        return;
      }
    };

    world.insert_resource(graphics);

    Koyote {
      world,
      flow: Default::default(),
    }.run::<App>(event_loop);
  }
}

impl Default for FrameworkBuilder {
  fn default() -> Self {
    Self {
      title: "Koyote",
      width: 800,
      height: 500,
      centered: false,
      tick_rate: 128.,
    }
  }
}