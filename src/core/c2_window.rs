use winit::{dpi::LogicalSize, window::WindowBuilder};

pub struct C2Window {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) decorations: bool,
    pub(crate) window_title: String,
}

impl Default for C2Window {
    fn default() -> Self {
        Self {
            width: Some(1920),
            height: Some(1080),
            decorations: false,
            window_title: from::String("A Mourning Light"),
        }
    }
}

impl C2Window {
    //
    pub(crate) fn spawn_window(
        config: C2WindowConfig,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) {
        let window = WindowBuilder::new()
            .with_title(config.window_title)
            .with_inner_size(LogicalSize::new(config.resolution_x, config.resolution_y))
            .build(event_loop)
            .unwrap();
    }
}
