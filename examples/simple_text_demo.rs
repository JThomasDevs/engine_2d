#[cfg(feature = "opengl")]
use engine_2d::engine::{Engine, EngineConfig};
#[cfg(feature = "opengl")]
use engine_2d::input::*;
#[cfg(feature = "opengl")]
use engine_2d::render::simple_text::{SimpleText, SimpleTextRenderer, TextAnchor};
#[cfg(feature = "opengl")]
use engine_2d::animation::Animation;
#[cfg(feature = "opengl")]
use engine_2d::render::sprite::SpriteRenderer;
#[cfg(feature = "opengl")]
use engine_2d::engine::window::WindowManager;
#[cfg(feature = "opengl")]
use glam::Vec2;
#[cfg(feature = "opengl")]
use std::collections::HashMap;
#[cfg(feature = "opengl")]
use glfw::Key;

// Helper function to convert GLFW keys to our KeyCode
#[cfg(feature = "opengl")]
fn glfw_key_to_keycode(key: Key) -> Option<KeyCode> {
    match key {
        Key::Space => Some(KeyCode::Space),
        Key::Backspace => Some(KeyCode::Backspace),
        Key::Escape => Some(KeyCode::Escape),
        Key::Enter => Some(KeyCode::Enter),
        Key::Tab => Some(KeyCode::Tab),
        Key::Left => Some(KeyCode::Left),
        Key::Right => Some(KeyCode::Right),
        Key::Up => Some(KeyCode::Up),
        Key::Down => Some(KeyCode::Down),
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
        _ => None, // Unsupported keys
    }
}

#[cfg(feature = "opengl")]
struct SimpleTextShowcase {
    input_manager: InputManager,
    last_action_states: HashMap<String, bool>,
    current_demo: usize,
    demos: Vec<&'static str>,
    // Store SimpleText objects for each demo
    demo_texts: Vec<Vec<SimpleText>>,
    // Mouse position for coordinate display
    mouse_position: Vec2,
}

