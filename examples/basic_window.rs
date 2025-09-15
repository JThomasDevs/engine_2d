use engine_2d::engine::{Engine, EngineConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a basic window using the engine...");
    
    // Create a custom configuration for this example
    let config = EngineConfig {
        window_title: "Rust 2D Engine - Basic Window Example".to_string(),
        window_width: 800,
        window_height: 600,
        target_fps: Some(60),
        show_fps: true, // Show FPS for this example
        vsync: true,
        fullscreen: false,
    };
    
    // Create and run the engine
    let mut engine = Engine::new_with_config(config)?;
    
    println!("Engine created with window: {} ({}x{})", 
             engine.get_window_manager().get_title(),
             engine.get_window_manager().get_size().0,
             engine.get_window_manager().get_size().1);
    
    // Run the engine (this will handle the event loop and window management)
    engine.run()?;
    Ok(())
}
