use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::animation::Animation;
use engine_2d::render::sprite::{Sprite, SpriteRenderer};
use engine_2d::render::texture::TextureId;
use engine_2d::engine::window::WindowManager;
use engine_2d::render::simple_text::SimpleTextRenderer;
use glam::Vec2;

/// Circular animation - sprites move in circular patterns
struct CircularAnimation {
    name: String,
    red_texture_id: Option<TextureId>,
}

impl CircularAnimation {
    fn new() -> Self {
        Self {
            name: "Circular Animation".to_string(),
            red_texture_id: None,
        }
    }
    
    fn initialize(&mut self, sprite_renderer: &mut SpriteRenderer) -> Result<(), String> {
        if self.red_texture_id.is_none() {
            self.red_texture_id = Some(sprite_renderer.texture_manager().create_color_texture(64, 64, (255, 0, 0, 255))?);
        }
        Ok(())
    }
}

impl Animation for CircularAnimation {
    fn update(&mut self, sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32, _delta_time: f32, _window_manager: Option<&mut WindowManager>, _text_renderer: Option<&mut SimpleTextRenderer>) {
        // Only render if we have a sprite renderer (OpenGL mode)
        let Some(sprite_renderer) = sprite_renderer else {
            // In headless mode, just do the animation logic without rendering
            return;
        };
        
        // Initialize texture if not already done
        if let Err(e) = self.initialize(sprite_renderer) {
            eprintln!("Failed to initialize red texture: {}", e);
            return;
        }
        
        let red_texture = match &self.red_texture_id {
            Some(id) => *id,
            None => {
                eprintln!("Red texture not initialized");
                return;
            }
        };
        
        // Create sprites in circular motion
        for i in 0..4 {
            let offset = i as f32 * 0.5;
            let x = (elapsed_time + offset).sin() * 0.3;
            let y = (elapsed_time * 0.7 + offset).cos() * 0.2;
            
            let sprite = Sprite::new(red_texture, Vec2::new(x, y), Vec2::new(0.1, 0.1));
            if let Err(e) = sprite_renderer.render_sprite(&sprite) {
                eprintln!("Failed to render sprite: {}", e);
            }
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for CircularAnimation {
    fn drop(&mut self) {
        // Note: We can't clean up textures here because we don't have access to the sprite_renderer
        // The texture manager will clean up all textures when it's dropped
    }
}

/// Bouncing animation - sprites bounce up and down
struct BouncingAnimation {
    name: String,
    green_texture_id: Option<TextureId>,
}

impl BouncingAnimation {
    fn new() -> Self {
        Self {
            name: "Bouncing Animation".to_string(),
            green_texture_id: None,
        }
    }
    
    fn initialize(&mut self, sprite_renderer: &mut SpriteRenderer) -> Result<(), String> {
        if self.green_texture_id.is_none() {
            self.green_texture_id = Some(sprite_renderer.texture_manager().create_color_texture(64, 64, (0, 255, 0, 255))?);
        }
        Ok(())
    }
}

impl Animation for BouncingAnimation {
    fn update(&mut self, sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32, _delta_time: f32, _window_manager: Option<&mut WindowManager>, _text_renderer: Option<&mut SimpleTextRenderer>) {
        // Only render if we have a sprite renderer (OpenGL mode)
        let Some(sprite_renderer) = sprite_renderer else {
            // In headless mode, just do the animation logic without rendering
            return;
        };
        
        // Initialize texture if not already done
        if let Err(e) = self.initialize(sprite_renderer) {
            eprintln!("Failed to initialize green texture: {}", e);
            return;
        }
        
        let green_texture = match &self.green_texture_id {
            Some(id) => *id,
            None => {
                eprintln!("Green texture not initialized");
                return;
            }
        };
        
        // Create sprites in bouncing motion
        for i in 0..4 {
            let offset = i as f32 * 0.3;
            let bounce_height = (elapsed_time * 2.0 + offset).sin().abs() * 0.4;
            let x = (i as f32 - 1.5) * 0.2; // Spread horizontally
            let y = bounce_height - 0.2; // Center the bounce
            
            let sprite = Sprite::new(green_texture, Vec2::new(x, y), Vec2::new(0.1, 0.1));
            if let Err(e) = sprite_renderer.render_sprite(&sprite) {
                eprintln!("Failed to render sprite: {}", e);
            }
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for BouncingAnimation {
    fn drop(&mut self) {
        // Note: We can't clean up textures here because we don't have access to the sprite_renderer
        // The texture manager will clean up all textures when it's dropped
    }
}

/// Spinning animation - sprites rotate around a center point
struct SpinningAnimation {
    name: String,
    blue_texture_id: Option<TextureId>,
}

impl SpinningAnimation {
    fn new() -> Self {
        Self {
            name: "Spinning Animation".to_string(),
            blue_texture_id: None,
        }
    }
    
    fn initialize(&mut self, sprite_renderer: &mut SpriteRenderer) -> Result<(), String> {
        if self.blue_texture_id.is_none() {
            self.blue_texture_id = Some(sprite_renderer.texture_manager().create_color_texture(64, 64, (0, 0, 255, 255))?);
        }
        Ok(())
    }
}

impl Animation for SpinningAnimation {
    fn update(&mut self, sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32, _delta_time: f32, _window_manager: Option<&mut WindowManager>, _text_renderer: Option<&mut SimpleTextRenderer>) {
        // Only render if we have a sprite renderer (OpenGL mode)
        let Some(sprite_renderer) = sprite_renderer else {
            // In headless mode, just do the animation logic without rendering
            return;
        };
        
        // Initialize texture if not already done
        if let Err(e) = self.initialize(sprite_renderer) {
            eprintln!("Failed to initialize blue texture: {}", e);
            return;
        }
        
        let blue_texture = match &self.blue_texture_id {
            Some(id) => *id,
            None => {
                eprintln!("Blue texture not initialized");
                return;
            }
        };
        
        // Create sprites in spinning motion
        for i in 0..4 {
            let angle = elapsed_time * 1.5 + (i as f32 * 1.57); // 90 degrees apart
            let radius = 0.3;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            
            let sprite = Sprite::new(blue_texture, Vec2::new(x, y), Vec2::new(0.1, 0.1));
            if let Err(e) = sprite_renderer.render_sprite(&sprite) {
                eprintln!("Failed to render sprite: {}", e);
            }
        }
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for SpinningAnimation {
    fn drop(&mut self) {
        // Note: We can't clean up textures here because we don't have access to the sprite_renderer
        // The texture manager will clean up all textures when it's dropped
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
        viewport: engine_2d::engine::config::ViewportConfig::ui_based(),
        fallback_font_path: "assets/fonts/default.ttf".to_string(),
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
