use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::animation::Animation;
use engine_2d::render::sprite::{Sprite, SpriteRenderer};
use glam::Vec2;

/// Custom pendulum animation implementation
/// 
/// This demonstrates how game makers can create their own animations
/// by implementing the Animation trait
struct PendulumAnimation {
    name: String,
}

impl PendulumAnimation {
    fn new() -> Self {
        Self {
            name: "Pendulum Animation".to_string(),
        }
    }
}

impl Animation for PendulumAnimation {
    fn update(&self, sprite_renderer: &mut SpriteRenderer, elapsed_time: f32) {
        // Create sprites each frame (simple approach for demonstration)
        // In a real game, you'd want to create them once and store them
        
        // Create color textures
        let red_texture = match sprite_renderer.texture_manager().create_color_texture(64, 64, (255, 0, 0, 255)) {
            Ok(id) => id,
            Err(_) => return, // Skip this frame if texture creation fails
        };
        let green_texture = match sprite_renderer.texture_manager().create_color_texture(64, 64, (0, 255, 0, 255)) {
            Ok(id) => id,
            Err(_) => return,
        };
        let blue_texture = match sprite_renderer.texture_manager().create_color_texture(64, 64, (0, 0, 255, 255)) {
            Ok(id) => id,
            Err(_) => return,
        };
        let yellow_texture = match sprite_renderer.texture_manager().create_color_texture(64, 64, (255, 255, 0, 255)) {
            Ok(id) => id,
            Err(_) => return,
        };
        
        // Create sprites stacked in the center (as requested)
        let mut sprites = vec![
            Sprite::new(red_texture, Vec2::new(0.0, 0.15), Vec2::new(0.2, 0.2)),   // Top
            Sprite::new(green_texture, Vec2::new(0.0, 0.05), Vec2::new(0.2, 0.2)),  // Upper middle
            Sprite::new(blue_texture, Vec2::new(0.0, -0.05), Vec2::new(0.2, 0.2)),  // Lower middle
            Sprite::new(yellow_texture, Vec2::new(0.0, -0.15), Vec2::new(0.2, 0.2)), // Bottom
        ];
        
        // Animate the sprites
        for (i, sprite) in sprites.iter_mut().enumerate() {
            // All sprites share the same period (same frequency)
            let pendulum_time = elapsed_time * 1.0; // Same speed for all sprites
            
            // Each sprite travels a different distance (amplitude) but same period
            // Sprite 0: amplitude 0.4, Sprite 1: amplitude 0.27, Sprite 2: amplitude 0.18, Sprite 3: amplitude 0.12
            let amplitude = 0.4 / (1.5_f32.powi(i as i32));
            
            // Pendulum motion: sin wave for left-right movement ONLY
            let x = pendulum_time.sin() * amplitude;
            
            // Keep Y position fixed - NO VERTICAL MOVEMENT
            let y = sprite.position.y;
            sprite.set_position(Vec2::new(x, y));
            
            // Render the sprite
            if let Err(e) = sprite_renderer.render_sprite(sprite) {
                eprintln!("Failed to render sprite: {}", e);
            }
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Pendulum Animation Example");
    println!("Demonstrating how to create custom animations");
    
    // Create a custom configuration
    let config = EngineConfig {
        window_title: "Rust 2D Engine - Custom Pendulum Animation".to_string(),
        window_width: 800,
        window_height: 600,
        target_fps: Some(60),
        show_fps: true,
        vsync: true,
        fullscreen: false,
    };
    
    // Create our custom pendulum animation
    let pendulum_animation = Box::new(PendulumAnimation::new());
    
    // Create engine with our custom animation
    let mut engine = Engine::new_with_config_and_animation(config, pendulum_animation)?;
    
    println!("Engine created successfully!");
    println!("Window: {} ({}x{})", 
             engine.get_window_manager().get_title(),
             engine.get_window_manager().get_size().0,
             engine.get_window_manager().get_size().1);
    
    println!("\n=== Custom Animation Features ===");
    println!("✅ 4 sprites positioned in center of window");
    println!("✅ Each sprite moves left and right along X-axis ONLY");
    println!("✅ Same period for all sprites (synchronized)");
    println!("✅ Each sprite travels different distances (varied amplitudes)");
    println!("✅ All sprites hit leftmost/rightmost points in sync");
    println!("✅ Mimics pendulums swaying in time");
    
    println!("\n=== How to Create Custom Animations ===");
    println!("1. Implement the Animation trait");
    println!("2. Define your animation logic in the update() method");
    println!("3. Pass your animation to Engine::new_with_config_and_animation()");
    println!("4. The engine will call your update() method each frame");
    
    println!("\nPress 'Q' or 'ESC' to quit");
    
    // Run the engine with our custom animation
    engine.run()?;
    
    println!("Custom pendulum animation demo completed!");
    Ok(())
}
