#[cfg(feature = "opengl")]
use engine_2d::engine::window::WindowEvent;
/// Interactive sprite movement demo
///
/// This example demonstrates the input system by allowing you to move a sprite
/// around the screen using WASD or arrow keys.
use engine_2d::engine::{Engine, EngineConfig};
use engine_2d::input::*;
#[cfg(feature = "opengl")]
use engine_2d::render::sprite::{Sprite, SpriteRenderer};
#[cfg(feature = "opengl")]
use engine_2d::render::texture::TextureId;
#[cfg(feature = "opengl")]
use glam::Vec2;
#[cfg(feature = "opengl")]
use glfw::{Action, Key};
#[cfg(feature = "opengl")]
use std::sync::atomic::{AtomicU32, Ordering};

// Custom animation that handles sprite movement
#[cfg(feature = "opengl")]
struct SpriteMovementAnimation {
    sprite: Sprite,
    input_manager: InputManager,
    move_speed: f32,
    texture_created: bool,
}

#[cfg(feature = "opengl")]
impl SpriteMovementAnimation {
    fn new() -> Self {
        let mut input_manager = InputManager::new();

        // Define movement actions
        let movement_actions = vec![
            GameAction {
                id: "MOVE_UP".to_string(),
                display_name: "Move Up".to_string(),
                category: ActionCategory::Movement,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W)),
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Up)),
                ],
                metadata: ActionMetadata {
                    description: Some("Move sprite upward".to_string()),
                    tags: vec!["movement".to_string(), "up".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "MOVE_DOWN".to_string(),
                display_name: "Move Down".to_string(),
                category: ActionCategory::Movement,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::S)),
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Down)),
                ],
                metadata: ActionMetadata {
                    description: Some("Move sprite downward".to_string()),
                    tags: vec!["movement".to_string(), "down".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "MOVE_LEFT".to_string(),
                display_name: "Move Left".to_string(),
                category: ActionCategory::Movement,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::A)),
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Left)),
                ],
                metadata: ActionMetadata {
                    description: Some("Move sprite leftward".to_string()),
                    tags: vec!["movement".to_string(), "left".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
            GameAction {
                id: "MOVE_RIGHT".to_string(),
                display_name: "Move Right".to_string(),
                category: ActionCategory::Movement,
                input_type: InputType::Digital,
                default_bindings: vec![
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::D)),
                    InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Right)),
                ],
                metadata: ActionMetadata {
                    description: Some("Move sprite rightward".to_string()),
                    tags: vec!["movement".to_string(), "right".to_string()],
                    priority: 1,
                    context_required: None,
                },
            },
        ];

        input_manager.register_actions(movement_actions);

        // Create a simple colored sprite (red square)
        let sprite = Sprite::new(
            TextureId(0),        // We'll create a texture in the animation
            Vec2::new(0.0, 0.0), // Start at origin (will be moved by input)
            Vec2::new(0.1, 0.1), // Size as fraction of screen (10% x 10%)
        );

        Self {
            sprite,
            input_manager,
            move_speed: 0.5, // units per second
            texture_created: false,
        }
    }
}

