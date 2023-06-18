use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;

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
            window_title: String::from("Corrision 2D"),
        }
    }
}

impl C2WindowConfig {
    pub fn spawn_window(self) -> WindowBuilder {
        let window_builder = WindowBuilder::new();

        window_builder
            .with_decorations(self.decorations)
            .with_title(self.window_title)
            .with_inner_size(LogicalSize::new(self.height, self.width))
    }
}
