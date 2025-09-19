/// Complete input system demonstration
/// 
/// This example shows how all input sources (keyboard, mouse, gamepad) work together
/// with the InputManager to provide a unified input system.

use engine_2d::input::*;

fn main() {
    println!("🎮 Complete Input System Demo");
    println!("=============================");
    
    // Create input handlers
    let mut keyboard = KeyboardInput::new();
    let mut mouse = MouseInput::new();
    let mut gamepad = GamepadInput::new();
    let mut input_manager = InputManager::new();
    
    // Register predefined actions
    input_manager.register_actions(get_predefined_actions());
    
    println!("\n📋 Input System Components:");
    println!("---------------------------");
    println!("✅ Keyboard Input Handler");
    println!("✅ Mouse Input Handler");
    println!("✅ Gamepad Input Handler");
    println!("✅ Input Manager with {} actions", input_manager.get_actions().len());
    
    // Simulate some input events
    println!("\n🎮 Simulating Input Events:");
    println!("---------------------------");
    
    // Simulate keyboard input
    keyboard.handle_event(KeyboardEvent::KeyPress { key: KeyCode::W });
    keyboard.handle_event(KeyboardEvent::KeyPress { key: KeyCode::A });
    println!("• Pressed W and A keys");
    
    // Simulate mouse input
    mouse.handle_event(MouseEvent::Move { x: 100.0, y: 200.0 });
    mouse.handle_event(MouseEvent::ButtonPress { button: MouseButton::Left });
    mouse.handle_event(MouseEvent::Scroll { delta_x: 0.0, delta_y: 1.0 });
    println!("• Mouse moved to (100, 200)");
    println!("• Left mouse button pressed");
    println!("• Scroll wheel up");
    
    // Simulate gamepad input
    gamepad.handle_event(GamepadEvent::Connected { 
        id: 0, 
        name: "Xbox Controller".to_string() 
    });
    gamepad.handle_event(GamepadEvent::Button { 
        id: 0, 
        button: GamepadButton::A, 
        pressed: true 
    });
    gamepad.handle_event(GamepadEvent::Axis { 
        id: 0, 
        axis: GamepadAxis::LeftStickX, 
        value: 0.8 
    });
    println!("• Xbox Controller connected");
    println!("• A button pressed");
    println!("• Left stick moved right (0.8)");
    
    // Update all input systems
    println!("\n🔄 Updating Input Systems:");
    println!("--------------------------");
    
    let delta_time = 0.016; // 60 FPS
    keyboard.update(delta_time);
    mouse.update();
    gamepad.update();
    
    // Update InputManager with all input sources
    keyboard.update_input_manager(&mut input_manager);
    mouse.update_input_manager(&mut input_manager);
    gamepad.update_input_manager(&mut input_manager);
    
    input_manager.update(delta_time);
    
    println!("• All input systems updated");
    
    // Check input states
    println!("\n📊 Input State Analysis:");
    println!("------------------------");
    
    // Keyboard states
    println!("⌨️ Keyboard:");
    println!("  W key pressed: {}", keyboard.is_key_pressed(KeyCode::W));
    println!("  A key pressed: {}", keyboard.is_key_pressed(KeyCode::A));
    println!("  Escape pressed: {}", keyboard.is_key_pressed(KeyCode::Escape));
    
    // Mouse states
    println!("🖱️ Mouse:");
    let (mouse_x, mouse_y) = mouse.position();
    println!("  Position: ({:.1}, {:.1})", mouse_x, mouse_y);
    println!("  Left button: {}", mouse.is_button_pressed(MouseButton::Left));
    let (scroll_x, scroll_y) = mouse.scroll_delta();
    println!("  Scroll delta: ({:.1}, {:.1})", scroll_x, scroll_y);
    
    // Gamepad states
    println!("🎮 Gamepad:");
    if let Some(gamepad_state) = gamepad.primary_gamepad() {
        println!("  Connected: {}", gamepad_state.connected);
        println!("  Name: {}", gamepad_state.name);
        println!("  A button: {}", gamepad_state.is_button_pressed(GamepadButton::A));
        println!("  Left stick X: {:.2}", gamepad_state.get_axis(GamepadAxis::LeftStickX));
    } else {
        println!("  No gamepad connected");
    }
    
    // Action states through InputManager
    println!("\n🎯 Action States:");
    println!("-----------------");
    
    // Movement actions
    println!("🏃 Movement Actions:");
    println!("  MOVE_FORWARD: {}", input_manager.is_action_pressed("MOVE_FORWARD"));
    println!("  MOVE_LEFT: {}", input_manager.is_action_pressed("MOVE_LEFT"));
    println!("  MOUSE_LOOK_X: {:.2}", input_manager.get_action_value("MOUSE_LOOK_X"));
    println!("  MOUSE_LOOK_Y: {:.2}", input_manager.get_action_value("MOUSE_LOOK_Y"));
    
    // Combat actions
    println!("⚔️ Combat Actions:");
    println!("  FIRE_WEAPON: {}", input_manager.is_action_pressed("FIRE_WEAPON"));
    println!("  AIM_DOWN_SIGHTS: {}", input_manager.is_action_pressed("AIM_DOWN_SIGHTS"));
    
    // UI actions
    println!("🖥️ UI Actions:");
    println!("  PAUSE: {}", input_manager.is_action_pressed("PAUSE"));
    
    // Demonstrate context system
    println!("\n🎯 Context System Demo:");
    println!("----------------------");
    
    // Create a menu context that disables combat
    let menu_context = InputContext::new("menu".to_string(), 1)
        .disable_action("FIRE_WEAPON".to_string())
        .disable_action("AIM_DOWN_SIGHTS".to_string());
    
    input_manager.push_context(menu_context);
    println!("• Pushed menu context (disables combat)");
    
    println!("  FIRE_WEAPON enabled: {}", input_manager.is_action_enabled("FIRE_WEAPON"));
    println!("  MOVE_FORWARD enabled: {}", input_manager.is_action_enabled("MOVE_FORWARD"));
    
    // Pop context
    input_manager.pop_context();
    println!("• Popped menu context");
    
    println!("  FIRE_WEAPON enabled: {}", input_manager.is_action_enabled("FIRE_WEAPON"));
    
    // Demonstrate simultaneous input handling
    println!("\n🎮 Simultaneous Input Demo:");
    println!("---------------------------");
    
    // Simulate complex simultaneous input
    keyboard.handle_event(KeyboardEvent::KeyPress { key: KeyCode::W });
    keyboard.handle_event(KeyboardEvent::KeyPress { key: KeyCode::D });
    mouse.handle_event(MouseEvent::Move { x: 50.0, y: -30.0 });
    mouse.handle_event(MouseEvent::ButtonPress { button: MouseButton::Right });
    
    if let Some(gamepad_state) = gamepad.get_gamepad_mut(0) {
        gamepad_state.set_button(GamepadButton::RightTrigger, true);
        gamepad_state.set_axis(GamepadAxis::RightStickX, 0.6);
    }
    
    // Update systems
    keyboard.update(delta_time);
    mouse.update();
    gamepad.update();
    
    keyboard.update_input_manager(&mut input_manager);
    mouse.update_input_manager(&mut input_manager);
    gamepad.update_input_manager(&mut input_manager);
    
    input_manager.update(delta_time);
    
    println!("• Simultaneous input:");
    println!("  - W + D keys (diagonal movement)");
    println!("  - Mouse movement + right click (look + aim)");
    println!("  - Gamepad right trigger + right stick (shoot + look)");
    
    println!("\n📊 Simultaneous Action States:");
    println!("  MOVE_FORWARD: {}", input_manager.is_action_pressed("MOVE_FORWARD"));
    println!("  MOVE_RIGHT: {}", input_manager.is_action_pressed("MOVE_RIGHT"));
    println!("  MOUSE_LOOK_X: {:.2}", input_manager.get_action_value("MOUSE_LOOK_X"));
    println!("  MOUSE_LOOK_Y: {:.2}", input_manager.get_action_value("MOUSE_LOOK_Y"));
    println!("  AIM_DOWN_SIGHTS: {}", input_manager.is_action_pressed("AIM_DOWN_SIGHTS"));
    
    // Show input history
    println!("\n📜 Recent Input Events:");
    println!("----------------------");
    
    let recent_events = input_manager.get_recent_events(5);
    for (i, event) in recent_events.iter().enumerate() {
        match event {
            InputEvent::ActionTriggered { action_id, intensity, .. } => {
                println!("  {}. {} triggered (intensity: {:.2})", i + 1, action_id, intensity);
            }
            InputEvent::ContextChanged { old_context, new_context } => {
                println!("  {}. Context changed: {:?} -> {:?}", i + 1, old_context, new_context);
            }
            InputEvent::InputCombo { actions, duration } => {
                println!("  {}. Combo: {:?} (duration: {:?})", i + 1, actions, duration);
            }
        }
    }
    
    println!("\n🎉 Complete Input System Demo Finished!");
    println!("=====================================");
    println!("The system successfully handles:");
    println!("✅ Keyboard input with key states and repeat");
    println!("✅ Mouse input with position, buttons, and scroll");
    println!("✅ Gamepad input with buttons and analog sticks");
    println!("✅ Unified action system with rich metadata");
    println!("✅ Context-aware input processing");
    println!("✅ Simultaneous multi-type input handling");
    println!("✅ Event-driven architecture with history");
    println!("✅ Type-safe action definitions with macros");
}
