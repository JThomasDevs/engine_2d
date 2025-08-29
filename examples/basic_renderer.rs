use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::render::renderer::Renderer;

use glam::Vec2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Basic Renderer Example");
    println!("Demonstrating the safe OpenGL wrapper and renderer capabilities");
    
    // Create a custom configuration
    let config = EngineConfig {
        window_title: "Rust 2D Engine - Renderer Example".to_string(),
        window_width: 800,
        window_height: 600,
        target_fps: Some(60),
        show_fps: true,
        vsync: true,
        fullscreen: false,
    };
    
    // Create engine
    let mut engine = Engine::new_with_config(config)?;
    
    println!("Engine created successfully!");
    println!("Window: {} ({}x{})", 
             engine.get_window_manager().get_title(),
             engine.get_window_manager().get_size().0,
             engine.get_window_manager().get_size().1);
    
    // Demonstrate renderer creation and safe wrapper usage
    println!("\n=== Renderer Demonstration ===");
    
    // Create a renderer (safe, no OpenGL context needed yet)
    let mut renderer = Renderer::new();
    println!("✅ Renderer created successfully");
    
    // Try to initialize renderer (will fail gracefully without OpenGL context)
    println!("Attempting to initialize renderer...");
    match renderer.initialize() {
        Ok(()) => println!("✅ Renderer initialized with OpenGL context"),
        Err(e) => println!("⚠️  Renderer initialization failed (expected): {}", e),
    }
    
    // Try to clear screen (will fail gracefully)
    println!("Attempting to clear screen...");
    match renderer.clear(0.2, 0.3, 0.3, 1.0) {
        Ok(()) => println!("✅ Screen cleared successfully"),
        Err(e) => println!("⚠️  Screen clear failed (expected): {}", e),
    }
    
    // Try to draw a rectangle (will fail gracefully)
    println!("Attempting to draw rectangle...");
    let position = Vec2::new(0.0, 0.0);
    let size = Vec2::new(100.0, 100.0);
    let color = (1.0, 0.0, 0.0); // Red
    
    match renderer.draw_rect(position, size, color) {
        Ok(()) => println!("✅ Rectangle drawn successfully"),
        Err(e) => println!("⚠️  Rectangle drawing failed (expected): {}", e),
    }
    
    println!("\n=== Safe Wrapper Features ===");
    println!("✅ All unsafe OpenGL code contained in GlWrapper");
    println!("✅ Error handling with Result types");
    println!("✅ Graceful failure when OpenGL not initialized");
    println!("✅ Clean API for engine users");
    println!("✅ No unsafe code visible to engine users");
    
    println!("\n=== Next Steps ===");
    println!("1. Implement OpenGL context creation");
    println!("2. Call gl_wrapper.initialize() when context is ready");
    println!("3. Call renderer.initialize() to set up shaders");
    println!("4. Add rendering calls to game loop");
    println!("5. See colored rectangles on screen!");
    
    println!("\nPress 'Q' or 'ESC' to quit");
    
    // Run the engine
    engine.run();
    Ok(())
}
