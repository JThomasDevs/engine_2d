/// Unit tests for the input system
/// 
/// These tests verify that the input system works correctly without requiring
/// a graphical window or user interaction.

use engine_2d::input::*;

#[test]
fn test_input_manager_creation() {
    let input_manager = InputManager::new();
    assert_eq!(input_manager.get_actions().len(), 0);
}

#[test]
fn test_action_registration() {
    let mut input_manager = InputManager::new();
    
    let action = GameAction {
        id: "TEST_ACTION".to_string(),
        display_name: "Test Action".to_string(),
        category: ActionCategory::Movement,
        input_type: InputType::Digital,
        default_bindings: vec![
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W)),
        ],
        metadata: ActionMetadata::default(),
    };
    
    input_manager.register_action(action);
    
    assert_eq!(input_manager.get_actions().len(), 1);
    assert!(input_manager.get_action("TEST_ACTION").is_some());
    assert!(input_manager.get_action("NONEXISTENT").is_none());
}

#[test]
fn test_action_categories() {
    let mut input_manager = InputManager::new();
    
    let movement_action = GameAction {
        id: "MOVE_UP".to_string(),
        display_name: "Move Up".to_string(),
        category: ActionCategory::Movement,
        input_type: InputType::Digital,
        default_bindings: vec![InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W))],
        metadata: ActionMetadata::default(),
    };
    
    let combat_action = GameAction {
        id: "FIRE_WEAPON".to_string(),
        display_name: "Fire Weapon".to_string(),
        category: ActionCategory::Combat,
        input_type: InputType::Digital,
        default_bindings: vec![InputBinding::Single(PhysicalInput::Mouse(MouseButton::Left))],
        metadata: ActionMetadata::default(),
    };
    
    input_manager.register_action(movement_action);
    input_manager.register_action(combat_action);
    
    let movement_actions = input_manager.get_actions_by_category(ActionCategory::Movement);
    let combat_actions = input_manager.get_actions_by_category(ActionCategory::Combat);
    
    assert_eq!(movement_actions.len(), 1);
    assert_eq!(combat_actions.len(), 1);
    assert_eq!(movement_actions[0].id, "MOVE_UP");
    assert_eq!(combat_actions[0].id, "FIRE_WEAPON");
}

#[test]
fn test_input_state_tracking() {
    let mut input_manager = InputManager::new();
    
    let action = GameAction {
        id: "TEST_ACTION".to_string(),
        display_name: "Test Action".to_string(),
        category: ActionCategory::Movement,
        input_type: InputType::Digital,
        default_bindings: vec![
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W)),
        ],
        metadata: ActionMetadata::default(),
    };
    
    input_manager.register_action(action);
    
    // Initially, action should not be pressed
    assert!(!input_manager.is_action_pressed("TEST_ACTION"));
    assert!(!input_manager.is_action_held("TEST_ACTION"));
    assert!(!input_manager.is_action_released("TEST_ACTION"));
    
    // Simulate key press with duplicate state sets (edge case test)
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::W), true);
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::W), true); // Duplicate set
    input_manager.update(0.016);
    
    // Action should now be pressed
    assert!(input_manager.is_action_pressed("TEST_ACTION"));
    assert!(!input_manager.is_action_held("TEST_ACTION"));
    
    // Update again (simulate holding)
    input_manager.update(0.016);
    
    // Action should now be held
    assert!(!input_manager.is_action_pressed("TEST_ACTION"));
    assert!(input_manager.is_action_held("TEST_ACTION"));
    
    // Simulate key release with duplicate state sets (edge case test)
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::W), false);
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::W), false); // Duplicate set
    input_manager.update(0.016);
    
    // Action should now be released
    assert!(!input_manager.is_action_pressed("TEST_ACTION"));
    assert!(!input_manager.is_action_held("TEST_ACTION"));
    assert!(input_manager.is_action_released("TEST_ACTION"));
}

#[test]
fn test_context_system() {
    let mut input_manager = InputManager::new();
    
    let action = GameAction {
        id: "TEST_ACTION".to_string(),
        display_name: "Test Action".to_string(),
        category: ActionCategory::Movement,
        input_type: InputType::Digital,
        default_bindings: vec![
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W)),
        ],
        metadata: ActionMetadata::default(),
    };
    
    input_manager.register_action(action);
    
    // Action should be enabled by default
    assert!(input_manager.is_action_enabled("TEST_ACTION"));
    
    // Create a context that disables the action
    let context = InputContext::new("test_context".to_string(), 1)
        .disable_action("TEST_ACTION".to_string());
    
    input_manager.push_context(context);
    
    // Action should now be disabled
    assert!(!input_manager.is_action_enabled("TEST_ACTION"));
    
    // Pop the context
    input_manager.pop_context();
    
    // Action should be enabled again
    assert!(input_manager.is_action_enabled("TEST_ACTION"));
}

