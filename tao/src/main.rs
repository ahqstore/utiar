use tao::{event_loop::EventLoop, window::Window};

use tao::platform::windows::WindowExtWindows;

fn main() {
  let event_loop = EventLoop::new();

  let window = Window::new(&event_loop).unwrap();

  println!("Handle: {}", window.hwnd());

  event_loop.run(move |_, _, _| {});
}
