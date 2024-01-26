use tracing::info;
use tracing_unwrap::ResultExt;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW};
use windows::Win32::UI::WindowsAndMessaging::{CS_HREDRAW, CS_VREDRAW, DefWindowProcW, IDC_ARROW, LoadCursorW, PostQuitMessage, WM_PAINT, WM_DESTROY, WNDCLASSEXW, RegisterClassExW, CreateWindowExW, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, MSG, GetMessageW, DispatchMessageW, CS_OWNDC, ShowWindow, SW_SHOW, WM_CREATE};

pub struct Window {
  hwnd: HWND,
  msg: MSG,
}

impl Window {
  pub fn new() -> Self {
    info!("kon kon kitsune!");
    let h_instance = unsafe {
      GetModuleHandleW(None)
    }.unwrap_or_log();
    debug_assert!(h_instance.0 != 0);

    let window_class = windows::w!("koyote_window");
    let window = WNDCLASSEXW {
      cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
      lpfnWndProc: Some(wndproc),
      hInstance: h_instance,
      lpszClassName: window_class,

      style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
      hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }.unwrap_or_log(),
      ..Default::default()
    };

    let atom = unsafe { RegisterClassExW(&window) };
    debug_assert!(atom != 0);

    let hwnd = unsafe {
      CreateWindowExW(
        Default::default(),
        window_class,
        windows::w!("Koyote"),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        None,
        None,
        h_instance,
        None,
      )
    };

    let msg = MSG::default();

    Self {
      hwnd,
      msg,
    }
  }

  fn next(&mut self) -> BOOL {
    unsafe { GetMessageW(&mut self.msg, None, 0, 0) }
  }

  pub fn run(mut self) {
    unsafe { ShowWindow(self.hwnd, SW_SHOW) };
    while self.next().into() {
      unsafe { DispatchMessageW(&self.msg); }
    }
  }
}

impl Default for Window {
  fn default() -> Self {
    Self::new()
  }
}

extern "system" fn wndproc(
  window: HWND,
  message: u32,
  wparam: WPARAM,
  lparam: LPARAM,
) -> LRESULT {
  match message {
    WM_CREATE => {
      info!("WM_CREATE");
      LRESULT(0)
    }
    WM_PAINT => {
      info!("WM_PAINT");
      unsafe { ValidateRect(window, None) };
      LRESULT(0)
    }
    WM_DESTROY => {
      info!("WM_DESTROY");
      unsafe { PostQuitMessage(0) };
      LRESULT(0)
    }
    _ => unsafe {
      DefWindowProcW(window, message, wparam, lparam)
    }
  }
}