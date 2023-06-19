use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;

use crate::core::c2_event_handler;

pub struct C2WindowConfig {
    pub width: i32,
    pub height: i32,
    pub decorations: bool,
    pub window_title: String,
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
    }
}
