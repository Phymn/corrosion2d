use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

struct Resolution {
    x: i32,
    y: i32,
}

pub struct C2DWindowBuilder {
    resolution: Resolution,
    window_title: String,
    window_decorations: bool,
}
