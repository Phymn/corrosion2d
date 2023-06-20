use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;

use crate::core::c2_event_handler;

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
    pub(crate) fn window_builder(
        self,
        event_handler: c2_event_handler::C2EventHandler,
    ) -> WindowBuilder {
        let mut builder = WindowBuilder::new();

        builder
            .with_decoration(self.decorations)
            .with_title(self.window_title)
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .build(&event_handler)
    }
}
