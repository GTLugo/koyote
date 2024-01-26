use anyhow::{Context, Result};
use tracing::{trace};
use winit::{
  event_loop::EventLoop,
  dpi::{
    LogicalSize,
    PhysicalPosition,
  },
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use ash::{self, vk, extensions::*};

pub struct Window {
  window: winit::window::Window,
  surface: Option<Box<vk::SurfaceKHR>>,
  surface_loader: Option<khr::Surface>,
}

impl Window {
  pub fn new(event_loop: &EventLoop<()>, title: &'static str, width: u32, height: u32) -> Result<Self> {
    let window = winit::window::WindowBuilder::new()
      .with_title(title)
      .with_inner_size(LogicalSize::new(width, height))
      .with_visible(false)
      .build(event_loop)
      .context("Failed to create window")?;

    Ok(Self {
      window,
      surface: None,
      surface_loader: None,
    })
  }

  unsafe fn free(&mut self) {
    if let Some(surface_loader) = self.surface_loader.as_ref() {
      surface_loader.destroy_surface(**self.surface.as_ref().unwrap(), None);
    }
  }

  pub(crate) fn create_surface(&mut self, entry: &ash::Entry, instance: &ash::Instance) -> Result<()> {
    self.surface = Some(Box::new(unsafe {
      ash_window::create_surface(
        entry,
        instance,
        self.window.raw_display_handle(),
        self.window.raw_window_handle(),
        None,
      )
    }.context("Failed to create window surface")?));

    self.surface_loader = Some(khr::Surface::new(entry, instance));

    Ok(())
  }

  // pub(crate) fn take_event_loop(&mut self) -> Result<EventLoop<()>> {
  //   self.event_loop.take().context("Event loop already taken!")
  // }

  pub(crate) fn winit(&self) -> &winit::window::Window {
    &self.window
  }

  #[allow(unused)]
  pub(crate) fn surface(&self) -> &vk::SurfaceKHR {
    unsafe {
      self.surface.as_ref().unwrap_unchecked()
    }
  }

  #[allow(unused)]
  pub(crate) fn surface_loader(&self) -> &khr::Surface {
    unsafe {
      self.surface_loader.as_ref().unwrap_unchecked()
    }
  }

  pub(crate) fn swapchain_support(&self, physical_device: vk::PhysicalDevice) -> Result<SwapchainSupport> {
    Ok(SwapchainSupport {
      capabilities: unsafe {
        self.surface_loader().get_physical_device_surface_capabilities(physical_device, *self.surface())
      }.context("Failed to get physical device surface capabilities")?,
      formats: unsafe {
        self.surface_loader().get_physical_device_surface_formats(physical_device, *self.surface())
      }.context("Failed to get physical device surface formats")?,
      present_modes: unsafe {
        self.surface_loader().get_physical_device_surface_present_modes(physical_device, *self.surface())
      }.context("Failed to get physical device surface present modes")?,
    })
  }

  // #[allow(unused)]
  // pub(crate) fn size_physical(&self) -> PhysicalSize<u32> {
  //   self.window.inner_size()
  // }

  pub fn title(&self) -> String {
    self.window.title()
  }

  pub fn size(&self) -> (u32, u32) {
    let x = self.window.inner_size();
    (x.width, x.height)
  }

  pub fn set_visible(&self, visible: bool) {
    self.window.set_visible(visible);
  }

  pub fn center_on_monitor(&self) {
    let monitor = self.window.current_monitor().unwrap();
    let monitor_center = PhysicalPosition::new(
      monitor.position().x + (monitor.size().width as f32 * 0.5).floor() as i32,
      monitor.position().y + (monitor.size().height as f32 * 0.5).floor() as i32,
    );
    let window_offset = PhysicalPosition::new(
      monitor_center.x - (self.window.outer_size().width as f32 * 0.5).floor() as i32,
      monitor_center.y - (self.window.outer_size().height as f32 * 0.5).floor() as i32,
    );
    self.window.set_outer_position(window_offset);
  }

  pub(crate) fn request_redraw(&self) {
    self.window.request_redraw();
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    trace!("Dropping Window...");
    unsafe {
      self.free()
    };
    trace!("Window freed!");
  }
}

#[derive(Default)]
pub struct SwapchainSupport {
  pub capabilities: vk::SurfaceCapabilitiesKHR,
  pub formats: Vec<vk::SurfaceFormatKHR>,
  pub present_modes: Vec<vk::PresentModeKHR>,
}