/// Text Rendering Demo
/// 
/// This example demonstrates the text rendering capabilities of the engine:
/// - Loading fonts
/// - Rendering different text styles
/// - Text alignment
/// - Color variations
/// - Dynamic text updates

use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::input::*;
#[cfg(feature = "opengl")]
use engine_2d::render::sprite::SpriteRenderer;
#[cfg(feature = "opengl")]
use engine_2d::render::text::{Text, TextRenderer};
use engine_2d::render::simple_text::SimpleTextRenderer;
#[cfg(feature = "opengl")]
use engine_2d::render::text_utils::{TextUtils, colors, sizes};
#[cfg(feature = "opengl")]
use engine_2d::engine::window::{WindowManager, WindowEvent};
#[cfg(feature = "opengl")]
use glfw::{Action, Key};
use glam::Vec2;

// Text rendering demo animation
#[cfg(feature = "opengl")]
struct TextRenderingDemo {
    input_manager: InputManager,
    last_instruction_time: f32,
    last_action_states: std::collections::HashMap<String, bool>,
    demo_texts: Vec<Text>,
    demo_text_counts: Vec<usize>, // Number of texts per demo
    current_demo: usize,
    time_elapsed: f32,
    time_label_index: Option<usize>, // Index of the dynamic time label
}

