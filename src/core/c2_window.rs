use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;

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

    pub fn redraw(self) {
        self::WindowBuilder::reque
    }
}
