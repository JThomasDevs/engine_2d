/// Advanced Window Features Demo
/// 
/// This example demonstrates the new advanced window features including:
/// - VSync control
/// - Fullscreen toggle
/// - Window positioning
/// - Cursor management
/// - Window state management

use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::input::*;
#[cfg(feature = "opengl")]
use engine_2d::render::sprite::SpriteRenderer;
#[cfg(feature = "opengl")]
use engine_2d::engine::window::{WindowManager, WindowEvent};
#[cfg(feature = "opengl")]
use glfw::{Action, Key};

// Custom animation that demonstrates window features
#[cfg(feature = "opengl")]
struct AdvancedWindowDemo {
    input_manager: InputManager,
    last_instruction_time: f32,
    last_action_states: std::collections::HashMap<String, bool>,
    frame_times: Vec<f32>,
    max_frame_time_samples: usize,
    vsync_test_pattern: f32,
    last_frame_time: std::time::Instant,
}

#[cfg(feature = "opengl")]
impl AdvancedWindowDemo {
    fn new() -> Self {
        let mut input_manager = InputManager::new();
        
        // Define demo actions
        let demo_actions = vec![
            GameAction {
                id: "TOGGLE_FULLSCREEN".to_string(),
                display_name: "Toggle Fullscreen".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::F)),
                ],
                metadata: ActionMetadata {
                    description: Some("Toggle between windowed and fullscreen mode".to_string()),
                    tags: vec!["window".to_string(), "fullscreen".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "TOGGLE_VSYNC".to_string(),
                display_name: "Toggle VSync".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::V)),
                ],
                metadata: ActionMetadata {
                    description: Some("Toggle VSync on/off".to_string()),
                    tags: vec!["window".to_string(), "vsync".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "MINIMIZE_WINDOW".to_string(),
                display_name: "Minimize Window".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::M)),
                ],
                metadata: ActionMetadata {
                    description: Some("Minimize the window".to_string()),
                    tags: vec!["window".to_string(), "minimize".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "HIDE_CURSOR".to_string(),
                display_name: "Hide Cursor".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::H)),
                ],
                metadata: ActionMetadata {
                    description: Some("Hide/show cursor".to_string()),
                    tags: vec!["window".to_string(), "cursor".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "CAPTURE_MOUSE".to_string(),
                display_name: "Capture Mouse".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::C)),
                ],
                metadata: ActionMetadata {
                    description: Some("Capture/release mouse cursor".to_string()),
                    tags: vec!["window".to_string(), "mouse".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "CENTER_WINDOW".to_string(),
                display_name: "Center Window".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::T)),
                ],
                metadata: ActionMetadata {
                    description: Some("Center window on screen".to_string()),
                    tags: vec!["window".to_string(), "position".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "ANALYZE_VSYNC".to_string(),
                display_name: "Analyze VSync".to_string(),
                category: ActionCategory::UI,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::A)),
                ],
                metadata: ActionMetadata {
                    description: Some("Show detailed VSync analysis".to_string()),
                    tags: vec!["window".to_string(), "vsync".to_string(), "analysis".to_string()],
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
            frame_times: Vec::new(),
            max_frame_time_samples: 120, // Keep 2 seconds of samples at 60fps
            vsync_test_pattern: 0.0,
            last_frame_time: std::time::Instant::now(),
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
    
    fn track_frame_time(&mut self, delta_time: f32) {
        // Track frame times for VSync analysis
        self.frame_times.push(delta_time);
        if self.frame_times.len() > self.max_frame_time_samples {
            self.frame_times.remove(0);
        }
    }
    
    fn analyze_vsync_performance(&self) -> (f32, f32, f32, f32) {
        if self.frame_times.is_empty() {
            return (0.0, 0.0, 0.0, 0.0);
        }
        
        let avg_frame_time = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
        let min_frame_time = self.frame_times.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_frame_time = self.frame_times.iter().fold(0.0_f32, |a, &b| a.max(b));
        let avg_fps = 1.0 / avg_frame_time;
        
        (avg_frame_time, min_frame_time, max_frame_time, avg_fps)
    }
    
    fn get_vsync_analysis(&self) -> String {
        let (avg_frame_time, min_frame_time, max_frame_time, avg_fps) = self.analyze_vsync_performance();
        
        // VSync should result in frame times close to 16.67ms (60fps) or 8.33ms (120fps)
        let expected_60fps = 1.0 / 60.0; // ~16.67ms
        let expected_120fps = 1.0 / 120.0; // ~8.33ms
        
        let vsync_60fps_deviation = (avg_frame_time - expected_60fps).abs();
        let vsync_120fps_deviation = (avg_frame_time - expected_120fps).abs();
        
        let is_likely_vsynced = vsync_60fps_deviation < 0.002 || vsync_120fps_deviation < 0.002; // 2ms tolerance
        
        // Calculate frame time variance (VSync should have low variance)
        let variance = if self.frame_times.len() > 1 {
            let mean = avg_frame_time;
            let sum_squared_diff: f32 = self.frame_times.iter()
                .map(|&x| (x - mean).powi(2))
                .sum();
            sum_squared_diff / (self.frame_times.len() - 1) as f32
        } else {
            0.0
        };
        
        let frame_time_consistency = if variance < 0.0001 { "EXCELLENT" } 
                                   else if variance < 0.001 { "GOOD" }
                                   else { "POOR" };
        
        format!(
            "Frame Analysis:\n  Avg: {:.2}ms ({:.1}fps)\n  Min: {:.2}ms\n  Max: {:.2}ms\n  Variance: {:.6}\n  Consistency: {}\n  VSync: {}",
            avg_frame_time * 1000.0,
            avg_fps,
            min_frame_time * 1000.0,
            max_frame_time * 1000.0,
            variance,
            frame_time_consistency,
            if is_likely_vsynced { "LIKELY ON" } else { "LIKELY OFF" }
        )
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
impl engine_2d::animation::Animation for AdvancedWindowDemo {
    fn name(&self) -> &str {
        "Advanced Window Demo"
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
    
    fn update(&mut self, _sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32, delta_time: f32, window_manager: Option<&mut WindowManager>) {
        // Update input
        self.input_manager.update(delta_time);
        
        // Track frame times for VSync analysis
        self.track_frame_time(delta_time);
        
        // Update VSync test pattern for visual tearing detection
        self.vsync_test_pattern += delta_time * 2.0; // Rotate pattern
        
        // Print instructions and VSync analysis periodically (every 5 seconds)
        if elapsed_time - self.last_instruction_time >= 5.0 {
            println!("Window Demo Controls: F=Fullscreen, V=VSync, M=Minimize, H=Cursor, C=Mouse, T=Center, A=Analyze");
            println!("{}", self.get_vsync_analysis());
            self.last_instruction_time = elapsed_time;
        }
        
        // Check for action inputs and perform window operations
        if let Some(wm) = window_manager {
            if self.is_action_just_pressed("TOGGLE_FULLSCREEN") {
                println!("F pressed - Toggling fullscreen...");
                if let Err(e) = wm.toggle_fullscreen() {
                    eprintln!("Failed to toggle fullscreen: {}", e);
                }
            }
            if self.is_action_just_pressed("TOGGLE_VSYNC") {
                println!("V pressed - Toggling VSync...");
                let current_vsync = wm.is_vsync_enabled();
                if let Err(e) = wm.set_vsync(!current_vsync) {
                    eprintln!("Failed to toggle VSync: {}", e);
                } else {
                    println!("VSync is now: {}", if !current_vsync { "ENABLED" } else { "DISABLED" });
                    println!("Watch for frame time changes in the next analysis...");
                }
            }
            if self.is_action_just_pressed("MINIMIZE_WINDOW") {
                println!("M pressed - Minimizing window...");
                wm.minimize();
            }
            if self.is_action_just_pressed("HIDE_CURSOR") {
                println!("H pressed - Toggling cursor visibility...");
                wm.toggle_cursor();
            }
            if self.is_action_just_pressed("CAPTURE_MOUSE") {
                println!("C pressed - Toggling mouse capture...");
                wm.toggle_capture_mouse();
            }
            if self.is_action_just_pressed("CENTER_WINDOW") {
                println!("T pressed - Centering window...");
                if let Err(e) = wm.center_on_screen() {
                    eprintln!("Failed to center window: {}", e);
                }
            }
            if self.is_action_just_pressed("ANALYZE_VSYNC") {
                println!("A pressed - VSync Analysis:");
                println!("{}", self.get_vsync_analysis());
                println!("VSync Status: {}", if wm.is_vsync_enabled() { "ENABLED" } else { "DISABLED" });
            }
        }
    }
    
    
}

#[cfg(feature = "opengl")]
fn main() {
    env_logger::init();
    
    println!("Advanced Window Features Demo");
    println!("=============================");
    println!("Controls:");
    println!("  F   - Toggle Fullscreen");
    println!("  V   - Toggle VSync");
    println!("  M   - Minimize/Restore Window");
    println!("  H   - Hide/Show Cursor");
    println!("  C   - Capture/Release Mouse");
    println!("  T   - Center Window");
    println!("  A   - Analyze VSync Performance");
    println!("  ESC - Exit");
    println!();
    
    let config = EngineConfig {
        window_title: "Advanced Window Demo".to_string(),
        window_width: 1024,
        window_height: 768,
        target_fps: Some(60),
        show_fps: true,
        vsync: true,
        fullscreen: false,
    };
    
    let demo = AdvancedWindowDemo::new();
    
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
    println!("Run with: cargo run --example advanced_window_demo --features opengl");
}
