/// Example game actions using the input system
/// 
/// This file demonstrates how to define game actions using the macro system.
/// In a real game, you would define your own actions here.

use crate::input::types::*;
use crate::define_actions;

// Define common game actions
define_actions! {
    // Movement actions
    MOVE_FORWARD: {
        name: "Move Forward",
        category: Movement,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::W)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::DPadUp))
        ],
        description: "Move the player forward",
        tags: ["movement", "basic"],
        priority: 1,
    };
    
    MOVE_BACKWARD: {
        name: "Move Backward",
        category: Movement,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::S)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::DPadDown))
        ],
        description: "Move the player backward",
        tags: ["movement", "basic"],
        priority: 1,
    };
    
    MOVE_LEFT: {
        name: "Move Left",
        category: Movement,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::A)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::DPadLeft))
        ],
        description: "Move the player left",
        tags: ["movement", "basic"],
        priority: 1,
    };
    
    MOVE_RIGHT: {
        name: "Move Right",
        category: Movement,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::D)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::DPadRight))
        ],
        description: "Move the player right",
        tags: ["movement", "basic"],
        priority: 1,
    };
    
    // Mouse look actions
    MOUSE_LOOK_X: {
        name: "Mouse Look X",
        category: Movement,
        input_type: Analog,
        bindings: [
            InputBinding::Analog {
                input: PhysicalInput::MouseAxis(MouseAxis::X),
                threshold: 0.1,
                deadzone: 0.05,
            }
        ],
        description: "Horizontal mouse look",
        tags: ["camera", "analog"],
        priority: 2,
    };
    
    MOUSE_LOOK_Y: {
        name: "Mouse Look Y",
        category: Movement,
        input_type: Analog,
        bindings: [
            InputBinding::Analog {
                input: PhysicalInput::MouseAxis(MouseAxis::Y),
                threshold: 0.1,
                deadzone: 0.05,
            }
        ],
        description: "Vertical mouse look",
        tags: ["camera", "analog"],
        priority: 2,
    };
    
    // Combat actions
    FIRE_WEAPON: {
        name: "Fire Weapon",
        category: Combat,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Mouse(MouseButton::Left)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::RightTrigger))
        ],
        description: "Fire the currently equipped weapon",
        tags: ["combat", "weapon"],
        priority: 1,
    };
    
    AIM_DOWN_SIGHTS: {
        name: "Aim Down Sights",
        category: Combat,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Mouse(MouseButton::Right)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::LeftTrigger))
        ],
        description: "Aim down sights for better accuracy",
        tags: ["combat", "weapon"],
        priority: 1,
    };
    
    RELOAD: {
        name: "Reload",
        category: Combat,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::R)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::X))
        ],
        description: "Reload the current weapon",
        tags: ["combat", "weapon"],
        priority: 1,
    };
    
    // UI actions
    PAUSE: {
        name: "Pause Game",
        category: UI,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::Escape)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::Start))
        ],
        description: "Pause or unpause the game",
        tags: ["ui", "pause"],
        priority: 3,
    };
    
    QUICK_SAVE: {
        name: "Quick Save",
        category: UI,
        input_type: Digital,
        bindings: [
            InputBinding::Modified {
                modifier: PhysicalInput::Keyboard(KeyCode::LeftCtrl),
                key: PhysicalInput::Keyboard(KeyCode::S),
            }
        ],
        description: "Quick save the game",
        tags: ["ui", "save", "shortcut"],
        priority: 3,
    };
    
    QUICK_LOAD: {
        name: "Quick Load",
        category: UI,
        input_type: Digital,
        bindings: [
            InputBinding::Modified {
                modifier: PhysicalInput::Keyboard(KeyCode::LeftCtrl),
                key: PhysicalInput::Keyboard(KeyCode::L),
            }
        ],
        description: "Quick load the last save",
        tags: ["ui", "load", "shortcut"],
        priority: 3,
    };
    
    // Interaction actions
    INTERACT: {
        name: "Interact",
        category: Interaction,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::E)),
            InputBinding::Single(PhysicalInput::Gamepad(GamepadButton::A))
        ],
        description: "Interact with objects or NPCs",
        tags: ["interaction", "basic"],
        priority: 1,
    };
    
    // Debug actions
    DEBUG_CONSOLE: {
        name: "Debug Console",
        category: Debug,
        input_type: Digital,
        bindings: [
            InputBinding::Combo(vec![
                PhysicalInput::Keyboard(KeyCode::LeftCtrl),
                PhysicalInput::Keyboard(KeyCode::LeftShift),
                PhysicalInput::Keyboard(KeyCode::C),
            ])
        ],
        description: "Open the debug console",
        tags: ["debug", "developer"],
        priority: 5,
        context: "debug_mode",
    };
    
    DEBUG_TOGGLE_WIREFRAME: {
        name: "Toggle Wireframe",
        category: Debug,
        input_type: Digital,
        bindings: [
            InputBinding::Single(PhysicalInput::Keyboard(KeyCode::F1))
        ],
        description: "Toggle wireframe rendering",
        tags: ["debug", "rendering"],
        priority: 4,
        context: "debug_mode",
    };
}

/// Get all predefined actions
pub fn get_predefined_actions() -> Vec<GameAction> {
    get_all_actions()
}

/// Get actions by category
pub fn get_movement_actions() -> Vec<GameAction> {
    get_actions_by_category(ActionCategory::Movement)
}

pub fn get_combat_actions() -> Vec<GameAction> {
    get_actions_by_category(ActionCategory::Combat)
}

pub fn get_ui_actions() -> Vec<GameAction> {
    get_actions_by_category(ActionCategory::UI)
}

pub fn get_debug_actions() -> Vec<GameAction> {
    get_actions_by_category(ActionCategory::Debug)
}

pub fn get_interaction_actions() -> Vec<GameAction> {
    get_actions_by_category(ActionCategory::Interaction)
}
