use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct Resolution {
    /// Game window resolution.
    pub width: i32,
    pub height: i32,
}

impl Resolution {
    /// Returns default resolution of 1920x1080
    pub fn default() -> Self {
        Resolution {
            width: 1920,
            height: 1080,
        }
    }
    /// Returns a new resolution
    pub fn new(width: i32, height: i32) -> Self {
        Resolution { width, height }
    }
}
pub struct C2DWindowConfig {
    /// Contains data needed to spawn a window with specified settings.
    pub resolution: Resolution,
    pub window_title: String,
    pub window_decorations: bool,
}

impl C2DWindowConfig {
    /// Returns default settings for the window.
    /// Resolution defaults to 1920, 1080.
    /// Window title defaults to Corrosion 2D.
    /// Window decorations is true.
    pub fn default() -> Self {
        C2DWindowConfig {
            resolution: Resolution::default(),
            window_title: String::from("Corrosion 2D"),
            window_decorations: true,
        }
    }

    pub fn new(resolution: Resolution, window_title: String, window_decorations: bool) -> Self {
        /// Returns a window config with specified values.
        C2DWindowConfig {
            resolution,
            window_title,
            window_decorations,
        }
    }
}