#[cfg(feature = "opengl")]
impl engine_2d::animation::Animation for SpriteMovementAnimation {
    fn update(
        &mut self,
        sprite_renderer: Option<&mut SpriteRenderer>,
        elapsed_time: f32,
        delta_time: f32,
    ) {
        // Initialize sprite renderer if we have one
        if let Some(renderer) = sprite_renderer {
            if !self.texture_created {
                // Create a red texture for our sprite (only once)
                match renderer
                    .texture_manager()
                    .create_color_texture(64, 64, (255, 0, 0, 255))
                {
                    Ok(texture_id) => {
                        self.sprite.texture_id = texture_id;
                        println!("✅ Created red texture for sprite");
                        self.texture_created = true;
                    }
                    Err(e) => {
                        println!("⚠️  Failed to create texture: {}", e);
                    }
                }
            }

            // Update input manager
            self.input_manager.update(elapsed_time);

            // Handle movement
            let mut velocity = Vec2::ZERO;

            if self.input_manager.is_action_held("MOVE_UP") {
                velocity.y += self.move_speed * delta_time;
            }
            if self.input_manager.is_action_held("MOVE_DOWN") {
                velocity.y -= self.move_speed * delta_time;
            }
            if self.input_manager.is_action_held("MOVE_LEFT") {
                velocity.x -= self.move_speed * delta_time;
            }
            if self.input_manager.is_action_held("MOVE_RIGHT") {
                velocity.x += self.move_speed * delta_time;
            }

            // Update sprite position
            self.sprite.position += velocity;

            // Keep sprite within screen bounds (normalized coordinates -1 to 1)
            self.sprite.position.x = self.sprite.position.x.max(-0.9).min(0.9);
            self.sprite.position.y = self.sprite.position.y.max(-0.9).min(0.9);

            // Render the sprite
            if let Err(e) = renderer.render_sprite(&self.sprite) {
                println!("⚠️  Failed to render sprite: {}", e);
            } else {
                // Debug: Print sprite position occasionally
                static FRAME_COUNT: AtomicU32 = AtomicU32::new(0);
                let count = FRAME_COUNT.fetch_add(1, Ordering::Relaxed);
                if count % 60 == 0 {
                    // Every 60 frames (about 1 second at 60fps)
                    println!(
                        "Sprite position: ({:.2}, {:.2})",
                        self.sprite.position.x, self.sprite.position.y
                    );
                }
            }
        }
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Glfw(glfw::WindowEvent::Key(key, _scancode, action, _modifiers)) => {
                // Convert GLFW key to our KeyCode
                let key_code = match key {
                    Key::W => KeyCode::W,
                    Key::A => KeyCode::A,
                    Key::S => KeyCode::S,
                    Key::D => KeyCode::D,
                    Key::Up => KeyCode::Up,
                    Key::Down => KeyCode::Down,
                    Key::Left => KeyCode::Left,
                    Key::Right => KeyCode::Right,
                    _ => return, // Ignore other keys
                };

                // Convert GLFW action to our input state
                match action {
                    Action::Press => {
                        self.input_manager
                            .set_raw_input(PhysicalInput::Keyboard(key_code), true);
                    }
                    Action::Release => {
                        self.input_manager
                            .set_raw_input(PhysicalInput::Keyboard(key_code), false);
                    }
                    Action::Repeat => {
                        // For repeat, we'll treat it as a press
                        self.input_manager
                            .set_raw_input(PhysicalInput::Keyboard(key_code), true);
                    }
                }
            }
            _ => {
                // Ignore other events
            }
        }
    }

    fn name(&self) -> &str {
        "Sprite Movement Demo"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(feature = "opengl"))]
    {
        eprintln!("❌ Error: This demo requires the 'opengl' feature to be enabled.");
        eprintln!("   Please run with: cargo run --example sprite_movement_demo --features opengl");
        std::process::exit(1);
    }

    #[cfg(feature = "opengl")]
    {
        println!("🎮 Sprite Movement Demo");
        println!("=======================");
        println!("Use WASD or Arrow Keys to move the red sprite");
        println!("Press ESC to quit");

        // Create engine configuration
        let config = EngineConfig {
            window_title: "Sprite Movement Demo".to_string(),
            window_width: 800,
            window_height: 600,
            target_fps: Some(60),
            show_fps: true,
            vsync: true,
            fullscreen: false,
        };

        // Create the animation
        let animation = Box::new(SpriteMovementAnimation::new());

        // Create the engine with our custom animation
        let mut engine = Engine::new_with_config_and_animation(config, animation)?;

        println!("✅ Engine created successfully!");
        println!("✅ Input system initialized with WASD/Arrow key bindings");
        println!("✅ Sprite movement animation loaded");

        // Run the engine
        engine.run()?;

        println!("Demo finished!");
        Ok(())
    }
}