#[cfg(feature = "opengl")]
impl SimpleTextShowcase {
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
                    description: Some("Switch to next demo".to_string()),
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
                    description: Some("Switch to previous demo".to_string()),
                    tags: vec!["demo".to_string(), "navigation".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "EXIT".to_string(),
                display_name: "Exit".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Escape)),
                ],
                metadata: ActionMetadata {
                    description: Some("Exit the demo".to_string()),
                    tags: vec!["demo".to_string(), "navigation".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
        ];
        
        input_manager.register_actions(demo_actions);

        let mut showcase = Self {
            input_manager,
            last_action_states: HashMap::new(),
            current_demo: 0,
            demos: vec![
                "Anchor Positioning",
                "Font Sizes", 
                "Colors",
                "Fluent API",
            ],
            demo_texts: Vec::new(),
            mouse_position: Vec2::new(0.0, 0.0),
        };
        
        // Initialize all demo texts
        showcase.setup_all_demo_texts();
        
        showcase
    }
    
    fn is_action_just_pressed(&mut self, action_id: &str) -> bool {
        let current_pressed = self.input_manager.is_action_held(action_id);
        let last_pressed = self.last_action_states.get(action_id).copied().unwrap_or(false);
        
        // Update the last state
        self.last_action_states.insert(action_id.to_string(), current_pressed);
        
        // Return true if currently pressed but wasn't pressed last frame
        current_pressed && !last_pressed
    }
    
    fn setup_all_demo_texts(&mut self) {
        self.demo_texts.clear();
        
        // Demo 1: Anchor-Based Positioning
        self.demo_texts.push(vec![
            SimpleText::new("Top Left".to_string(), 18)
                .anchor(TextAnchor::TopLeft)
                .position(Vec2::new(10.0, -60.0)), // Moved down to avoid title
            SimpleText::new("Top Center".to_string(), 18)
                .anchor(TextAnchor::TopCenter)
                .position(Vec2::new(0.0, -60.0)), // Moved down to avoid title
            SimpleText::new("Top Right".to_string(), 18)
                .anchor(TextAnchor::TopRight)
                .position(Vec2::new(-10.0, -60.0)), // Moved down to avoid title
            SimpleText::new("Center".to_string(), 20)
                .anchor(TextAnchor::MiddleCenter)
                .position(Vec2::new(0.0, 0.0)), // Exactly center
            SimpleText::new("Bottom Left".to_string(), 18)
                .anchor(TextAnchor::BottomLeft)
                .position(Vec2::new(10.0, 60.0)), // Moved up to avoid controls
            SimpleText::new("Bottom Center".to_string(), 18)
                .anchor(TextAnchor::BottomCenter)
                .position(Vec2::new(0.0, 60.0)), // Moved up to avoid controls
            SimpleText::new("Bottom Right".to_string(), 18)
                .anchor(TextAnchor::BottomRight)
                .position(Vec2::new(-10.0, 60.0)), // Moved up to avoid controls
        ]);
        
        // Demo 2: Font Sizes with Anchors - More dramatic size differences
        self.demo_texts.push(vec![
            SimpleText::new("Tiny Text".to_string(), 10)
                .anchor(TextAnchor::MiddleLeft)
                .position(Vec2::new(50.0, 80.0)), // Moved right to avoid clipping
            SimpleText::new("Small Text".to_string(), 16)
                .anchor(TextAnchor::MiddleLeft)
                .position(Vec2::new(50.0, 40.0)), // Moved right to avoid clipping
            SimpleText::new("Normal Text".to_string(), 24)
                .anchor(TextAnchor::MiddleLeft)
                .position(Vec2::new(50.0, 0.0)), // Moved right to avoid clipping
            SimpleText::new("Large Text".to_string(), 36)
                .anchor(TextAnchor::MiddleLeft)
                .position(Vec2::new(50.0, -40.0)), // Moved right to avoid clipping
            SimpleText::new("Huge Text".to_string(), 48)
                .anchor(TextAnchor::MiddleLeft)
                .position(Vec2::new(50.0, -80.0)), // Moved right to avoid clipping
        ]);
        
        // Demo 3: Colors with Anchors
        self.demo_texts.push(vec![
            SimpleText::new("Red Text".to_string(), 16)
                .color((1.0, 0.0, 0.0))
                .anchor(TextAnchor::MiddleRight)
                .position(Vec2::new(-50.0, 60.0)),
            SimpleText::new("Green Text".to_string(), 16)
                .color((0.0, 1.0, 0.0))
                .anchor(TextAnchor::MiddleRight)
                .position(Vec2::new(-50.0, 20.0)),
            SimpleText::new("Blue Text".to_string(), 16)
                .color((0.0, 0.0, 1.0))
                .anchor(TextAnchor::MiddleRight)
                .position(Vec2::new(-50.0, -20.0)),
            SimpleText::new("Yellow Text".to_string(), 16)
                .color((1.0, 1.0, 0.0))
                .anchor(TextAnchor::MiddleRight)
                .position(Vec2::new(-50.0, -60.0)),
            SimpleText::new("Purple Text".to_string(), 16)
                .color((1.0, 0.0, 1.0))
                .anchor(TextAnchor::MiddleRight)
                .position(Vec2::new(-50.0, -100.0)),
        ]);
        
        // Demo 4: Fluent API with Anchors
        self.demo_texts.push(vec![
            SimpleText::new("Fluent API Demo".to_string(), 18)
                .color((1.0, 1.0, 0.0))
                .anchor(TextAnchor::TopCenter)
                .position(Vec2::new(0.0, -100.0)),
            SimpleText::new("Method Chaining".to_string(), 16)
                .color((0.0, 1.0, 1.0))
                .anchor(TextAnchor::TopCenter)
                .position(Vec2::new(0.0, -140.0)),
            SimpleText::new("Clean & Simple".to_string(), 14)
                .color((1.0, 0.5, 0.0))
                .anchor(TextAnchor::TopCenter)
                .position(Vec2::new(0.0, -180.0)),
        ]);
        
    }
    
}

#[cfg(feature = "opengl")]
impl Animation for SimpleTextShowcase {
    fn name(&self) -> &str {
        "SimpleText Showcase"
    }
    
