use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[dervive(Default)]
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

impl Default for Resolution {
    fn default() -> Self {
        Resolution { x: 1920, y: 1080 }
    }
}

impl Default for C2DWindowBuilder {
    fn default() -> Self {
        C2DWindowBuilder {
            resolution: Resolution::default(),
            window_title: String::from("Corrosion 2D"),
            window_decorations: true,
        }
    }
}
