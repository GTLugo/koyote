use std::sync::Arc;
use ash::{self, vk};
use anyhow::{Context, Result};
use crate::graphics::context::RenderContext;
use crate::graphics::image::Image;

pub struct Buffer {
  device: Arc<ash::Device>,
  pub buffer: vk::Buffer,
  pub memory: vk::DeviceMemory,
  pub size: vk::DeviceSize,
}

impl Buffer {
  pub fn new(
    context: &RenderContext,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
  ) -> Result<Self> {
    let buffer_create_info = vk::BufferCreateInfo {
      size,
      usage,
      sharing_mode: vk::SharingMode::EXCLUSIVE,
      ..Default::default()
    };

    let buffer = unsafe {
      context.device().create_buffer(&buffer_create_info, None)
    }.context("Failed to create buffer")?;

    let memory_reqs = unsafe {
      context.device().get_buffer_memory_requirements(buffer)
    };

    let memory_create_info = vk::MemoryAllocateInfo {
      allocation_size: memory_reqs.size,
      memory_type_index: context.find_memory_type(memory_reqs.memory_type_bits, properties).heap_index,
      ..Default::default()
    };

    let memory = match unsafe {
      context.device().allocate_memory(&memory_create_info, None)
    }.context("Failed to allocate buffer memory") {
      Ok(value) => value,
      Err(err) => unsafe {
        context.device().destroy_buffer(buffer, None);
        Err(err)?
      }
    };

    Ok(Self {
      device: context.device(),
      buffer,
      memory,
      size,
    })
  }

  unsafe fn free(&mut self) {
    self.device.destroy_buffer(self.buffer, None);
    self.device.free_memory(self.memory, None);
  }

  pub fn copy_to_buffer(&self, context: &RenderContext, dst: &Buffer) {
    context.issue_single_time_commands(|command_buffer| {
      let copy_region = vk::BufferCopy {
        size: self.size,
        ..Default::default()
      };

      unsafe {
        self.device.cmd_copy_buffer(command_buffer, self.buffer, dst.buffer, &[copy_region]);
      }
    });
  }

  pub fn copy_to_image(
    &self,
    context: &RenderContext,
    image: &Image,
  ) {
    context.issue_single_time_commands(|command_buffer| {
      let copy_region = vk::BufferImageCopy {
        image_subresource: vk::ImageSubresourceLayers {
          aspect_mask: vk::ImageAspectFlags::COLOR,
          layer_count: image.layer_count,
          ..Default::default()
        },
        image_extent: vk::Extent3D {
          width: image.extent.width,
          height: image.extent.height,
          depth: 1,
        },
        ..Default::default()
      };

      unsafe {
        self.device.cmd_copy_buffer_to_image(
          command_buffer,
          self.buffer,
          image.image,
          vk::ImageLayout::TRANSFER_DST_OPTIMAL,
          &[copy_region],
        );
      }
    });
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    unsafe {
      self.free();
    }
  }
}