#[cfg(feature = "opengl")]
impl TextRenderingDemo {
    fn new() -> Self {
        let mut input_manager = InputManager::new();
        
        // Define demo actions
        let demo_actions = vec![
            GameAction {
                id: "NEXT_DEMO".to_string(),
                display_name: "Next Demo".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Space)),
                ],
                metadata: ActionMetadata {
                    description: Some("Switch to next text demo".to_string()),
                    tags: vec!["demo".to_string(), "navigation".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "PREV_DEMO".to_string(),
                display_name: "Previous Demo".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Backspace)),
                ],
                metadata: ActionMetadata {
                    description: Some("Switch to previous text demo".to_string()),
                    tags: vec!["demo".to_string(), "navigation".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
        ];
        
        input_manager.register_actions(demo_actions);
        
        Self {
            input_manager,
            last_instruction_time: 0.0,
            last_action_states: std::collections::HashMap::new(),
            demo_texts: Vec::new(),
            demo_text_counts: Vec::new(),
            current_demo: 0,
            time_elapsed: 0.0,
            time_label_index: None,
        }
    }
    
    fn is_action_just_pressed(&mut self, action_id: &str) -> bool {
        let current_pressed = self.input_manager.is_action_pressed(action_id);
        let last_pressed = self.last_action_states.get(action_id).copied().unwrap_or(false);
        
        // Update the last state
        self.last_action_states.insert(action_id.to_string(), current_pressed);
        
        // Return true if currently pressed but wasn't pressed last frame
        current_pressed && !last_pressed
    }
    
    fn setup_demo_texts(&mut self, text_renderer: &mut TextRenderer) {
        // Load a default font (placeholder implementation)
        if !text_renderer.has_font("default") {
            match text_renderer.load_font("default", "assets/fonts/default.ttf", 16) {
                Ok(_) => println!("Font loaded successfully"),
                Err(e) => {
                    println!("ERROR: Could not load font: {}", e);
                    println!("Font file exists: {}", std::path::Path::new("assets/fonts/default.ttf").exists());
                    return; // Exit early if font loading fails
                }
            }
        }
        
        self.demo_texts.clear();
        self.demo_text_counts.clear();
        
        // Demo 1: Basic text styles (using normalized coordinates 0-1)
        let demo1_start = self.demo_texts.len();
        self.demo_texts.push(TextUtils::title_text("Text Rendering Demo", Vec2::new(0.5, 0.01), "default"));
        // Create a left-aligned subtitle for "Basic Text Styles"
        let mut basic_styles_text = TextUtils::subtitle_text("Basic Text Styles", Vec2::new(0.0, 0.5), "default");
        basic_styles_text.set_align(engine_2d::render::text::TextAlign::Left);
        self.demo_texts.push(basic_styles_text);
        self.demo_texts.push(TextUtils::simple_text("This is normal text", Vec2::new(0.02, 0.8), "default"));
        self.demo_texts.push(TextUtils::colored_text("This is colored text", Vec2::new(0.1, 0.75), "default", colors::GREEN));
        self.demo_texts.push(TextUtils::warning_text("This is warning text", Vec2::new(0.1, 0.7), "default"));
        self.demo_texts.push(TextUtils::error_text("This is error text", Vec2::new(0.1, 0.65), "default"));
        self.demo_texts.push(TextUtils::info_text("This is info text", Vec2::new(0.1, 0.6), "default"));
        self.demo_text_counts.push(self.demo_texts.len() - demo1_start);
        
        // Demo 2: Text alignment
        let demo2_start = self.demo_texts.len();
        self.demo_texts.push(TextUtils::title_text("Text Alignment Demo", Vec2::new(0.5, 0.9), "default"));
        self.demo_texts.push(TextUtils::centered_text("This text is centered", Vec2::new(0.5, 0.8), "default"));
        self.demo_texts.push(TextUtils::right_aligned_text("This text is right-aligned", Vec2::new(0.9, 0.75), "default"));
        self.demo_texts.push(TextUtils::simple_text("This text is left-aligned", Vec2::new(0.1, 0.7), "default"));
        self.demo_text_counts.push(self.demo_texts.len() - demo2_start);
        
        // Demo 3: Font sizes
        let demo3_start = self.demo_texts.len();
        self.demo_texts.push(TextUtils::title_text("Font Sizes Demo", Vec2::new(0.5, 0.9), "default"));
        self.demo_texts.push(TextUtils::sized_text("Tiny text", Vec2::new(0.1, 0.8), "default", sizes::TINY));
        self.demo_texts.push(TextUtils::sized_text("Small text", Vec2::new(0.1, 0.75), "default", sizes::SMALL));
        self.demo_texts.push(TextUtils::sized_text("Normal text", Vec2::new(0.1, 0.7), "default", sizes::NORMAL));
        self.demo_texts.push(TextUtils::sized_text("Large text", Vec2::new(0.1, 0.6), "default", sizes::LARGE));
        self.demo_texts.push(TextUtils::sized_text("Huge text", Vec2::new(0.1, 0.45), "default", sizes::HUGE));
        self.demo_text_counts.push(self.demo_texts.len() - demo3_start);
        
        // Demo 4: Multiline text
        let demo4_start = self.demo_texts.len();
        self.demo_texts.push(TextUtils::title_text("Multiline Text Demo", Vec2::new(0.5, 0.9), "default"));
        self.demo_texts.push(TextUtils::multiline_text(
            "This is a multiline text example.\nIt demonstrates how text\ncan span multiple lines\nwith proper spacing.",
            Vec2::new(0.1, 0.8),
            "default",
            1.5
        ));
        self.demo_text_counts.push(self.demo_texts.len() - demo4_start);
        
        // Demo 5: Dynamic text
        let demo5_start = self.demo_texts.len();
        self.demo_texts.push(TextUtils::title_text("Dynamic Text Demo", Vec2::new(0.5, 0.9), "default"));
        self.demo_texts.push(TextUtils::info_text("Time elapsed:", Vec2::new(0.1, 0.8), "default"));
        // Store the index of the dynamic time label
        self.time_label_index = Some(self.demo_texts.len());
        self.demo_texts.push(TextUtils::info_text("0.00 seconds", Vec2::new(0.1, 0.75), "default"));
        self.demo_text_counts.push(self.demo_texts.len() - demo5_start);
    }
    
    fn get_current_demo_texts(&self) -> &[Text] {
        if self.current_demo >= self.demo_text_counts.len() {
            return &[];
        }
        
        // Calculate start index by summing up all previous demo counts
        let start: usize = self.demo_text_counts[..self.current_demo].iter().sum();
        let count = self.demo_text_counts[self.current_demo];
        let end = (start + count).min(self.demo_texts.len());
        
        if start >= self.demo_texts.len() {
            return &[];
        }
        
        &self.demo_texts[start..end]
    }
    
    fn get_demo_name(&self) -> &str {
        match self.current_demo {
            0 => "Basic Text Styles",
            1 => "Text Alignment",
            2 => "Font Sizes", 
            3 => "Multiline Text",
            4 => "Dynamic Text",
            _ => "Unknown Demo",
        }
    }
}

