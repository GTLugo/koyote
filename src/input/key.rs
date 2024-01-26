use strum::EnumIter;
use winit::event::VirtualKeyCode;

#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum KeyCode {
  Unknown = 0,
  // ASCII
  Tab = 9,
  Enter = 10,
  Space = 32,
  Apostrophe = 39,
  Comma = 44,
  Minus = 45,
  Period = 46,
  ForwardSlash = 47,
  _0 = 48,
  _1 = 49,
  _2 = 50,
  _3 = 51,
  _4 = 52,
  _5 = 53,
  _6 = 54,
  _7 = 55,
  _8 = 56,
  _9 = 57,
  Semicolon = 59,
  Equals = 61,
  A = 65,
  B = 66,
  C = 67,
  D = 68,
  E = 69,
  // ;)
  F = 70,
  G = 71,
  H = 72,
  I = 73,
  J = 74,
  K = 75,
  L = 76,
  M = 77,
  N = 78,
  O = 79,
  P = 80,
  Q = 81,
  R = 82,
  S = 83,
  T = 84,
  U = 85,
  V = 86,
  W = 87,
  X = 88,
  Y = 89,
  Z = 90,
  LeftBracket = 91,
  BackSlash = 92,
  RightBracket = 93,
  Accent = 96,
  // Non-ASCII
  Escape = 256,
  NumEnter,
  Backspace,
  Insert,
  Delete,
  Up,
  Down,
  Left,
  Right,
  PageUp,
  PageDown,
  Home,
  End,
  CapsLock,
  ScrollLock,
  NumLock,
  PrintScreen,
  Pause,
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  F25,
  NumPeriod,
  NumComma,
  NumPlus,
  NumHyphen,
  NumSlash,
  NumAsterisk,
  NumEquals,
  LeftShift,
  LeftControl,
  LeftAlt,
  LeftSuper,
  RightShift,
  RightControl,
  RightAlt,
  RightSuper,
  Menu,

  /// The "Compose" key on Linux.
  Compose,
  Caret,
  AbntC1,
  AbntC2,
  Asterisk,
  At,
  Ax,
  Calculator,
  Colon,
  Convert,
  Kana,
  Kanji,
  Mail,
  MediaSelect,
  MediaStop,
  Mute,
  MyComputer,
  // also called "Next"
  NavigateForward,
  // also called "Prior"
  NavigateBackward,
  NextTrack,
  NoConvert,
  OEM102,
  PlayPause,
  Plus,
  Power,
  PrevTrack,
  Sleep,
  Stop,
  SysRq,
  Underline,
  Unlabeled,
  VolumeDown,
  VolumeUp,
  Wake,
  WebBack,
  WebFavorites,
  WebForward,
  WebHome,
  WebRefresh,
  WebSearch,
  WebStop,
  Yen,
  Copy,
  Paste,
  Cut,
}

