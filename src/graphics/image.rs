use std::sync::Arc;
use ash::{self, vk};
use anyhow::{Context, Result};
use crate::graphics::context::RenderContext;

pub struct Image {
  device: Arc<ash::Device>,
  pub image: vk::Image,
  pub memory: vk::DeviceMemory,
  pub extent: vk::Extent3D,
  pub layer_count: u32,
}

impl Image {
  pub fn new(
    context: &RenderContext,
    image_info: vk::ImageCreateInfo,
    properties: vk::MemoryPropertyFlags,
  ) -> Result<Self> {
    let image = unsafe {
      context.device().create_image(&image_info, None)
    }.context("Failed to create image")?;

    let memory_reqs = unsafe {
      context.device().get_image_memory_requirements(image)
    };

    let allocation_info = vk::MemoryAllocateInfo {
      memory_type_index: context.find_memory_type(memory_reqs.memory_type_bits, properties).heap_index,
      ..Default::default()
    };

    let memory = match unsafe {
      context.device().allocate_memory(&allocation_info, None)
    }.context("Failed to allocate memory for image") {
      Ok(value) => value,
      Err(err) => unsafe {
        context.device().destroy_image(image, None);
        Err(err)?
      }
    };

    if let Err(err) = unsafe {
      context.device().bind_image_memory(image, memory, 0)
    }.context("Failed to bind image memory") {
      unsafe {
        context.device().destroy_image(image, None);
        context.device().free_memory(memory, None);
      }
      Err(err)?
    };

    Ok(Self {
      device: context.device(),
      image,
      memory,
      extent: image_info.extent,
      layer_count: image_info.array_layers,
    })
  }

  unsafe fn free(&mut self) {
    self.device.destroy_image(self.image, None);
    self.device.free_memory(self.memory, None);
  }
}

impl Drop for Image {
  fn drop(&mut self) {
    unsafe {
      self.free();
    }
  }
}