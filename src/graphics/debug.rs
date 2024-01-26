use std::ffi;
use anyhow::{Context, Result};
use ash::{self, vk, extensions::*};
use tracing::{error, info, trace, warn};

pub struct DebugMessenger {
  debug_utils: ext::DebugUtils,
  debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugMessenger {
  pub fn new(entry: &ash::Entry, instance: &ash::Instance) -> Result<Self> {
    let debug_utils = ext::DebugUtils::new(entry, instance);
    let messenger_info = vk::DebugUtilsMessengerCreateInfoEXT {
      message_severity:
      vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO |
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
      message_type:
      // vk::DebugUtilsMessageTypeFlagsEXT::GENERAL |
      vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION |
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
      pfn_user_callback: Some(vulkan_debug_utils_callback),
      ..Default::default()
    };

    let debug_messenger = unsafe {
      debug_utils.create_debug_utils_messenger(&messenger_info, None)
    }.context("Failed to create debug messenger")?;

    Ok(Self {
      debug_utils,
      debug_messenger,
    })
  }

  pub fn free(&mut self) {
    unsafe { self.debug_utils.destroy_debug_utils_messenger(self.debug_messenger, None) };
  }
}

#[allow(unused)]
unsafe extern "system" fn vulkan_debug_utils_callback(
  message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
  message_type: vk::DebugUtilsMessageTypeFlagsEXT,
  p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
  _p_user_data: *mut ffi::c_void,
) -> vk::Bool32 {
  let message = String::from_utf8_lossy(ffi::CStr::from_ptr((*p_callback_data).p_message).to_bytes());
  let severity = format!("{:?}", message_severity).to_lowercase();
  let type_ = format!("{:?}", message_type).to_uppercase();

  match severity.as_str() {
    "verbose" => trace!("VULKAN | {type_} | {message}"),
    "info" => info!("VULKAN | {type_} | {message}"),
    "warning" => warn!("VULKAN | {type_} | {message}"),
    "error" => error!("VULKAN | {type_} | {message}"),
    _ => unreachable!()
  };

  vk::FALSE
}