    fn handle_event(&mut self, event: &engine_2d::engine::window::WindowEvent) {
        match event {
            engine_2d::engine::window::WindowEvent::Glfw(glfw::WindowEvent::Key(key, _scancode, action, _mods)) => {
                // Convert GLFW key to our KeyCode
                if let Some(key_code) = glfw_key_to_keycode(*key) {
                    let pressed = *action == glfw::Action::Press || *action == glfw::Action::Repeat;
                    self.input_manager.set_raw_input(PhysicalInput::Keyboard(key_code), pressed);
                }
            }
            _ => {
                // Handle other events if needed
            }
        }
    }

    
    fn update(&mut self, _sprite_renderer: Option<&mut SpriteRenderer>, _elapsed_time: f32, delta_time: f32, window_manager: Option<&mut WindowManager>, text_renderer: Option<&mut SimpleTextRenderer>) {
        
        // Handle input
        self.input_manager.update(delta_time);
        
        // Process mouse events from the event system
        if let Some(ref window_manager) = window_manager {
            if let Some(event_system) = window_manager.get_event_system() {
                // Process all pending input events
                while let Some(input_event) = event_system.receive_input_event() {
                    match input_event {
                        engine_2d::events::event_types::InputEvent::MouseMove { x, y, .. } => {
                            self.mouse_position = Vec2::new(x, y);
                        }
                        _ => {} // Ignore other input events
                    }
                }
            }
        }
        
        // Check for demo switching
        if self.is_action_just_pressed("NEXT_DEMO") {
            self.current_demo = (self.current_demo + 1) % self.demos.len();
            println!("Switched to demo: {}", self.demos[self.current_demo]);
        }
        
        if self.is_action_just_pressed("PREV_DEMO") {
            self.current_demo = if self.current_demo == 0 { 
                self.demos.len() - 1 
            } else { 
                self.current_demo - 1 
            };
            println!("Switched to demo: {}", self.demos[self.current_demo]);
        }
        
        if self.is_action_just_pressed("EXIT") {
            println!("Exiting demo...");
            if let Some(window_manager) = window_manager {
                window_manager.request_close();
            }
            return;
        }
        
        // Render text
        if let Some(tr) = text_renderer {
            // Render the SimpleText objects for current demo
            if let Some(current_texts) = self.demo_texts.get(self.current_demo) {
                for simple_text in current_texts {
                    let _ = tr.render(simple_text);
                }
            }
            
            // Add mouse coordinate display for Demo 1 (Anchor Positioning)
            if self.current_demo == 0 {
                let mouse_coords_text = format!("Mouse: ({:.0}, {:.0})", self.mouse_position.x, self.mouse_position.y);
                let mouse_display = SimpleText::new(mouse_coords_text, 14)
                    .color((0.0, 1.0, 1.0)) // Cyan color
                    .anchor(TextAnchor::TopRight)
                    .position(Vec2::new(-20.0, -40.0)) // Position in top-right, avoiding other text
                    .align(engine_2d::render::text::TextAlign::Right);
                let _ = tr.render(&mouse_display);
            }
            
            // Show demo info using SimpleText
            let demo_info = format!("Demo {} of {}: {}", 
                self.current_demo + 1, 
                self.demos.len(), 
                self.demos[self.current_demo]
            );
            
            // Position demo title and controls to avoid overlap with demo content
            let demo_text = SimpleText::new(demo_info, 14)
                .anchor(TextAnchor::TopCenter)
                .position(Vec2::new(0.0, -20.0))
                .align(engine_2d::render::text::TextAlign::Center);
            let _ = tr.render(&demo_text);
            
            // Show controls using SimpleText - position at bottom center to avoid demo content
            let controls_text = SimpleText::new("SPACE=Next | BACKSPACE=Prev | ESC=Exit".to_string(), 12)
                .anchor(TextAnchor::BottomCenter)
                .position(Vec2::new(0.0, 20.0))
                .align(engine_2d::render::text::TextAlign::Center);
            let _ = tr.render(&controls_text);
        }
    }
}

#[cfg(feature = "opengl")]
fn main() {
    println!("SimpleText Showcase");
    println!("===================");
    println!("This demo showcases the new SimpleText system:");
    println!("- Clean, simple API with fluent method chaining");
    println!("- No classifications (title_text, subtitle_text, etc.)");
    println!("- User controls size and purpose");
    println!("- Perfect for text editor functionality");
    println!();
    println!("Controls:");
    println!("  SPACE     - Next Demo");
    println!("  BACKSPACE - Previous Demo");
    println!("  ESC       - Exit");
    
    let config = EngineConfig {
        window_title: "SimpleText Showcase".to_string(),
        window_width: 1024,
        window_height: 768,
        target_fps: Some(60),
        show_fps: true,
        vsync: true,
        fullscreen: false,
        viewport: engine_2d::engine::config::ViewportConfig::with_bounds(0.0, 1024.0, 0.0, 768.0),
        fallback_font_path: "assets/fonts/default.ttf".to_string(),
    };
    
    let animation = SimpleTextShowcase::new();
    
    let mut engine = Engine::new_with_config_and_animation(config, Box::new(animation)).expect("Failed to create engine");
            
            if let Err(e) = engine.run() {
                eprintln!("Engine error: {}", e);
            }
        }

#[cfg(not(feature = "opengl"))]
fn main() {
    println!("This example requires the 'opengl' feature to be enabled.");
    println!("Run with: cargo run --example simple_text_showcase --features opengl");
}