// Helper function to convert GLFW keys to our KeyCode enum
#[cfg(feature = "opengl")]
fn glfw_key_to_keycode(glfw_key: Key) -> Option<KeyCode> {
    match glfw_key {
        Key::A => Some(KeyCode::A),
        Key::B => Some(KeyCode::B),
        Key::C => Some(KeyCode::C),
        Key::D => Some(KeyCode::D),
        Key::E => Some(KeyCode::E),
        Key::F => Some(KeyCode::F),
        Key::G => Some(KeyCode::G),
        Key::H => Some(KeyCode::H),
        Key::I => Some(KeyCode::I),
        Key::J => Some(KeyCode::J),
        Key::K => Some(KeyCode::K),
        Key::L => Some(KeyCode::L),
        Key::M => Some(KeyCode::M),
        Key::N => Some(KeyCode::N),
        Key::O => Some(KeyCode::O),
        Key::P => Some(KeyCode::P),
        Key::Q => Some(KeyCode::Q),
        Key::R => Some(KeyCode::R),
        Key::S => Some(KeyCode::S),
        Key::T => Some(KeyCode::T),
        Key::U => Some(KeyCode::U),
        Key::V => Some(KeyCode::V),
        Key::W => Some(KeyCode::W),
        Key::X => Some(KeyCode::X),
        Key::Y => Some(KeyCode::Y),
        Key::Z => Some(KeyCode::Z),
        Key::Num0 => Some(KeyCode::Key0),
        Key::Num1 => Some(KeyCode::Key1),
        Key::Num2 => Some(KeyCode::Key2),
        Key::Num3 => Some(KeyCode::Key3),
        Key::Num4 => Some(KeyCode::Key4),
        Key::Num5 => Some(KeyCode::Key5),
        Key::Num6 => Some(KeyCode::Key6),
        Key::Num7 => Some(KeyCode::Key7),
        Key::Num8 => Some(KeyCode::Key8),
        Key::Num9 => Some(KeyCode::Key9),
        Key::F1 => Some(KeyCode::F1),
        Key::F2 => Some(KeyCode::F2),
        Key::F3 => Some(KeyCode::F3),
        Key::F4 => Some(KeyCode::F4),
        Key::F5 => Some(KeyCode::F5),
        Key::F6 => Some(KeyCode::F6),
        Key::F7 => Some(KeyCode::F7),
        Key::F8 => Some(KeyCode::F8),
        Key::F9 => Some(KeyCode::F9),
        Key::F10 => Some(KeyCode::F10),
        Key::F11 => Some(KeyCode::F11),
        Key::F12 => Some(KeyCode::F12),
        Key::Space => Some(KeyCode::Space),
        Key::Enter => Some(KeyCode::Enter),
        Key::Escape => Some(KeyCode::Escape),
        Key::Tab => Some(KeyCode::Tab),
        Key::Backspace => Some(KeyCode::Backspace),
        Key::Delete => Some(KeyCode::Delete),
        Key::LeftShift => Some(KeyCode::LeftShift),
        Key::RightShift => Some(KeyCode::RightShift),
        Key::LeftControl => Some(KeyCode::LeftCtrl),
        Key::RightControl => Some(KeyCode::RightCtrl),
        Key::LeftAlt => Some(KeyCode::LeftAlt),
        Key::RightAlt => Some(KeyCode::RightAlt),
        Key::LeftSuper => Some(KeyCode::LeftSuper),
        Key::RightSuper => Some(KeyCode::RightSuper),
        Key::Up => Some(KeyCode::Up),
        Key::Down => Some(KeyCode::Down),
        Key::Left => Some(KeyCode::Left),
        Key::Right => Some(KeyCode::Right),
        _ => None, // Unsupported keys
    }
}