impl From<VirtualKeyCode> for KeyCode {
  fn from(value: VirtualKeyCode) -> Self {
    #[allow(unreachable_patterns)]
    match value {
      VirtualKeyCode::Key1 => KeyCode::_1,
      VirtualKeyCode::Key2 => KeyCode::_2,
      VirtualKeyCode::Key3 => KeyCode::_3,
      VirtualKeyCode::Key4 => KeyCode::_4,
      VirtualKeyCode::Key5 => KeyCode::_5,
      VirtualKeyCode::Key6 => KeyCode::_6,
      VirtualKeyCode::Key7 => KeyCode::_7,
      VirtualKeyCode::Key8 => KeyCode::_8,
      VirtualKeyCode::Key9 => KeyCode::_9,
      VirtualKeyCode::Key0 => KeyCode::_0,
      VirtualKeyCode::A => KeyCode::A,
      VirtualKeyCode::B => KeyCode::B,
      VirtualKeyCode::C => KeyCode::C,
      VirtualKeyCode::D => KeyCode::D,
      VirtualKeyCode::E => KeyCode::E,
      VirtualKeyCode::F => KeyCode::F,
      VirtualKeyCode::G => KeyCode::G,
      VirtualKeyCode::H => KeyCode::H,
      VirtualKeyCode::I => KeyCode::I,
      VirtualKeyCode::J => KeyCode::J,
      VirtualKeyCode::K => KeyCode::K,
      VirtualKeyCode::L => KeyCode::L,
      VirtualKeyCode::M => KeyCode::M,
      VirtualKeyCode::N => KeyCode::N,
      VirtualKeyCode::O => KeyCode::O,
      VirtualKeyCode::P => KeyCode::P,
      VirtualKeyCode::Q => KeyCode::Q,
      VirtualKeyCode::R => KeyCode::R,
      VirtualKeyCode::S => KeyCode::S,
      VirtualKeyCode::T => KeyCode::T,
      VirtualKeyCode::U => KeyCode::U,
      VirtualKeyCode::V => KeyCode::V,
      VirtualKeyCode::W => KeyCode::W,
      VirtualKeyCode::X => KeyCode::X,
      VirtualKeyCode::Y => KeyCode::Y,
      VirtualKeyCode::Z => KeyCode::Z,
      VirtualKeyCode::Escape => KeyCode::Escape,
      VirtualKeyCode::F1 => KeyCode::F1,
      VirtualKeyCode::F2 => KeyCode::F2,
      VirtualKeyCode::F3 => KeyCode::F3,
      VirtualKeyCode::F4 => KeyCode::F4,
      VirtualKeyCode::F5 => KeyCode::F5,
      VirtualKeyCode::F6 => KeyCode::F6,
      VirtualKeyCode::F7 => KeyCode::F7,
      VirtualKeyCode::F8 => KeyCode::F8,
      VirtualKeyCode::F9 => KeyCode::F9,
      VirtualKeyCode::F10 => KeyCode::F10,
      VirtualKeyCode::F11 => KeyCode::F11,
      VirtualKeyCode::F12 => KeyCode::F12,
      VirtualKeyCode::F13 => KeyCode::F13,
      VirtualKeyCode::F14 => KeyCode::F14,
      VirtualKeyCode::F15 => KeyCode::F15,
      VirtualKeyCode::F16 => KeyCode::F16,
      VirtualKeyCode::F17 => KeyCode::F17,
      VirtualKeyCode::F18 => KeyCode::F18,
      VirtualKeyCode::F19 => KeyCode::F19,
      VirtualKeyCode::F20 => KeyCode::F20,
      VirtualKeyCode::F21 => KeyCode::F21,
      VirtualKeyCode::F22 => KeyCode::F22,
      VirtualKeyCode::F23 => KeyCode::F23,
      VirtualKeyCode::F24 => KeyCode::F24,
      VirtualKeyCode::Snapshot => KeyCode::PrintScreen,
      VirtualKeyCode::Scroll => KeyCode::ScrollLock,
      VirtualKeyCode::Pause => KeyCode::Pause,
      VirtualKeyCode::Insert => KeyCode::Insert,
      VirtualKeyCode::Home => KeyCode::Home,
      VirtualKeyCode::Delete => KeyCode::Delete,
      VirtualKeyCode::End => KeyCode::End,
      VirtualKeyCode::PageDown => KeyCode::PageDown,
      VirtualKeyCode::PageUp => KeyCode::PageUp,
      VirtualKeyCode::Left => KeyCode::Left,
      VirtualKeyCode::Up => KeyCode::Up,
      VirtualKeyCode::Right => KeyCode::Right,
      VirtualKeyCode::Down => KeyCode::Down,
      VirtualKeyCode::Back => KeyCode::Backspace,
      VirtualKeyCode::Return => KeyCode::Enter,
      VirtualKeyCode::Space => KeyCode::Space,
      VirtualKeyCode::Compose => KeyCode::Compose,
      VirtualKeyCode::Caret => KeyCode::Caret,
      VirtualKeyCode::Numlock => KeyCode::NumLock,
      VirtualKeyCode::Numpad0 => KeyCode::Num0,
      VirtualKeyCode::Numpad1 => KeyCode::Num1,
      VirtualKeyCode::Numpad2 => KeyCode::Num2,
      VirtualKeyCode::Numpad3 => KeyCode::Num3,
      VirtualKeyCode::Numpad4 => KeyCode::Num4,
      VirtualKeyCode::Numpad5 => KeyCode::Num5,
      VirtualKeyCode::Numpad6 => KeyCode::Num6,
      VirtualKeyCode::Numpad7 => KeyCode::Num7,
      VirtualKeyCode::Numpad8 => KeyCode::Num8,
      VirtualKeyCode::Numpad9 => KeyCode::Num9,
      VirtualKeyCode::NumpadAdd => KeyCode::NumPlus,
      VirtualKeyCode::NumpadDivide => KeyCode::NumSlash,
      VirtualKeyCode::NumpadDecimal => KeyCode::NumPeriod,
      VirtualKeyCode::NumpadComma => KeyCode::NumComma,
      VirtualKeyCode::NumpadEnter => KeyCode::NumEnter,
      VirtualKeyCode::NumpadEquals => KeyCode::NumEquals,
      VirtualKeyCode::NumpadMultiply => KeyCode::NumAsterisk,
      VirtualKeyCode::NumpadSubtract => KeyCode::NumHyphen,
      VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
      VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
      VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
      VirtualKeyCode::Apps => KeyCode::Menu,
      VirtualKeyCode::Asterisk => KeyCode::Asterisk,
      VirtualKeyCode::At => KeyCode::At,
      VirtualKeyCode::Ax => KeyCode::Ax,
      VirtualKeyCode::Backslash => KeyCode::BackSlash,
      VirtualKeyCode::Calculator => KeyCode::Calculator,
      VirtualKeyCode::Capital => KeyCode::CapsLock,
      VirtualKeyCode::Colon => KeyCode::Colon,
      VirtualKeyCode::Comma => KeyCode::Comma,
      VirtualKeyCode::Convert => KeyCode::Convert,
      VirtualKeyCode::Equals => KeyCode::Equals,
      VirtualKeyCode::Grave => KeyCode::Accent,
      VirtualKeyCode::Kana => KeyCode::Kana,
      VirtualKeyCode::Kanji => KeyCode::Kanji,
      VirtualKeyCode::LAlt => KeyCode::LeftAlt,
      VirtualKeyCode::LBracket => KeyCode::LeftBracket,
      VirtualKeyCode::LControl => KeyCode::LeftControl,
      VirtualKeyCode::LShift => KeyCode::LeftShift,
      VirtualKeyCode::LWin => KeyCode::LeftSuper,
      VirtualKeyCode::Mail => KeyCode::Mail,
      VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
      VirtualKeyCode::MediaStop => KeyCode::MediaStop,
      VirtualKeyCode::Minus => KeyCode::Minus,
      VirtualKeyCode::Mute => KeyCode::Mute,
      VirtualKeyCode::MyComputer => KeyCode::MyComputer,
      VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
      VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
      VirtualKeyCode::NextTrack => KeyCode::NextTrack,
      VirtualKeyCode::NoConvert => KeyCode::NoConvert,
      VirtualKeyCode::OEM102 => KeyCode::OEM102,
      VirtualKeyCode::Period => KeyCode::Period,
      VirtualKeyCode::PlayPause => KeyCode::PlayPause,
      VirtualKeyCode::Plus => KeyCode::Plus,
      VirtualKeyCode::Power => KeyCode::Power,
      VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
      VirtualKeyCode::RAlt => KeyCode::RightAlt,
      VirtualKeyCode::RBracket => KeyCode::RightBracket,
      VirtualKeyCode::RControl => KeyCode::RightControl,
      VirtualKeyCode::RShift => KeyCode::RightShift,
      VirtualKeyCode::RWin => KeyCode::RightSuper,
      VirtualKeyCode::Semicolon => KeyCode::Semicolon,
      VirtualKeyCode::Slash => KeyCode::ForwardSlash,
      VirtualKeyCode::Sleep => KeyCode::Sleep,
      VirtualKeyCode::Stop => KeyCode::Stop,
      VirtualKeyCode::Sysrq => KeyCode::SysRq,
      VirtualKeyCode::Tab => KeyCode::Tab,
      VirtualKeyCode::Underline => KeyCode::Underline,
      VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
      VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
      VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
      VirtualKeyCode::Wake => KeyCode::Wake,
      VirtualKeyCode::WebBack => KeyCode::WebBack,
      VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
      VirtualKeyCode::WebForward => KeyCode::WebForward,
      VirtualKeyCode::WebHome => KeyCode::WebHome,
      VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
      VirtualKeyCode::WebSearch => KeyCode::WebSearch,
      VirtualKeyCode::WebStop => KeyCode::WebStop,
      VirtualKeyCode::Yen => KeyCode::Yen,
      VirtualKeyCode::Copy => KeyCode::Copy,
      VirtualKeyCode::Paste => KeyCode::Paste,
      VirtualKeyCode::Cut => KeyCode::Cut,
      _ => KeyCode::Unknown,
    }
  }
}