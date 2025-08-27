#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub target_fps: Option<u32>,
    pub show_fps: bool,
    pub vsync: bool,
    pub fullscreen: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "Rust 2D Engine".to_string(),
            window_width: 800,
            window_height: 600,
            target_fps: Some(60),
            show_fps: false,
            vsync: true,
            fullscreen: false,
        }
    }
}
