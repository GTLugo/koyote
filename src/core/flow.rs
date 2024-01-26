#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Flow {
  Exit(i32),
  Continue,
}

impl Flow {
  pub const SUCCESS: Flow = Flow::Exit(0);
  pub const FAILURE: Flow = Flow::Exit(1);

  pub fn exit_code(&self) -> i32 {
    match &self {
      Flow::Exit(code) => *code,
      Flow::Continue => 0,
    }
  }
}

impl Default for Flow {
  fn default() -> Self {
    Flow::Exit(0)
  }
}