pub mod mouse;
pub mod key;
pub mod button;
pub mod modifier;

use std::collections::HashMap;
use bevy_ecs::prelude::*;
use enumflags2::BitFlags;
use strum::IntoEnumIterator;
use winit::event::{ElementState, ModifiersState, MouseButton, VirtualKeyCode};
use crate::{
  input::{
    modifier::Modifiers,
    key::KeyCode,
    button::ButtonState,
    mouse::MouseCode,
  }
};

#[derive(Debug, Resource)]
pub struct Input {
  mouse_buttons: HashMap<MouseCode, ButtonState>,
  keys: HashMap<KeyCode, ButtonState>,
  modifiers: BitFlags<Modifiers>,
}

impl Input {
  pub fn new() -> Self {
    let mouse_buttons = {
      let mut map = HashMap::default();
      for code in MouseCode::iter() {
        map.insert(code, ButtonState::Released);
      }
      map
    };

    let keys = {
      let mut map = HashMap::default();
      for code in KeyCode::iter() {
        map.insert(code, ButtonState::Released);
      }
      map
    };

    let modifiers = Default::default();

    Self {
      mouse_buttons,
      keys,
      modifiers,
    }
  }

  // KEYBOARD

  pub fn key_state(&self, code: KeyCode) -> ButtonState {
    self.keys.get(&code).copied().unwrap_or(ButtonState::Released)
  }

  pub fn key_down(&self, code: KeyCode) -> bool {
    !matches!(self.key_state(code), ButtonState::Released)
  }

  pub(crate) fn update_key_state(&mut self, keycode: VirtualKeyCode, state: ElementState) -> ButtonState {
    if let Some(key_state) = self.keys.get_mut(&keycode.into()) {
      *key_state = match state {
        ElementState::Pressed => match key_state {
          ButtonState::Pressed => ButtonState::Held,
          ButtonState::Held => ButtonState::Held,
          ButtonState::Released => ButtonState::Pressed,
        },
        ElementState::Released => ButtonState::Released,
      };
      *key_state
    } else {
      ButtonState::Released
    }
  }

  // MOUSE

  pub fn mouse_button_state(&self, code: MouseCode) -> ButtonState {
    self.mouse_buttons.get(&code).copied().unwrap_or(ButtonState::Released)
  }

  pub fn mouse_button_down(&self, code: MouseCode) -> bool {
    !matches!(self.mouse_button_state(code), ButtonState::Released)
  }

  pub(crate) fn update_mouse_button_state(&mut self, button: MouseButton, state: ElementState) -> ButtonState {
    if let Some(mouse_state) = self.mouse_buttons.get_mut(&button.into()) {
      *mouse_state = match state {
        ElementState::Pressed => {
          match mouse_state {
            ButtonState::Pressed => ButtonState::Held,
            ButtonState::Held => ButtonState::Held,
            ButtonState::Released => ButtonState::Pressed,
          }
        }
        ElementState::Released => ButtonState::Released,
      };
      *mouse_state
    } else {
      ButtonState::Released
    }
  }

  // MODS

  pub fn modifiers_state(&self) -> BitFlags<Modifiers> {
    self.modifiers
  }

  pub fn modifier_down(&self, modifier: Modifiers) -> bool {
    self.modifiers.contains(modifier)
  }

  pub fn modifiers_down(&self, modifiers: BitFlags<Modifiers>) -> bool {
    self.modifiers.contains(modifiers)
  }

  pub(crate) fn update_modifiers_state(&mut self, modifiers: ModifiersState) -> BitFlags<Modifiers> {
    // TODO: just swap to bit manipulation to speed up. Stop being lazy, Gabriel.
    for modifier in Modifiers::iter() {
      if modifiers.contains(modifier.into()) != self.modifiers.contains(modifier) {
        self.modifiers.toggle(modifier);
      }
    }

    self.modifiers
  }
}

impl Default for Input {
  fn default() -> Self {
    Self::new()
  }
}

