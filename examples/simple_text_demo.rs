use engine_2d::animation::Animation;
use engine_2d::engine::window::{WindowEvent, WindowManager};
use engine_2d::render::sprite::SpriteRenderer;
use engine_2d::render::simple_text::SimpleTextRenderer;
use glfw::{Action, Key};

const DEFAULT_FONT_PATH: &str = "assets/fonts/default.ttf";

/// Simple text rendering demo showcasing the new intuitive API
pub struct SimpleTextDemo {
    current_demo: usize,
    demos: Vec<&'static str>,
    last_action_states: std::collections::HashMap<Key, bool>,
}

impl SimpleTextDemo {
    pub fn new() -> Self {
        Self {
            current_demo: 0,
            demos: vec![
                "Basic Text",
                "Colored Text", 
                "Aligned Text",
                "Styled Text",
                "Mixed Content",
                "Text Wrapping"
            ],
            last_action_states: std::collections::HashMap::new(),
        }
    }
    
    fn render_demo(&self, text_renderer: &mut SimpleTextRenderer, demo_name: &str) -> Result<(), String> {
        match demo_name {
            "Basic Text" => {
                text_renderer.draw_text_centered_sized("Simple Text Demo", 0.5, 0.1, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_centered_sized("Basic Text Rendering", 0.5, 0.2, "default", DEFAULT_FONT_PATH, 14)?;
                text_renderer.draw_text_sized("This is normal text", 0.1, 0.4, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("More text here", 0.1, 0.5, "default", DEFAULT_FONT_PATH, 14)?;
                text_renderer.draw_text_sized("And even more text", 0.1, 0.6, "default", DEFAULT_FONT_PATH, 12)?;
                
                // Example using top-left coordinate system
                text_renderer.draw_text_colored_top_left("Top-left coords (0.1, 0.1)", 0.1, 0.1, "default", DEFAULT_FONT_PATH, 14, 1.0, 0.0, 1.0)?;
                text_renderer.draw_text_colored_top_left("Top-right coords (0.9, 0.1)", 0.9, 0.1, "default", DEFAULT_FONT_PATH, 14, 0.0, 1.0, 1.0)?;
            },
            "Colored Text" => {
                text_renderer.draw_text_centered_sized("Colored Text", 0.5, 0.1, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_colored_sized("Red text", 0.1, 0.3, "default", DEFAULT_FONT_PATH, 16, 1.0, 0.0, 0.0)?;
                text_renderer.draw_text_colored_sized("Green text", 0.1, 0.4, "default", DEFAULT_FONT_PATH, 16, 0.0, 1.0, 0.0)?;
                text_renderer.draw_text_colored_sized("Blue text", 0.1, 0.5, "default", DEFAULT_FONT_PATH, 16, 0.0, 0.0, 1.0)?;
                text_renderer.draw_text_colored_sized("Yellow text", 0.1, 0.6, "default", DEFAULT_FONT_PATH, 16, 1.0, 1.0, 0.0)?;
                text_renderer.draw_text_colored_sized("Purple text", 0.1, 0.7, "default", DEFAULT_FONT_PATH, 16, 1.0, 0.0, 1.0)?;
            },
            "Aligned Text" => {
                text_renderer.draw_text_centered_sized("Text Alignment", 0.5, 0.1, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("Left aligned text", 0.1, 0.3, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_centered_sized("Centered text", 0.5, 0.4, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("Right aligned text", 0.9, 0.5, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("Left again", 0.1, 0.6, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_centered_sized("Center again", 0.5, 0.7, "default", DEFAULT_FONT_PATH, 16)?;
            },
            "Styled Text" => {
                text_renderer.draw_text_centered_sized("Text Styles", 0.5, 0.1, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("This is info text", 0.1, 0.3, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_colored_sized("This is warning text", 0.1, 0.4, "default", DEFAULT_FONT_PATH, 16, 1.0, 0.5, 0.0)?;
                text_renderer.draw_text_colored_sized("This is error text", 0.1, 0.5, "default", DEFAULT_FONT_PATH, 16, 1.0, 0.0, 0.0)?;
                text_renderer.draw_text_colored_sized("This is success text", 0.1, 0.6, "default", DEFAULT_FONT_PATH, 16, 0.0, 1.0, 0.0)?;
                text_renderer.draw_text_colored_sized("Semi-transparent text", 0.1, 0.7, "default", DEFAULT_FONT_PATH, 16, 1.0, 1.0, 1.0)?;
            },
            "Mixed Content" => {
                text_renderer.draw_text_centered_sized("Mixed Content Demo", 0.5, 0.05, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_centered_sized("Combining different styles", 0.5, 0.15, "default", DEFAULT_FONT_PATH, 16)?;
                
                text_renderer.draw_text_sized("Game Status:", 0.1, 0.25, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_colored_sized("✓ Player Connected", 0.15, 0.3, "default", DEFAULT_FONT_PATH, 16, 0.0, 1.0, 0.0)?;
                text_renderer.draw_text_sized("ℹ Score: 1,250 points", 0.15, 0.35, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_colored_sized("⚠ Low Health: 25%", 0.15, 0.4, "default", DEFAULT_FONT_PATH, 16, 1.0, 0.5, 0.0)?;
                
                text_renderer.draw_text_sized("Controls:", 0.1, 0.5, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("WASD - Move", 0.15, 0.55, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("Space - Jump", 0.15, 0.6, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_sized("ESC - Menu", 0.15, 0.65, "default", DEFAULT_FONT_PATH, 16)?;
                
                text_renderer.draw_text_centered_sized("Press SPACE for next demo", 0.5, 0.8, "default", DEFAULT_FONT_PATH, 16)?;
                text_renderer.draw_text_centered_sized("Press ESC to exit", 0.5, 0.85, "default", DEFAULT_FONT_PATH, 16)?;
            },
            "Text Wrapping" => {
                text_renderer.draw_text_centered_sized("Text Wrapping Demo", 0.5, 0.1, "default", DEFAULT_FONT_PATH, 16)?;
                
                // Word wrapping example - positioned in top area
                let long_text = "This is a very long text that should wrap at word boundaries when it exceeds the maximum width. It demonstrates how text wrapping works in the engine.";
                text_renderer.draw_text_wrapped_top_left(long_text, 0.1, 0.25, "default", DEFAULT_FONT_PATH, 14, 0.8)?;
                
                // Ellipsis truncation example - positioned in middle area
                let truncate_text = "This text is too long and will be truncated with ellipsis when it exceeds the maximum width.";
                text_renderer.draw_text_ellipsis_top_left(truncate_text, 0.1, 0.55, "default", DEFAULT_FONT_PATH, 14, 0.6)?;
                
                // Labels - positioned above their respective examples
                text_renderer.draw_text_colored_top_left("Word Wrapping:", 0.1, 0.2, "default", DEFAULT_FONT_PATH, 12, 0.0, 1.0, 0.0)?;
                text_renderer.draw_text_colored_top_left("Ellipsis Truncation:", 0.1, 0.5, "default", DEFAULT_FONT_PATH, 12, 1.0, 0.0, 0.0)?;
            },
            _ => {}
        }
        
        Ok(())
    }
}

impl Animation for SimpleTextDemo {
    fn update(&mut self, _sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32, _delta_time: f32, _window_manager: Option<&mut WindowManager>, text_renderer: Option<&mut SimpleTextRenderer>) {
        
        // Render current demo
        if let Some(tr) = text_renderer {
            // Viewport is configured at engine initialization
            // Text is viewport-independent by default - font size 20 looks the same regardless of viewport scale
            
            // Render the current demo (fonts are loaded on-demand)
            if let Err(e) = self.render_demo(tr, self.demos[self.current_demo]) {
                println!("Error rendering demo: {}", e);
            }
            
            // Show demo info (top of screen) using top-left coordinates
            let demo_info = format!("Demo {} of {}: {}", 
                self.current_demo + 1, 
                self.demos.len(), 
                self.demos[self.current_demo]
            );
            let _ = tr.draw_text_top_left(&demo_info, 0.02, 0.02, "default", DEFAULT_FONT_PATH, 14);
            
            // Show controls (bottom of screen) using top-left coordinates
            let _ = tr.draw_text_top_left("SPACE=Next | BACKSPACE=Prev | ESC=Exit", 0.02, 0.98, "default", DEFAULT_FONT_PATH, 14);
        }
        
        // Print demo info every 5 seconds
        if (elapsed_time as u32) % 5 == 0 && (elapsed_time * 1000.0) as u32 % 5000 < 16 {
            println!("Current Demo: {} ({})", self.demos[self.current_demo], self.current_demo + 1);
        }
    }
    
    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Glfw(glfw::WindowEvent::Key(key, _scancode, action, _mods)) => {
                let was_pressed = self.last_action_states.get(key).copied().unwrap_or(false);
                let is_pressed = *action == Action::Press || *action == Action::Repeat;
                
                if is_pressed && !was_pressed {
                    match key {
                        Key::Space => {
                            self.current_demo = (self.current_demo + 1) % self.demos.len();
                        },
                        Key::Backspace => {
                            self.current_demo = if self.current_demo == 0 { 
                                self.demos.len() - 1 
                            } else { 
                                self.current_demo - 1 
                            };
                        },
                        Key::Escape => {
                            // Exit will be handled by the engine
                        },
                        _ => {}
                    }
                }
                
                self.last_action_states.insert(*key, is_pressed);
            },
            _ => {}
        }
    }
    
    fn name(&self) -> &str {
        "Simple Text Demo"
    }
}


fn main() {
    let config = engine_2d::engine::config::EngineConfig {
        window_width: 1024,
        window_height: 768,
        window_title: "Simple Text Demo".to_string(),
        target_fps: Some(60),
        show_fps: false,
        vsync: true,
        fullscreen: false,
        // Configure viewport - you can choose from several presets or create custom bounds
        viewport: engine_2d::engine::config::ViewportConfig::default(), // Default: (-1, 1, -1, 1)
        // viewport: engine_2d::engine::config::ViewportConfig::ndc(), // Traditional OpenGL: (-10, 10, -10, 10)
        // viewport: engine_2d::engine::config::ViewportConfig::ui_based(), // UI coordinates: (0, 1, 0, 1)
        // viewport: engine_2d::engine::config::ViewportConfig::pixel_based(1024.0, 768.0), // Pixel coordinates
        // viewport: engine_2d::engine::config::ViewportConfig::with_bounds(-5.0, 5.0, -3.0, 3.0), // Custom bounds
    };
    
    let animation = Box::new(SimpleTextDemo::new());
    
    match engine_2d::engine::core::Engine::new_with_config_and_animation(config, animation) {
        Ok(mut engine) => {
            println!("Simple Text Demo");
            println!("================");
            println!("Controls:");
            println!("  SPACE     - Next Demo");
            println!("  BACKSPACE - Previous Demo");
            println!("  ESC       - Exit");
            println!();
            
            if let Err(e) = engine.run() {
                eprintln!("Engine error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to create engine: {}", e);
        }
    }
}
