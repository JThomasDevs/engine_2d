use engine_2d::engine::config::EngineConfig;

#[test]
fn test_engine_config_defaults() {
    let config = EngineConfig::default();
    
    assert_eq!(config.window_title, "Rust 2D Engine");
    assert_eq!(config.window_width, 800);
    assert_eq!(config.window_height, 600);
    assert_eq!(config.target_fps, Some(60));
    assert!(!config.show_fps);
    assert!(config.vsync);
    assert!(!config.fullscreen);
}

#[test]
fn test_engine_config_custom_values() {
    let config = EngineConfig {
        window_title: "My Game".to_string(),
        window_width: 1920,
        window_height: 1080,
        target_fps: Some(120),
        show_fps: true,
        vsync: false,
        fullscreen: true,
        viewport: engine_2d::engine::config::ViewportConfig::default(),
        fallback_font_path: "assets/fonts/default.ttf".to_string(),
    };
    
    assert_eq!(config.window_title, "My Game");
    assert_eq!(config.window_width, 1920);
    assert_eq!(config.window_height, 1080);
    assert_eq!(config.target_fps, Some(120));
    assert!(config.show_fps);
    assert!(!config.vsync);
    assert!(config.fullscreen);
}

#[test]
fn test_engine_config_clone() {
    let config = EngineConfig::default();
    let cloned_config = config.clone();
    
    assert_eq!(config.window_title, cloned_config.window_title);
    assert_eq!(config.window_width, cloned_config.window_width);
    assert_eq!(config.window_height, cloned_config.window_height);
    assert_eq!(config.target_fps, cloned_config.target_fps);
    assert_eq!(config.show_fps, cloned_config.show_fps);
    assert_eq!(config.vsync, cloned_config.vsync);
    assert_eq!(config.fullscreen, cloned_config.fullscreen);
}

#[test]
fn test_engine_config_debug() {
    let config = EngineConfig::default();
    let debug_str = format!("{:?}", config);
    
    // Should contain the window title
    assert!(debug_str.contains("Rust 2D Engine"));
    // Should contain the window dimensions
    assert!(debug_str.contains("800"));
    assert!(debug_str.contains("600"));
}

#[test]
fn test_fps_configuration() {
    let config = EngineConfig {
        show_fps: true,
        target_fps: Some(60),
        ..Default::default()
    };
    
    assert!(config.show_fps);
    assert_eq!(config.target_fps, Some(60));
}

#[test]
fn test_window_config_resizable() {
    // Test that window configuration can be customized with specific combinations
    let config = EngineConfig {
        window_title: "Resizable Window".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        vsync: true,
        ..Default::default()
    };
    
    assert_eq!(config.window_title, "Resizable Window");
    assert_eq!(config.window_width, 1920);
    assert_eq!(config.window_height, 1080);
    assert!(!config.fullscreen);
    assert!(config.vsync);
}
