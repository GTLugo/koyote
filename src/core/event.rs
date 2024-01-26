use enumflags2::BitFlags;
use crate::{
  input::modifier::Modifiers,
  prelude::{ButtonState, KeyCode, MouseCode},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowEvent {
  Moved,
  Resized,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
  Mouse(MouseCode, ButtonState),
  Keyboard(KeyCode, ButtonState),
  Modifiers(BitFlags<Modifiers>),
  Cursor,
  Scroll,
}