#[test]
fn test_analog_input_values() {
    let mut input_manager = InputManager::new();
    
    let action = GameAction {
        id: "MOUSE_LOOK_X".to_string(),
        display_name: "Mouse Look X".to_string(),
        category: ActionCategory::Movement,
        input_type: InputType::Analog,
        default_bindings: vec![
            InputBinding::Analog {
                input: PhysicalInput::MouseAxis(MouseAxis::X),
                threshold: 0.1,
                deadzone: 0.05,
            },
        ],
        metadata: ActionMetadata::default(),
    };
    
    input_manager.register_action(action);
    
    // Set mouse axis value
    input_manager.set_physical_input_value(PhysicalInput::MouseAxis(MouseAxis::X), 0.5);
    input_manager.update(0.016);
    
    // Check action value (should be 1.0 because 0.5 > threshold of 0.1)
    let value = input_manager.get_action_value("MOUSE_LOOK_X");
    assert_eq!(value, 1.0);
    
    // Test deadzone
    input_manager.set_physical_input_value(PhysicalInput::MouseAxis(MouseAxis::X), 0.02);
    input_manager.update(0.016);
    
    let value = input_manager.get_action_value("MOUSE_LOOK_X");
    assert_eq!(value, 0.0); // Should be zero due to deadzone
}

#[test]
fn test_keyboard_input_handler() {
    let mut keyboard = KeyboardInput::new();
    
    // Initially no keys should be pressed
    assert!(!keyboard.is_key_pressed(KeyCode::W));
    assert!(!keyboard.is_key_just_pressed(KeyCode::W));
    assert!(!keyboard.is_key_just_released(KeyCode::W));
    
    // Simulate key press
    keyboard.handle_key_press(KeyCode::W);
    assert!(keyboard.is_key_pressed(KeyCode::W));
    assert!(keyboard.is_key_just_pressed(KeyCode::W));
    
    // Update to move to next frame
    keyboard.update(0.016);
    
    // Simulate the key still being held (in a real game loop, input events would be processed each frame)
    keyboard.handle_key_press(KeyCode::W);
    
    // Key should still be pressed but not just pressed
    assert!(keyboard.is_key_pressed(KeyCode::W));
    assert!(!keyboard.is_key_just_pressed(KeyCode::W));
    
    // Simulate key release
    keyboard.handle_key_release(KeyCode::W);
    assert!(!keyboard.is_key_pressed(KeyCode::W));
    assert!(keyboard.is_key_just_released(KeyCode::W));
}

#[test]
fn test_mouse_input_handler() {
    let mut mouse = MouseInput::new();
    
    // Initially no buttons should be pressed
    assert!(!mouse.is_button_pressed(MouseButton::Left));
    assert!(!mouse.is_button_just_pressed(MouseButton::Left));
    assert!(!mouse.is_button_just_released(MouseButton::Left));
    
    // Simulate button press
    mouse.handle_button_press(MouseButton::Left);
    assert!(mouse.is_button_pressed(MouseButton::Left));
    assert!(mouse.is_button_just_pressed(MouseButton::Left));
    
    // Update to move to next frame
    mouse.update();
    
    // Simulate the button still being held (in a real game loop, input events would be processed each frame)
    mouse.handle_button_press(MouseButton::Left);
    
    // Button should still be pressed but not just pressed
    assert!(mouse.is_button_pressed(MouseButton::Left));
    assert!(!mouse.is_button_just_pressed(MouseButton::Left));
    
    // Simulate button release
    mouse.handle_button_release(MouseButton::Left);
    assert!(!mouse.is_button_pressed(MouseButton::Left));
    assert!(mouse.is_button_just_released(MouseButton::Left));
}

#[test]
fn test_mouse_position_tracking() {
    let mut mouse = MouseInput::new();
    
    // Initial position should be (0, 0)
    assert_eq!(mouse.position(), (0.0, 0.0));
    assert_eq!(mouse.position_delta(), (0.0, 0.0));
    
    // Simulate mouse movement
    mouse.handle_mouse_move(100.0, 200.0);
    assert_eq!(mouse.position(), (100.0, 200.0));
    
    // Update to capture previous position
    mouse.update();
    
    // Simulate more movement
    mouse.handle_mouse_move(150.0, 250.0);
    assert_eq!(mouse.position(), (150.0, 250.0));
    assert_eq!(mouse.position_delta(), (50.0, 50.0));
}

#[test]
fn test_mouse_scroll_tracking() {
    let mut mouse = MouseInput::new();
    
    // Initial scroll delta should be (0, 0)
    assert_eq!(mouse.scroll_delta(), (0.0, 0.0));
    
    // Simulate scroll
    mouse.handle_scroll(0.0, 1.0);
    assert_eq!(mouse.scroll_delta(), (0.0, 1.0));
    
    // Simulate more scroll
    mouse.handle_scroll(0.0, 1.0);
    assert_eq!(mouse.scroll_delta(), (0.0, 2.0));
    
    // Update should reset scroll delta
    mouse.update();
    assert_eq!(mouse.scroll_delta(), (0.0, 0.0));
}

