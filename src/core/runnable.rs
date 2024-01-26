use crate::core::{
  event::{InputEvent, WindowEvent},
  framework::Koyote,
  flow::Flow,
};

#[allow(unused)]
pub trait Runnable {
  fn setup(koyote: &mut Koyote) -> Self;

  fn start(&mut self, koyote: &mut Koyote) {}

  fn fixed_update(&mut self, koyote: &mut Koyote) {}

  fn update(&mut self, koyote: &mut Koyote) {}

  fn late_update(&mut self, koyote: &mut Koyote) {}

  fn stop(&mut self, koyote: &mut Koyote) -> Flow {
    Default::default()
  }

  fn shutdown(&mut self, koyote: &mut Koyote) {}

  fn window(&mut self, event: WindowEvent, koyote: &mut Koyote) {}

  fn input(&mut self, event: InputEvent, koyote: &mut Koyote) {}
}

// EXAMPLE
//
// fn input(&mut self, event: InputEvent, _: &mut Graphics, input: &Input, _: &Time) {
//   match event {
//     InputEvent::Keyboard(c, s) => {
//       tracing::info!("{c:?}: {s:?}");
//     }
//     InputEvent::Cursor | InputEvent::Scroll | InputEvent::Modifiers(..) => { }
//     _ => tracing::info!("input")
//   }
// }