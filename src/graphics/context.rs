use std::{ffi, sync::OnceLock};
use std::collections::HashSet;
use std::sync::Arc;

use anyhow::{Context, Error, Result};
use ash::{extensions::*, vk};
use raw_window_handle::HasRawDisplayHandle;
use tracing::{debug, error, trace};

use crate::graphics::debug::DebugMessenger;
use crate::graphics::shader::stage::Stage;
use crate::graphics::window::Window;

#[allow(unused)]
pub struct RenderContext {
  instance: ash::Instance,
  debug: Option<DebugMessenger>,
  physical_device: vk::PhysicalDevice,

  device: Arc<ash::Device>,
  command_pool: vk::CommandPool,
  graphics_queue: vk::Queue,
  present_queue: vk::Queue,

  pub physical_device_properties: vk::PhysicalDeviceProperties,
}

static ENABLE_VALIDATION_LAYERS: OnceLock<bool> = OnceLock::new();
static VALIDATION_LAYERS: OnceLock<HashSet<ffi::CString>> = OnceLock::new();
static INSTANCE_EXTENSIONS: OnceLock<HashSet<&ffi::CStr>> = OnceLock::new();
static DEVICE_EXTENSIONS: OnceLock<HashSet<&ffi::CStr>> = OnceLock::new();

impl RenderContext {
  pub fn new(window: &mut Window) -> Result<Self> {
    let entry = ash::Entry::linked();
    let instance = Self::create_instance(&entry, window)?;
    let debug = Self::create_debug_messenger(&entry, &instance);
    window.create_surface(&entry, &instance)?;
    let physical_device = Self::pick_physical_device(window, &instance)?;
    let device = Self::create_logical_device(window, &instance, physical_device)?;
    let command_pool = Self::create_command_pool(window, &instance, &device, physical_device)?;
    Stage::set_static_entry_points()?;

    Ok(Self {
      instance,
      debug,
      physical_device,
      device,
      command_pool,
      graphics_queue: Default::default(),
      present_queue: Default::default(),
      physical_device_properties: Default::default(),
    })
  }

  pub fn validation_layers_enabled() -> bool {
    *ENABLE_VALIDATION_LAYERS.get().unwrap()
  }

  pub fn command_pool(&self) -> &vk::CommandPool {
    &self.command_pool
  }

  pub fn device(&self) -> Arc<ash::Device> {
    self.device.clone()
  }

  pub fn graphics_queue(&self) -> &vk::Queue {
    &self.graphics_queue
  }

  pub fn present_queue(&self) -> &vk::Queue {
    &self.present_queue
  }

  pub fn find_memory_type(
    &self,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
  ) -> vk::MemoryType {
    let props = unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };

    for mem_type in props.memory_types {
      if (type_filter & (1 << mem_type.heap_index)) != 0 && mem_type.property_flags.contains(properties) {
        return mem_type;
      }
    }
    // for i in 0..props.memory_type_count as usize {
    //   if (type_filter & (1 << i)) != 0 && props.memory_types[i].property_flags.contains(properties) {
    //     return props.memory_types[i];
    //   }
    // }
    error!("Failed to find supported memory type.");
    vk::MemoryType::default()
  }

  pub fn begin_single_time_commands(&self) -> Result<vk::CommandBuffer> {
    let allocate_info = vk::CommandBufferAllocateInfo {
      level: vk::CommandBufferLevel::PRIMARY,
      command_pool: self.command_pool,
      command_buffer_count: 1,
      ..Default::default()
    };

    let command_buffer = unsafe {
      self.device.allocate_command_buffers(&allocate_info)
    }.context("Failed to allocate command buffers")?;

    let begin_info = vk::CommandBufferBeginInfo {
      flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
      ..Default::default()
    };

    unsafe {
      self.device.begin_command_buffer(command_buffer[0], &begin_info)
    }.context("Failed to begin command buffer")?;

    Ok(command_buffer[0])
  }

  pub fn end_single_time_commands(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
    unsafe {
      self.device.end_command_buffer(command_buffer)
    }.context("Failed to end command buffer")?;

    let submit_info = vk::SubmitInfo {
      command_buffer_count: 1,
      p_command_buffers: &command_buffer,
      ..Default::default()
    };

    unsafe {
      self.device.queue_submit(self.graphics_queue, &[submit_info], vk::Fence::null())
    }.context("Failed to submit graphics queue")?;

    unsafe {
      self.device.queue_wait_idle(self.graphics_queue)
    }.context("Failed to process graphics queue")?;

    unsafe {
      self.device.free_command_buffers(self.command_pool, &[command_buffer])
    };

    Ok(())
  }

  pub fn issue_single_time_commands<F: FnOnce(vk::CommandBuffer)>(&self, commands: F) {
    match self.begin_single_time_commands() {
      Ok(command_buffer) => {
        commands(command_buffer);
        match self.end_single_time_commands(command_buffer) {
          Ok(_) => {}
          Err(e) => error!("{e:#}")
        };
      }
      Err(e) => error!("{e:#}")
    }
  }
}