#[test]
fn test_gamepad_input_handler() {
    let mut gamepad = GamepadInput::new();
    
    // Initially no gamepads should be connected
    assert!(gamepad.primary_gamepad().is_none());
    
    // Simulate gamepad connection
    gamepad.handle_event(GamepadEvent::Connected {
        id: 0,
        name: "Test Controller".to_string(),
    });
    
    // Should now have a primary gamepad
    assert!(gamepad.primary_gamepad().is_some());
    
    if let Some(gamepad_state) = gamepad.primary_gamepad() {
        assert_eq!(gamepad_state.name, "Test Controller");
        assert!(gamepad_state.connected);
    }
    
    // Simulate button press
    gamepad.handle_event(GamepadEvent::Button {
        id: 0,
        button: GamepadButton::A,
        pressed: true,
    });
    
    if let Some(gamepad_state) = gamepad.primary_gamepad() {
        assert!(gamepad_state.is_button_pressed(GamepadButton::A));
    }
    
    // Simulate axis movement
    gamepad.handle_event(GamepadEvent::Axis {
        id: 0,
        axis: GamepadAxis::LeftStickX,
        value: 0.8,
    });
    
    if let Some(gamepad_state) = gamepad.primary_gamepad() {
        assert_eq!(gamepad_state.get_axis(GamepadAxis::LeftStickX), 0.8);
    }
}

#[test]
fn test_predefined_actions() {
    let actions = get_predefined_actions();
    
    // Should have some predefined actions
    assert!(!actions.is_empty());
    
    // Check that we have actions in different categories
    let movement_actions: Vec<_> = actions.iter()
        .filter(|action| matches!(action.category, ActionCategory::Movement))
        .collect();
    
    let combat_actions: Vec<_> = actions.iter()
        .filter(|action| matches!(action.category, ActionCategory::Combat))
        .collect();
    
    let ui_actions: Vec<_> = actions.iter()
        .filter(|action| matches!(action.category, ActionCategory::UI))
        .collect();
    
    assert!(!movement_actions.is_empty());
    assert!(!combat_actions.is_empty());
    assert!(!ui_actions.is_empty());
    
    // Check that actions have proper bindings
    for action in &actions {
        assert!(!action.default_bindings.is_empty());
        assert!(!action.id.is_empty());
        assert!(!action.display_name.is_empty());
    }
}

#[test]
fn test_input_event_generation() {
    let mut input_manager = InputManager::new();
    
    let action = GameAction {
        id: "TEST_ACTION".to_string(),
        display_name: "Test Action".to_string(),
        category: ActionCategory::Movement,
        input_type: InputType::Digital,
        default_bindings: vec![
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W)),
        ],
        metadata: ActionMetadata::default(),
    };
    
    input_manager.register_action(action);
    
    // Initially no events
    assert_eq!(input_manager.get_recent_events(10).len(), 0);
    
    // Simulate key press and update
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::W), true);
    input_manager.update(0.016);
    
    // Should have generated an event
    let events = input_manager.get_recent_events(10);
    assert!(!events.is_empty());
    
    if let Some(event) = events.first() {
        match event {
            InputEvent::ActionTriggered { action_id, intensity, .. } => {
                assert_eq!(action_id, "TEST_ACTION");
                assert_eq!(*intensity, 1.0);
            }
            _ => panic!("Expected ActionTriggered event"),
        }
    }
}

#[test]
fn test_simultaneous_input_handling() {
    let mut input_manager = InputManager::new();
    
    let actions = vec![
        GameAction {
            id: "MOVE_UP".to_string(),
            display_name: "Move Up".to_string(),
            category: ActionCategory::Movement,
            input_type: InputType::Digital,
            default_bindings: vec![InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W))],
            metadata: ActionMetadata::default(),
        },
        GameAction {
            id: "MOVE_LEFT".to_string(),
            display_name: "Move Left".to_string(),
            category: ActionCategory::Movement,
            input_type: InputType::Digital,
            default_bindings: vec![InputBinding::Single(PhysicalInput::Keyboard(KeyCode::A))],
            metadata: ActionMetadata::default(),
        },
    ];
    
    input_manager.register_actions(actions);
    
    // Simulate simultaneous key presses
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::W), true);
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::A), true);
    input_manager.update(0.016);
    
    // Both actions should be active
    assert!(input_manager.is_action_pressed("MOVE_UP"));
    assert!(input_manager.is_action_pressed("MOVE_LEFT"));
    
    // Release one key
    input_manager.set_physical_input_state(PhysicalInput::Keyboard(KeyCode::A), false);
    input_manager.update(0.016);
    
    // Only one action should be active
    assert!(input_manager.is_action_held("MOVE_UP"));
    assert!(input_manager.is_action_released("MOVE_LEFT"));
}
