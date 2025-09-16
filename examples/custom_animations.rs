use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::animation::Animation;
use engine_2d::render::sprite::Sprite;
use glam::Vec2;

/// Circular animation - sprites move in circular patterns
struct CircularAnimation {
    name: String,
}

impl CircularAnimation {
    fn new() -> Self {
        Self {
            name: "Circular Animation".to_string(),
        }
    }
}

impl Animation for CircularAnimation {
    fn update(&self, sprites: &mut [Sprite], elapsed_time: f32) {
        for (i, sprite) in sprites.iter_mut().enumerate() {
            let offset = i as f32 * 0.5;
            let x = (elapsed_time + offset).sin() * 0.3;
            let y = (elapsed_time * 0.7 + offset).cos() * 0.2;
            sprite.set_position(Vec2::new(x, y));
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Bouncing animation - sprites bounce up and down
struct BouncingAnimation {
    name: String,
}

impl BouncingAnimation {
    fn new() -> Self {
        Self {
            name: "Bouncing Animation".to_string(),
        }
    }
}

impl Animation for BouncingAnimation {
    fn update(&self, sprites: &mut [Sprite], elapsed_time: f32) {
        for (i, sprite) in sprites.iter_mut().enumerate() {
            let offset = i as f32 * 0.3;
            let bounce_height = (elapsed_time * 2.0 + offset).sin().abs() * 0.4;
            let x = sprite.position.x; // Keep X position fixed
            let y = bounce_height - 0.2; // Center the bounce
            sprite.set_position(Vec2::new(x, y));
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

/// Spinning animation - sprites rotate around a center point
struct SpinningAnimation {
    name: String,
}

impl SpinningAnimation {
    fn new() -> Self {
        Self {
            name: "Spinning Animation".to_string(),
        }
    }
}

impl Animation for SpinningAnimation {
    fn update(&self, sprites: &mut [Sprite], elapsed_time: f32) {
        for (i, sprite) in sprites.iter_mut().enumerate() {
            let angle = elapsed_time * 1.5 + (i as f32 * 1.57); // 90 degrees apart
            let radius = 0.3;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            sprite.set_position(Vec2::new(x, y));
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Custom Animations Example");
    println!("Demonstrating different types of custom animations");
    
    // Create a custom configuration
    let config = EngineConfig {
        window_title: "Rust 2D Engine - Custom Animations".to_string(),
        window_width: 800,
        window_height: 600,
        target_fps: Some(60),
        show_fps: true,
        vsync: true,
        fullscreen: false,
    };
    
    // Choose which animation to demonstrate
    let animation_choice = std::env::args().nth(1).unwrap_or_else(|| "circular".to_string());
    
    let animation: Box<dyn Animation> = match animation_choice.as_str() {
        "bouncing" => Box::new(BouncingAnimation::new()),
        "spinning" => Box::new(SpinningAnimation::new()),
        _ => Box::new(CircularAnimation::new()),
    };
    
    println!("Using animation: {}", animation.name());
    
    // Create engine with the chosen animation
    let mut engine = Engine::new_with_config_and_animation(config, animation)?;
    
    println!("Engine created successfully!");
    println!("Window: {} ({}x{})", 
             engine.get_window_manager().get_title(),
             engine.get_window_manager().get_size().0,
             engine.get_window_manager().get_size().1);
    
    println!("\n=== Available Animations ===");
    println!("Run with argument to choose animation:");
    println!("  cargo run --example custom_animations circular  (default)");
    println!("  cargo run --example custom_animations bouncing");
    println!("  cargo run --example custom_animations spinning");
    
    println!("\nPress 'Q' or 'ESC' to quit");
    
    // Run the engine with the chosen animation
    engine.run()?;
    
    println!("Custom animations demo completed!");
    Ok(())
}
