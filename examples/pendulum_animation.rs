use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::animation::Animation;
use engine_2d::render::sprite::{Sprite, SpriteRenderer};
use engine_2d::render::texture::TextureId;
use glam::Vec2;

/// Custom pendulum animation implementation
/// 
/// This demonstrates how game makers can create their own animations
/// by implementing the Animation trait
struct PendulumAnimation {
    name: String,
    red_texture: Option<TextureId>,
    green_texture: Option<TextureId>,
    blue_texture: Option<TextureId>,
    yellow_texture: Option<TextureId>,
}

impl PendulumAnimation {
    fn new() -> Self {
        Self {
            name: "Pendulum Animation".to_string(),
            red_texture: None,
            green_texture: None,
            blue_texture: None,
            yellow_texture: None,
        }
    }
    
    /// Initialize textures - call this after the sprite renderer is available
    fn initialize_textures(&mut self, sprite_renderer: &mut SpriteRenderer) -> Result<(), String> {
        // Create color textures once
        self.red_texture = Some(sprite_renderer.texture_manager().create_color_texture(64, 64, (255, 0, 0, 255))?);
        self.green_texture = Some(sprite_renderer.texture_manager().create_color_texture(64, 64, (0, 255, 0, 255))?);
        self.blue_texture = Some(sprite_renderer.texture_manager().create_color_texture(64, 64, (0, 0, 255, 255))?);
        self.yellow_texture = Some(sprite_renderer.texture_manager().create_color_texture(64, 64, (255, 255, 0, 255))?);
        Ok(())
    }
    
    /// Clean up textures
    #[allow(dead_code)] // Method is available for explicit cleanup but not used in current implementation
    fn cleanup_textures(&mut self, sprite_renderer: &mut SpriteRenderer) {
        if let Some(texture_id) = self.red_texture.take() {
            let _ = sprite_renderer.texture_manager().delete_texture(texture_id);
        }
        if let Some(texture_id) = self.green_texture.take() {
            let _ = sprite_renderer.texture_manager().delete_texture(texture_id);
        }
        if let Some(texture_id) = self.blue_texture.take() {
            let _ = sprite_renderer.texture_manager().delete_texture(texture_id);
        }
        if let Some(texture_id) = self.yellow_texture.take() {
            let _ = sprite_renderer.texture_manager().delete_texture(texture_id);
        }
    }
}

impl Animation for PendulumAnimation {
    fn update(&mut self, sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32) {
        // Only render if we have a sprite renderer (OpenGL mode)
        let Some(sprite_renderer) = sprite_renderer else {
            // In headless mode, just do the animation logic without rendering
            return;
        };
        
        // Initialize textures on first update if not already done
        if self.red_texture.is_none() {
            if let Err(e) = self.initialize_textures(sprite_renderer) {
                eprintln!("Failed to initialize textures: {}", e);
                return;
            }
        }
        
        // Get texture IDs (we know they exist at this point)
        let red_texture = self.red_texture.unwrap();
        let green_texture = self.green_texture.unwrap();
        let blue_texture = self.blue_texture.unwrap();
        let yellow_texture = self.yellow_texture.unwrap();
        
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

impl Drop for PendulumAnimation {
    fn drop(&mut self) {
        // Note: We can't access sprite_renderer here since it's not available in Drop
        // The cleanup will be handled by the engine when the animation is replaced
        // or by calling cleanup_textures explicitly before dropping
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
        viewport: engine_2d::engine::config::ViewportConfig::ui_based(),
        fallback_font_path: "assets/fonts/default.ttf".to_string(),
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
