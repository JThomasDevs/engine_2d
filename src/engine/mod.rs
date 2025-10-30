pub mod config;
pub mod core;
#[cfg(feature = "opengl")]
pub mod window;

pub use config::{EngineConfig, ViewportConfig};
pub use core::Engine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_config_defaults() {
        let config = EngineConfig::default();
        assert_eq!(config.window_title, "Rust 2D Engine");
        assert_eq!(config.window_width, 800);
        assert_eq!(config.window_height, 600);
        assert_eq!(config.target_fps, Some(60));
        assert_eq!(config.show_fps, false);
        assert_eq!(config.vsync, true);
        assert_eq!(config.fullscreen, false);
        assert_eq!(config.viewport.logical_bounds, (-10.0, 10.0, -10.0, 10.0));
        assert_eq!(config.viewport.text_height_fraction, 0.02);
        assert_eq!(config.viewport.base_font_size, 16.0);
        assert_eq!(config.viewport.viewport_independent_text, true);
    }

    #[test]
    fn test_engine_config_custom() {
        let config = EngineConfig {
            window_title: "Test Game".to_string(),
            window_width: 1024,
            window_height: 768,
            target_fps: Some(120),
            show_fps: true,
            vsync: false,
            fullscreen: true,
            viewport: ViewportConfig::ndc(), // Use NDC coordinates
            fallback_font_path: "assets/fonts/default.ttf".to_string(),
        };

        assert_eq!(config.window_title, "Test Game");
        assert_eq!(config.window_width, 1024);
        assert_eq!(config.window_height, 768);
        assert_eq!(config.target_fps, Some(120));
        assert_eq!(config.show_fps, true);
        assert_eq!(config.vsync, false);
        assert_eq!(config.fullscreen, true);
        assert_eq!(config.viewport.logical_bounds, (-1.0, 1.0, -1.0, 1.0));
    }

    #[test]
    fn test_engine_config_clone() {
        let config1 = EngineConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.window_title, config2.window_title);
        assert_eq!(config1.window_width, config2.window_width);
        assert_eq!(config1.window_height, config2.window_height);
        assert_eq!(config1.target_fps, config2.target_fps);
        assert_eq!(config1.show_fps, config2.show_fps);
        assert_eq!(config1.vsync, config2.vsync);
        assert_eq!(config1.fullscreen, config2.fullscreen);
    }

    #[test]
    fn test_engine_config_debug() {
        let config = EngineConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("EngineConfig"));
        assert!(debug_str.contains("Rust 2D Engine"));
        assert!(debug_str.contains("800"));
        assert!(debug_str.contains("600"));
    }

    #[test]
    fn test_engine_config_fps_variations() {
        // Test with no FPS limit
        let config_no_fps = EngineConfig {
            target_fps: None,
            ..Default::default()
        };
        assert_eq!(config_no_fps.target_fps, None);

        // Test with custom FPS
        let config_custom_fps = EngineConfig {
            target_fps: Some(144),
            ..Default::default()
        };
        assert_eq!(config_custom_fps.target_fps, Some(144));
    }

    #[test]
    fn test_engine_config_window_sizes() {
        // Test common window sizes
        let sizes = vec![
            (640, 480),   // VGA
            (800, 600),   // SVGA
            (1024, 768),  // XGA
            (1280, 720),  // HD
            (1920, 1080), // Full HD
        ];

        for (width, height) in sizes {
            let config = EngineConfig {
                window_width: width,
                window_height: height,
                ..Default::default()
            };
            assert_eq!(config.window_width, width);
            assert_eq!(config.window_height, height);
        }
    }

    #[test]
    fn test_viewport_config_defaults() {
        let viewport = ViewportConfig::default();
        assert_eq!(viewport.logical_bounds, (-10.0, 10.0, -10.0, 10.0));
        assert_eq!(viewport.text_height_fraction, 0.02);
        assert_eq!(viewport.base_font_size, 16.0);
        assert_eq!(viewport.viewport_independent_text, true);
    }

    #[test]
    fn test_viewport_config_ndc() {
        let viewport = ViewportConfig::ndc();
        assert_eq!(viewport.logical_bounds, (-1.0, 1.0, -1.0, 1.0));
        assert_eq!(viewport.text_height_fraction, 0.05);
        assert_eq!(viewport.viewport_independent_text, true);
    }

    #[test]
    fn test_viewport_config_ui_based() {
        let viewport = ViewportConfig::ui_based();
        assert_eq!(viewport.logical_bounds, (0.0, 1.0, 0.0, 1.0));
        assert_eq!(viewport.text_height_fraction, 0.05);
        assert_eq!(viewport.viewport_independent_text, true);
    }

    #[test]
    fn test_viewport_config_pixel_based() {
        let viewport = ViewportConfig::pixel_based(1920.0, 1080.0);
        assert_eq!(viewport.logical_bounds, (0.0, 1920.0, 0.0, 1080.0));
        assert_eq!(viewport.viewport_independent_text, false);
    }

    #[test]
    fn test_viewport_config_with_bounds() {
        let viewport = ViewportConfig::with_bounds(-5.0, 5.0, -3.0, 3.0);
        assert_eq!(viewport.logical_bounds, (-5.0, 5.0, -3.0, 3.0));
        assert_eq!(viewport.text_height_fraction, 0.02);
        assert_eq!(viewport.viewport_independent_text, true);
    }
}
