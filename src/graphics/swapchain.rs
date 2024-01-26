use anyhow::Result;

pub struct Swapchain {}

impl Swapchain {
  const MAX_FRAMES_IN_FLIGHT: u32 = 2;

  pub fn new() -> Result<Self> {
    Ok(Self {})
  }
}

impl Swapchain {
  unsafe fn free(&mut self) {}
}

impl Drop for Swapchain {
  fn drop(&mut self) {
    unsafe {
      self.free();
    }
  }
}