#[cfg(feature = "opengl")]
impl engine_2d::animation::Animation for TextRenderingDemo {
    fn name(&self) -> &str {
        "Text Rendering Demo"
    }
    
    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Glfw(glfw::WindowEvent::Key(key, _scancode, action, _mods)) => {
                // Convert GLFW key to our KeyCode
                if let Some(key_code) = glfw_key_to_keycode(*key) {
                    let pressed = *action == Action::Press || *action == Action::Repeat;
                    self.input_manager.set_raw_input(PhysicalInput::Keyboard(key_code), pressed);
                }
            }
            _ => {
                // Handle other events if needed
            }
        }
    }
    
    fn update(&mut self, _sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32, delta_time: f32, _window_manager: Option<&mut WindowManager>, mut text_renderer: Option<&mut SimpleTextRenderer>) {
        // Update input
        self.input_manager.update(delta_time);
        
        // Update time
        self.time_elapsed = elapsed_time;
        
        // Setup demo texts on first run
        if let Some(ref mut tr) = text_renderer {
            if self.demo_texts.is_empty() {
                self.setup_demo_texts(tr.get_renderer_mut());
            }
            
            // Update dynamic text (demo 5)
            if self.current_demo == 4 {
                if let Some(time_index) = self.time_label_index {
                    if time_index < self.demo_texts.len() {
                        let time_text = format!("{:.2} seconds", self.time_elapsed);
                        self.demo_texts[time_index].set_content(time_text);
                    }
                }
            }
        }
        
        // Print instructions periodically (every 3 seconds)
        if elapsed_time - self.last_instruction_time >= 3.0 {
            println!("Text Demo Controls: SPACE=Next Demo, BACKSPACE=Previous Demo, ESC=Exit");
            println!("Current Demo: {}", self.get_demo_name());
            self.last_instruction_time = elapsed_time;
        }
        
        // Handle demo navigation
        if self.is_action_just_pressed("NEXT_DEMO") {
            self.current_demo = (self.current_demo + 1) % 5;
            println!("Switched to demo: {}", self.get_demo_name());
        }
        
        if self.is_action_just_pressed("PREV_DEMO") {
            self.current_demo = if self.current_demo == 0 { 4 } else { self.current_demo - 1 };
            println!("Switched to demo: {}", self.get_demo_name());
        }
        
        // Render current demo texts
        if let Some(ref mut tr) = text_renderer {
            for text in self.get_current_demo_texts() {
                if let Err(e) = tr.get_renderer_mut().render_text(text) {
                    eprintln!("Failed to render text: {}", e);
                }
            }
        }
    }
}

#[cfg(feature = "opengl")]
fn main() {
    env_logger::init();
    
    println!("Text Rendering Demo");
    println!("==================");
    println!("Controls:");
    println!("  SPACE     - Next Demo");
    println!("  BACKSPACE - Previous Demo");
    println!("  ESC       - Exit");
    println!();
    
    let config = EngineConfig {
        window_title: "Text Rendering Demo".to_string(),
        window_width: 1024,
        window_height: 768,
        target_fps: Some(60),
        show_fps: true,
        vsync: true,
        fullscreen: false,
    };
    
    let demo = TextRenderingDemo::new();
    
    let mut engine = match Engine::new_with_config_and_animation(config, Box::new(demo)) {
        Ok(engine) => engine,
        Err(e) => {
            eprintln!("Failed to create engine: {}", e);
            return;
        }
    };
    
    // Run the demo
    if let Err(e) = engine.run() {
        eprintln!("Engine error: {}", e);
    }
}

#[cfg(not(feature = "opengl"))]
fn main() {
    println!("This example requires the 'opengl' feature to be enabled.");
    println!("Run with: cargo run --example text_rendering_demo --features opengl");
}
