use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;

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
            window_title: String::from("A Mourning Light"),
        }
    }
}

impl C2WindowConfig {
    pub(crate) fn into(self) -> WindowBuilder {
        let mut window_builder = WindowBuilder::new();

        window_builder
            .with_decorations(self.decorations)
            .with_title(self.window_title)
            .with_inner_size(LogicalSize::new(self.height, self.width))
    }
}
