use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[derive(Debug)]
pub struct C2WindowConfig {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) decorations: bool,
    pub(crate) window_title: String,
}

impl Default for C2WindowConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            decorations: false,
            window_title: String::from("Corrosion 2D"),
        }
    }
}

impl C2WindowConfig {
    pub(crate) fn window_builder(self) -> WindowBuilder {
        let builder = WindowBuilder::new();

        builder
            .with_decorations(self.decorations)
            .with_title(self.window_title)
            .with_inner_size(LogicalSize::new(self.width, self.height))
    }

    pub(crate) fn window_spawn(self, window_builder: WindowBuilder) {
        let event_loop = EventLoop::new();
        let window = match window_builder.build(&event_loop) {
            Ok(result) => result,
            Err(os_error) => {
                panic!("Could not build window! Err code: {}", os_error);
            }
        };
    }
}
