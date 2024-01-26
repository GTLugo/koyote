use std::ffi;

use ash::vk;
use strum::{Display, EnumIter};

#[derive(EnumIter, Display, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Stage {
  Vertex,
  Fragment,
  Compute,
  Geometry,
}

impl Stage {
  pub fn into_entry_point_string(self) -> String {
    self.to_string() + "_main"
  }

  pub fn into_entry_point(self) -> ffi::CString {
    ffi::CString::new(self.into_entry_point_string())
      .expect("invalid entry point strings set in library. must abort.")
  }
}

impl From<Stage> for shaderc::ShaderKind {
  fn from(value: Stage) -> Self {
    match value {
      Stage::Vertex => shaderc::ShaderKind::Vertex,
      Stage::Fragment => shaderc::ShaderKind::Fragment,
      Stage::Compute => shaderc::ShaderKind::Compute,
      Stage::Geometry => shaderc::ShaderKind::Geometry,
    }
  }
}

impl From<Stage> for vk::ShaderStageFlags {
  fn from(value: Stage) -> Self {
    match value {
      Stage::Vertex => vk::ShaderStageFlags::VERTEX,
      Stage::Fragment => vk::ShaderStageFlags::FRAGMENT,
      Stage::Compute => vk::ShaderStageFlags::COMPUTE,
      Stage::Geometry => vk::ShaderStageFlags::GEOMETRY,
    }
  }
}