impl Drop for RenderContext {
  fn drop(&mut self) {
    trace!("Dropping Context...");
    unsafe { self.free() };
    trace!("Context freed!");
  }
}

impl RenderContext {
  // PRIVATE
  unsafe fn free(&mut self) {
    self.device.destroy_command_pool(self.command_pool, None);
    self.device.destroy_device(None);
    if let Some(db) = self.debug.as_mut() {
      db.free()
    };
    self.instance.destroy_instance(None);
  }

  fn create_instance(entry: &ash::Entry, window: &Window) -> Result<ash::Instance> {
    let app_info = vk::ApplicationInfo {
      p_engine_name: ffi::CString::new("Koyote")?.as_ptr(),
      engine_version: vk::make_api_version(1, 0, 1, 0),
      p_application_name: ffi::CString::new(window.title())?.as_ptr(),
      application_version: vk::make_api_version(1, 0, 0, 0),
      api_version: vk::API_VERSION_1_3,
      ..Default::default()
    };

    Self::check_layers(entry)?;
    Self::check_extensions(window)?;

    let enabled_layers = Self::enabled_layers();
    let enabled_instance_extensions = Self::enabled_instance_extensions_c_chars();

    let create_info = vk::InstanceCreateInfo {
      enabled_extension_count: enabled_instance_extensions.len() as u32,
      pp_enabled_extension_names: enabled_instance_extensions.as_ptr(),
      enabled_layer_count: enabled_layers.len() as u32,
      pp_enabled_layer_names: enabled_layers.as_ptr(),
      p_application_info: &app_info,
      ..Default::default()
    };

    unsafe {
      entry.create_instance(&create_info, None).context("Failed to create Vulkan instance")
    }
  }

