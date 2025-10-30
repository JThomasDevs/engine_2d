use engine_2d::animation::Animation;
use engine_2d::engine::window::{WindowEvent, WindowManager};
use engine_2d::render::simple_text::{Font, SimpleTextRenderer, TextAnchor};
use engine_2d::render::sprite::SpriteRenderer;
use glfw::{Action, Key};

const DEFAULT_FONT_PATH: &str = "assets/fonts/default.ttf";

/// Simple text rendering demo showcasing the new intuitive API
pub struct SimpleTextDemo {
    current_demo: usize,
    demos: Vec<&'static str>,
    last_action_states: std::collections::HashMap<Key, bool>,
    fonts_registered: bool,
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
                "Text Wrapping",
                "Anchor Points",
                "Variable Length Anchors",
            ],
            last_action_states: std::collections::HashMap::new(),
            fonts_registered: false,
        }
    }

    /// Register fonts with the text renderer
    fn register_fonts(&mut self, text_renderer: &mut SimpleTextRenderer) {
        if !self.fonts_registered {
            let default_font = Font::new("default", DEFAULT_FONT_PATH);
            text_renderer.register_font("default", default_font);
            self.fonts_registered = true;
        }
    }

    fn render_demo(
        &self,
        text_renderer: &mut SimpleTextRenderer,
        demo_name: &str,
    ) -> Result<(), String> {
        match demo_name {
            "Basic Text" => {
                // Using the new fluent API
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Simple Text Demo", 0.5, 0.1)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Basic Text Rendering", 0.5, 0.2)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .draw("This is normal text", 0.1, 0.4)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .draw("More text here", 0.1, 0.5)?;
                text_renderer
                    .font("default")
                    .size(12)
                    .draw("And even more text", 0.1, 0.6)?;

                // Example using top-left coordinate system with colored text
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 0.0, 1.0)
                    .draw("Top-left coords (0.1, 0.1)", 0.1, 0.1)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 1.0)
                    .draw("Top-right coords (0.9, 0.1)", 0.9, 0.1)?;
            }
            "Colored Text" => {
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Colored Text", 0.5, 0.1)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.0, 0.0)
                    .draw("Red text", 0.1, 0.3)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(0.0, 1.0, 0.0)
                    .draw("Green text", 0.1, 0.4)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(0.0, 0.0, 1.0)
                    .draw("Blue text", 0.1, 0.5)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .draw("Yellow text", 0.1, 0.6)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.0, 1.0)
                    .draw("Purple text", 0.1, 0.7)?;
            }
            "Aligned Text" => {
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Text Alignment", 0.5, 0.1)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .align(engine_2d::render::text::TextAlign::Left)
                    .draw("Left aligned text", 0.1, 0.3)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Centered text", 0.5, 0.4)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .align(engine_2d::render::text::TextAlign::Right)
                    .draw("Right aligned text", 0.9, 0.5)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .align(engine_2d::render::text::TextAlign::Left)
                    .draw("Left again", 0.1, 0.6)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Center again", 0.5, 0.7)?;
            }
            "Styled Text" => {
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Text Styles", 0.5, 0.1)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .draw("This is info text", 0.1, 0.3)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.5, 0.0)
                    .draw("This is warning text", 0.1, 0.4)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.0, 0.0)
                    .draw("This is error text", 0.1, 0.5)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(0.0, 1.0, 0.0)
                    .draw("This is success text", 0.1, 0.6)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 1.0)
                    .alpha(0.7)
                    .draw("Semi-transparent text", 0.1, 0.7)?;
            }
            "Mixed Content" => {
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Mixed Content Demo", 0.5, 0.05)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Combining different styles", 0.5, 0.15)?;

                text_renderer
                    .font("default")
                    .size(16)
                    .draw("Game Status:", 0.1, 0.25)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(0.0, 1.0, 0.0)
                    .draw("✓ Player Connected", 0.15, 0.3)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .draw("ℹ Score: 1,250 points", 0.15, 0.35)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.5, 0.0)
                    .draw("⚠ Low Health: 25%", 0.15, 0.4)?;

                text_renderer
                    .font("default")
                    .size(16)
                    .draw("Controls:", 0.1, 0.5)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .draw("WASD - Move", 0.15, 0.55)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .draw("Space - Jump", 0.15, 0.6)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .draw("ESC - Menu", 0.15, 0.65)?;

                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Press SPACE for next demo", 0.5, 0.8)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Press ESC to exit", 0.5, 0.85)?;
            }
            "Text Wrapping" => {
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Text Wrapping Demo", 0.5, 0.1)?;

                // Word wrapping example - positioned in top area
                let long_text = "This is a very long text that should wrap at word boundaries when it exceeds the maximum width. It demonstrates how text wrapping works in the engine.";
                text_renderer
                    .font("default")
                    .size(14)
                    .max_width(0.8)
                    .draw(long_text, 0.1, 0.25)?;

                // Ellipsis truncation example - positioned in middle area
                let truncate_text = "This text is too long and will be truncated with ellipsis when it exceeds the maximum width.";
                text_renderer.font("default").size(14).max_width(0.6).draw(
                    truncate_text,
                    0.1,
                    0.55,
                )?;

                // Labels - positioned above their respective examples
                text_renderer
                    .font("default")
                    .size(12)
                    .color(0.0, 1.0, 0.0)
                    .draw("Word Wrapping:", 0.1, 0.2)?;
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.0, 0.0)
                    .draw("Ellipsis Truncation:", 0.1, 0.5)?;
            }
            "Anchor Points" => {
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Anchor Points Demo", 0.5, 0.1)?;

                // Draw reference crosshairs at key positions
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.2, 0.3)?; // Top-left reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.5, 0.3)?; // Top-center reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.8, 0.3)?; // Top-right reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.2, 0.5)?; // Middle-left reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.5, 0.5)?; // Middle-center reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.8, 0.5)?; // Middle-right reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.2, 0.7)?; // Bottom-left reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.5, 0.7)?; // Bottom-center reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.8, 0.7)?; // Bottom-right reference

                // Demonstrate different anchor points
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 0.0, 0.0)
                    .anchor(TextAnchor::TopLeft)
                    .draw("TL", 0.2, 0.3)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 0.0, 0.0)
                    .anchor(TextAnchor::TopCenter)
                    .draw("TC", 0.5, 0.3)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 0.0, 0.0)
                    .anchor(TextAnchor::TopRight)
                    .draw("TR", 0.8, 0.3)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 0.0)
                    .anchor(TextAnchor::MiddleLeft)
                    .draw("ML", 0.2, 0.5)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 0.0)
                    .anchor(TextAnchor::MiddleCenter)
                    .draw("MC", 0.5, 0.5)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 0.0)
                    .anchor(TextAnchor::MiddleRight)
                    .draw("MR", 0.8, 0.5)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 0.0, 1.0)
                    .anchor(TextAnchor::BottomLeft)
                    .draw("BL", 0.2, 0.7)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 0.0, 1.0)
                    .anchor(TextAnchor::BottomCenter)
                    .draw("BC", 0.5, 0.7)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 0.0, 1.0)
                    .anchor(TextAnchor::BottomRight)
                    .draw("BR", 0.8, 0.7)?;

                // Show practical examples
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 1.0, 1.0)
                    .draw("Easy right-edge alignment:", 0.1, 0.85)?;
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 1.0, 0.0)
                    .anchor(TextAnchor::TopRight)
                    .draw("Right edge text", 0.95, 0.85)?;
            }
            "Variable Length Anchors" => {
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 1.0, 0.0)
                    .align(engine_2d::render::text::TextAlign::Center)
                    .draw("Variable Length Text Anchoring", 0.5, 0.05)?;

                // Draw reference crosshairs at key positions
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.2, 0.2)?; // Top-left reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.5, 0.2)?; // Top-center reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.8, 0.2)?; // Top-right reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.2, 0.4)?; // Middle-left reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.5, 0.4)?; // Middle-center reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.8, 0.4)?; // Middle-right reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.2, 0.6)?; // Bottom-left reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.5, 0.6)?; // Bottom-center reference
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 0.41, 0.71)
                    .draw("+", 0.8, 0.6)?; // Bottom-right reference

                // Short text examples
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 0.0, 0.0)
                    .anchor(TextAnchor::TopLeft)
                    .draw("Hi", 0.2, 0.2)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 0.0, 0.0)
                    .anchor(TextAnchor::TopCenter)
                    .draw("Hi", 0.5, 0.2)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(1.0, 0.0, 0.0)
                    .anchor(TextAnchor::TopRight)
                    .draw("Hi", 0.8, 0.2)?;

                // Medium text examples
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 0.0)
                    .anchor(TextAnchor::MiddleLeft)
                    .draw("Hello World", 0.2, 0.4)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 0.0)
                    .anchor(TextAnchor::MiddleCenter)
                    .draw("Hello World", 0.5, 0.4)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 0.0)
                    .anchor(TextAnchor::MiddleRight)
                    .draw("Hello World", 0.8, 0.4)?;

                // Long text examples
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 0.0, 1.0)
                    .anchor(TextAnchor::BottomLeft)
                    .draw("This is a very long text example", 0.2, 0.6)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 0.0, 1.0)
                    .anchor(TextAnchor::BottomCenter)
                    .draw("This is a very long text example", 0.5, 0.6)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 0.0, 1.0)
                    .anchor(TextAnchor::BottomRight)
                    .draw("This is a very long text example", 0.8, 0.6)?;

                // Additional examples with different sizes
                text_renderer
                    .font("default")
                    .size(12)
                    .color(1.0, 1.0, 0.0)
                    .anchor(TextAnchor::TopLeft)
                    .draw("Small", 0.1, 0.75)?;
                text_renderer
                    .font("default")
                    .size(18)
                    .color(1.0, 0.5, 0.0)
                    .anchor(TextAnchor::TopCenter)
                    .draw("Large Text", 0.5, 0.75)?;
                text_renderer
                    .font("default")
                    .size(10)
                    .color(0.5, 0.5, 1.0)
                    .anchor(TextAnchor::TopRight)
                    .draw("Tiny", 0.9, 0.75)?;

                // Mixed case examples
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.0, 1.0)
                    .anchor(TextAnchor::MiddleLeft)
                    .draw("Mixed Case Text", 0.1, 0.85)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.0, 1.0)
                    .anchor(TextAnchor::MiddleCenter)
                    .draw("Mixed Case Text", 0.5, 0.85)?;
                text_renderer
                    .font("default")
                    .size(16)
                    .color(1.0, 0.0, 1.0)
                    .anchor(TextAnchor::MiddleRight)
                    .draw("Mixed Case Text", 0.9, 0.85)?;

                // Numbers and symbols
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 1.0)
                    .anchor(TextAnchor::BottomLeft)
                    .draw("12345", 0.1, 0.95)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 1.0)
                    .anchor(TextAnchor::BottomCenter)
                    .draw("!@#$%", 0.5, 0.95)?;
                text_renderer
                    .font("default")
                    .size(14)
                    .color(0.0, 1.0, 1.0)
                    .anchor(TextAnchor::BottomRight)
                    .draw("ABCDEF", 0.9, 0.95)?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl Animation for SimpleTextDemo {
    fn update(
        &mut self,
        _sprite_renderer: Option<&mut SpriteRenderer>,
        elapsed_time: f32,
        _delta_time: f32,
        _window_manager: Option<&mut WindowManager>,
        text_renderer: Option<&mut SimpleTextRenderer>,
    ) {
        // Render current demo
        if let Some(tr) = text_renderer {
            // Register fonts if not already registered
            self.register_fonts(tr);

            // Viewport is configured at engine initialization
            // Text is viewport-independent by default - font size 20 looks the same regardless of viewport scale

            // Render the current demo (fonts are loaded on-demand)
            if let Err(e) = self.render_demo(tr, self.demos[self.current_demo]) {
                println!("Error rendering demo: {}", e);
            }

            // Show demo info (top of screen) using top-left coordinates
            let demo_info = format!(
                "Demo {} of {}: {}",
                self.current_demo + 1,
                self.demos.len(),
                self.demos[self.current_demo]
            );
            let _ = tr.draw_text_top_left(&demo_info, 0.02, 0.02, "default", DEFAULT_FONT_PATH, 14);

            // Show controls (bottom of screen) using top-left coordinates
            let _ = tr.draw_text_top_left(
                "SPACE=Next | BACKSPACE=Prev | ESC=Exit",
                0.02,
                0.98,
                "default",
                DEFAULT_FONT_PATH,
                14,
            );
        }

        // Print demo info every 5 seconds
        if (elapsed_time as u32) % 5 == 0 && (elapsed_time * 1000.0) as u32 % 5000 < 16 {
            println!(
                "Current Demo: {} ({})",
                self.demos[self.current_demo],
                self.current_demo + 1
            );
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
                        }
                        Key::Backspace => {
                            self.current_demo = if self.current_demo == 0 {
                                self.demos.len() - 1
                            } else {
                                self.current_demo - 1
                            };
                        }
                        Key::Escape => {
                            // Exit will be handled by the engine
                        }
                        _ => {}
                    }
                }

                self.last_action_states.insert(*key, is_pressed);
            }
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
        fallback_font_path: DEFAULT_FONT_PATH.to_string(),
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
