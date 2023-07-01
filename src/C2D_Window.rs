use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[derive(Default)]
pub struct Resolution {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct C2DWindowBuilder {
    pub resolution: Resolution,
    pub window_title: String,
    pub window_decorations: bool,
}

impl Resolution {
    pub fn new() -> Self {
        Default::default()
    }
}

impl C2DWindowBuilder {
    pub fn new() -> Self {
        Default::default()
    }
}
