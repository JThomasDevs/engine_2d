pub mod animation;
pub mod ecs;
pub mod engine;
pub mod events;
pub mod input;
pub mod render;
pub mod utils;

#[cfg(test)]
mod tests {

    #[test]
    fn test_library_integration() {
        // Test that modules can work together functionally
        use crate::animation::{Animation, NoAnimation};
        use crate::engine::EngineConfig;

        // Test that we can create an engine configuration
        let config = EngineConfig {
            window_title: "Test Game".to_string(),
            window_width: 1024,
            window_height: 768,
            target_fps: Some(60),
            show_fps: true,
            vsync: true,
            fullscreen: false,
            viewport: crate::engine::ViewportConfig::default(),
            fallback_font_path: "assets/fonts/default.ttf".to_string(),
        };

        // Test that we can create an animation
        let animation = NoAnimation::new();

        // Test that both work together (basic integration test)
        assert_eq!(config.window_title, "Test Game");
        assert_eq!(animation.name(), "No Animation");
    }
}
