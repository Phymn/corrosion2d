use winit::{dpi::LogicalSize, window::WindowBuilder};

pub struct WindowConfig {
    pub resolution_x: i32,
    pub resolution_y: i32,
    pub window_title: String,
}

pub fn main_window(config: WindowConfig, event_loop: &winit::event_loop::EventLoop<()>) {
    let window = WindowBuilder::new()
        .with_title(config.window_title)
        .with_inner_size(LogicalSize::new(config.resolution_x, config.resolution_y))
        .build(event_loop)
        .unwrap();
}
