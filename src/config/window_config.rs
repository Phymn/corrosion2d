use winit::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct WindowConfig {
    pub resolution_x: i32,
    pub resolution_y: i32,
    pub window_title: String,
}

pub fn main_window(config: WindowConfig) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.window_title)
        .with_inner_size(LogicalSize::new(config.resolution_x, config.resolution_y))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