  fn check_layers(entry: &ash::Entry) -> Result<()> {
    match VALIDATION_LAYERS.set({
      let mut v: HashSet<ffi::CString> = Default::default();
      let mut layers_enabled = cfg!(debug_assertions);

      if layers_enabled {
        v.insert(ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap());
      };

      let layer_props = entry.enumerate_instance_layer_properties()
        .context("Failed to enumerate instance layer properties")?;

      for l in &v {
        let mut found = false;
        for layer in &layer_props {
          if l.as_c_str() == unsafe { ffi::CStr::from_ptr(layer.layer_name.as_ptr()) } {
            found = true;
          }
        }

        if !found {
          error!("One or more validation layers requested were unavailable!");
          layers_enabled = false;
        }
      }

      if !layers_enabled {
        v.clear();
      }

      match ENABLE_VALIDATION_LAYERS.set(layers_enabled) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::msg("Failed to set validation layer boolean"))
      }?;

      v
    }) {
      Ok(_) => Ok(()),
      Err(_) => Err(anyhow::anyhow!("Failed to set validation layer HashSet"))
    }
  }

  fn enabled_layers() -> Vec<*const ffi::c_char> {
    VALIDATION_LAYERS.get().unwrap()
      .iter()
      .map(|l| l.as_ptr())
      .collect()
  }

  fn check_extensions(window: &Window) -> Result<()> {
    if INSTANCE_EXTENSIONS.set({
      let mut v: HashSet<&ffi::CStr> = Default::default();
      if Self::validation_layers_enabled() {
        v.insert(ext::DebugUtils::name());
      }

      for ext_name in ash_window::enumerate_required_extensions(
        window.winit().raw_display_handle()
      )? {
        v.insert(unsafe { ffi::CStr::from_ptr(*ext_name) });
      }

      v
    }).is_err() {
      anyhow::bail!("Failed to set instance extensions HashSet");
    }

    if DEVICE_EXTENSIONS.set({
      let mut v: HashSet<&ffi::CStr> = Default::default();

      v.extend([
        khr::Swapchain::name(),

        // ffi::CString::new("VK_KHR_driver_properties").unwrap().as_c_str()
      ]);

      v
    }).is_err() {
      anyhow::bail!("Failed to set instance extensions HashSet");
    }

    Ok(())
  }

  fn enabled_instance_extensions() -> &'static HashSet<&'static ffi::CStr> {
    INSTANCE_EXTENSIONS.get()
      .expect("Failed to get INSTANCE_EXTENSIONS variable")
  }

  fn enabled_instance_extensions_c_chars() -> Vec<*const ffi::c_char> {
    Self::enabled_instance_extensions()
      .iter()
      .map(|e| e.as_ptr())
      .collect()
  }

  fn enabled_device_extensions() -> &'static HashSet<&'static ffi::CStr> {
    DEVICE_EXTENSIONS.get()
      .expect("Failed to get DEVICE_EXTENSIONS variable")
  }

  fn enabled_device_extensions_c_chars() -> Vec<*const ffi::c_char> {
    Self::enabled_device_extensions()
      .iter()
      .map(|e| e.as_ptr())
      .collect()
  }

  fn create_debug_messenger(entry: &ash::Entry, instance: &ash::Instance) -> Option<DebugMessenger> {
    if Self::validation_layers_enabled() {
      Some(match DebugMessenger::new(entry, instance) {
        Ok(value) => value,
        Err(e) => {
          error!("{e}");
          return None;
        }
      })
    } else {
      None
    }
  }

  fn pick_physical_device(window: &Window, instance: &ash::Instance) -> Result<vk::PhysicalDevice> {
    let physical_devices = unsafe {
      instance.enumerate_physical_devices()
    }.context("Failed to enumerate physical devices")?;
    debug!("Physical device count: {}", physical_devices.len());

    let physical_device = physical_devices
      .iter()
      .filter(|p| Self::device_suitable(window, instance, **p))
      .min_by_key(|p| unsafe {
        // lower score for preferred device types
        match instance.get_physical_device_properties(**p).device_type {
          vk::PhysicalDeviceType::DISCRETE_GPU => 0,
          vk::PhysicalDeviceType::INTEGRATED_GPU => 1,
          vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
          vk::PhysicalDeviceType::CPU => 3,
          vk::PhysicalDeviceType::OTHER => 4,
          _ => 5,
        }
      }).context("Failed to find valid physical device")?;

    let props = unsafe {
      instance.get_physical_device_properties(*physical_device)
    };
    let device_name = unsafe {
      ffi::CStr::from_ptr(props.device_name.as_ptr()).to_str().unwrap()
    };
    debug!("Chosen device: [{}]", device_name);

    Ok(*physical_device)
  }

  fn create_logical_device(
    window: &Window,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
  ) -> Result<Arc<ash::Device>> {
    let indices = Self::find_queue_families(window, instance, physical_device)?;
    let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = vec![];
    let unique_queue_families: HashSet<u32> = HashSet::from([
      indices.graphics_family,
      indices.present_family,
    ]);

    let queue_priority = 1.0;
    for queue_family in unique_queue_families {
      let queue_create_info = vk::DeviceQueueCreateInfo {
        queue_family_index: queue_family,
        queue_count: 1,
        p_queue_priorities: &queue_priority,
        ..Default::default()
      };
      queue_create_infos.push(queue_create_info);
    }

    let device_features = vk::PhysicalDeviceFeatures {
      sampler_anisotropy: vk::TRUE,
      ..Default::default()
    };

    let enabled_device_extensions = Self::enabled_device_extensions_c_chars();

    let create_info = vk::DeviceCreateInfo {
      queue_create_info_count: queue_create_infos.len() as u32,
      p_queue_create_infos: queue_create_infos.as_ptr(),
      p_enabled_features: &device_features,
      enabled_extension_count: enabled_device_extensions.len() as u32,
      pp_enabled_extension_names: enabled_device_extensions.as_ptr(),
      ..Default::default()
    };

    let device = unsafe {
      instance.create_device(physical_device, &create_info, None)
    }.context("Failed to create logical graphics device")?;

    Ok(Arc::new(device))
  }

  fn create_command_pool(
    window: &Window,
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
  ) -> Result<vk::CommandPool> {
    let indices = Self::find_queue_families(window, instance, physical_device)?;

    let create_info = vk::CommandPoolCreateInfo {
      queue_family_index: indices.graphics_family,
      flags: vk::CommandPoolCreateFlags::TRANSIENT | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
      ..Default::default()
    };

    unsafe {
      device.create_command_pool(&create_info, None)
    }.context("Failed to create command pool")
  }

  fn device_extensions_supported(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Result<bool> {
    let available_extensions = unsafe {
      instance.enumerate_device_extension_properties(physical_device)
    }.context("Failed to enumerate device extension properties")?;

    let mut requested_extensions: HashSet<ffi::CString> = Default::default();
    for str in Self::enabled_device_extensions() {
      requested_extensions.insert(ffi::CString::from(*str));
    }
    for ext in available_extensions {
      requested_extensions.remove(unsafe { ffi::CStr::from_ptr(ext.extension_name.as_ptr()) });
    }

    Ok(requested_extensions.is_empty())
  }

  fn device_suitable(
    window: &Window,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
  ) -> bool {
    let indices = Self::find_queue_families(window, instance, physical_device);
    let props = unsafe {
      instance.get_physical_device_properties(physical_device)
    };
    let device_name = unsafe {
      String::from(ffi::CStr::from_ptr(props.device_name.as_ptr()).to_str().unwrap())
    };

    debug!("Checking if suitable: [{}]", device_name);
    // debug!("Checking if suitable: [{}]", unsafe { std::str::from_utf8_unchecked(std::mem::transmute(&props.device_name as &[i8])) });

    let extensions_supported = match Self::device_extensions_supported(instance, physical_device) {
      Ok(value) => value,
      Err(e) => {
        error!("{e}");
        false
      }
    };

    let swapchain_adequate = if extensions_supported {
      let swapchain_support = match window.swapchain_support(physical_device) {
        Ok(value) => value,
        Err(e) => {
          error!("{e}");
          return false;
        }
      };
      !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty()
    } else {
      false
    };

    let supported_features = unsafe { instance.get_physical_device_features(physical_device) };

    indices.is_ok() && extensions_supported && swapchain_adequate && supported_features.sampler_anisotropy == vk::TRUE
  }

  fn find_queue_families(
    window: &Window,
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
  ) -> Result<QueueFamilyIndices> {
    let queue_families = unsafe {
      instance.get_physical_device_queue_family_properties(physical_device)
    };

    let mut graphics_family = None;
    let mut present_family = None;
    for (i, family) in queue_families.iter().enumerate() {
      if family.queue_count > 0 && family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
        graphics_family = Some(i as u32);
      }

      let present_support = unsafe {
        window.surface_loader().get_physical_device_surface_support(
          physical_device,
          i as u32,
          *window.surface(),
        )
      }?;

      if family.queue_count > 0 && present_support {
        present_family = Some(i as u32);
      }

      if let (Some(graphics_family), Some(present_family)) = (graphics_family, present_family) {
        return Ok(QueueFamilyIndices {
          graphics_family,
          present_family,
        });
      }
    }

    Err(Error::msg("Failed to find suitable queue families"))
  }

  #[allow(unused)]
  fn find_supported_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    candidates: Arc<[vk::Format]>,
    tiling: vk::ImageTiling,
    features: vk::FormatFeatureFlags,
  ) -> vk::Format {
    for format in candidates.iter() {
      let props = unsafe { instance.get_physical_device_format_properties(physical_device, *format) };

      if (tiling == vk::ImageTiling::LINEAR && props.linear_tiling_features.contains(features))
        || (tiling == vk::ImageTiling::OPTIMAL && props.optimal_tiling_features.contains(features)) {
        return *format;
      }
    }
    error!("Failed to find supported format.");
    vk::Format::B8G8R8_UNORM
  }
}

// #[derive(Debug)]
// pub struct DeviceInfo {
//   pub name: String,
//   pub driver_version: String,
// }
//
// impl Display for DeviceInfo {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     write!("")
//   }
// }

#[derive(Default)]
struct QueueFamilyIndices {
  pub graphics_family: u32,
  pub present_family: u32,
}

impl QueueFamilyIndices {
  // pub fn complete(&self) -> bool { self.graphics_family.is_some() && self.present_family.is_some() }
}
