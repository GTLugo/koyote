#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ButtonState {
  Pressed,
  Held,
  Released,
}