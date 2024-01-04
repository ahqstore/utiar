#![windows_subsystem = "windows"]
mod window;
use window::*;

use serde_json::{Number, Value};
use windows::Win32::{
  Foundation::HWND,
  System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
  UI::WindowsAndMessaging,
};

fn main() {
  unsafe {
    CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();
  }
  window::set_process_dpi_awareness().unwrap();
}
