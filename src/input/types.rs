use std::hash::{Hash, Hasher};

/// Core input system types for the game engine
///
/// This module provides the foundational types for a flexible, type-safe
/// input system that supports customizable keybindings and action mapping.
/// Represents a game action with rich metadata
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GameAction {
    pub id: String,
    pub display_name: String,
    pub category: ActionCategory,
    pub input_type: InputType,
    pub default_bindings: Vec<InputBinding>,
    pub metadata: ActionMetadata,
}

/// Categories for organizing actions
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ActionCategory {
    Movement,
    Combat,
    UI,
    Debug,
    Interaction,
    Custom(String),
}

/// Types of input that actions can handle
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum InputType {
    Digital, // On/off inputs (keys, buttons)
    Analog,  // Continuous inputs (mouse, gamepad sticks)
    Hybrid,  // Can be both digital and analog
}

/// Rich metadata for actions
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ActionMetadata {
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub priority: u32,
    pub context_required: Option<String>,
}

/// Input bindings that map physical inputs to actions
#[derive(Clone, PartialEq, Debug)]
pub enum InputBinding {
    /// Single input (key, button, axis)
    Single(PhysicalInput),

    /// Modifier + key combination (e.g., Ctrl+S)
    Modified {
        modifier: PhysicalInput,
        key: PhysicalInput,
    },

    /// Multiple simultaneous inputs (e.g., Ctrl+Shift+D)
    Combo(Vec<PhysicalInput>),

    /// Analog input with threshold and deadzone
    Analog {
        input: PhysicalInput,
        threshold: f32,
        deadzone: f32,
    },
}

impl Eq for InputBinding {}

impl Hash for InputBinding {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            InputBinding::Single(input) => {
                0u8.hash(state);
                input.hash(state);
            }
            InputBinding::Modified { modifier, key } => {
                1u8.hash(state);
                modifier.hash(state);
                key.hash(state);
            }
            InputBinding::Combo(inputs) => {
                2u8.hash(state);
                inputs.len().hash(state);
                for input in inputs {
                    input.hash(state);
                }
            }
            InputBinding::Analog {
                input,
                threshold,
                deadzone,
            } => {
                3u8.hash(state);
                input.hash(state);
                // Convert f32 to u32 for hashing
                threshold.to_bits().hash(state);
                deadzone.to_bits().hash(state);
            }
        }
    }
}

/// Physical input devices and their specific inputs
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum PhysicalInput {
    Keyboard(KeyCode),
    Mouse(MouseButton),
    MouseAxis(MouseAxis),
    Gamepad(GamepadButton),
    GamepadAxis(GamepadAxis),
}

/// Keyboard key codes
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum KeyCode {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Special keys
    Space,
    Enter,
    Escape,
    Tab,
    Backspace,
    Delete,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    LeftSuper,
    RightSuper, // Windows/Command keys

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Other
    CapsLock,
    NumLock,
    ScrollLock,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    PrintScreen,
    Pause,

    // Numpad
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadEnter,

    // Punctuation
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Period,
    Slash,
    Backslash,
    LeftBracket,
    RightBracket,
    Minus,
    Equals,
}

/// Mouse button types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Forward,   // Mouse 4
    Back,      // Mouse 5
    Other(u8), // Additional mouse buttons
}

/// Mouse axis types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MouseAxis {
    X,
    Y,
    ScrollX,
    ScrollY,
}

/// Gamepad button types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GamepadButton {
    // Face buttons (PlayStation: X, Square, Circle, Triangle)
    South, // X/A button
    East,  // Circle/B button
    West,  // Square/X button
    North, // Triangle/Y button

    // Xbox-style face buttons (for compatibility)
    A, // Same as South
    B, // Same as East
    X, // Same as West
    Y, // Same as North

    // Shoulder buttons
    LeftTrigger,
    RightTrigger,
    LeftShoulder,
    RightShoulder,

    // D-pad
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,

    // Center buttons
    Start,
    Select,
    Guide, // Xbox/PS button

    // Sticks
    LeftStick,
    RightStick,
}

/// Gamepad axis types
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

/// Input state for actions
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum InputState {
    Pressed,  // Just pressed this frame
    Held,     // Held down
    Released, // Just released this frame
    Idle,     // Not pressed
}

/// Input context for managing different game states
#[derive(Clone, PartialEq, Debug)]
pub struct InputContext {
    pub name: String,
    pub priority: u32,
    pub enabled_actions: std::collections::HashSet<String>,
    pub disabled_actions: std::collections::HashSet<String>,
}

impl Eq for InputContext {}

impl Hash for InputContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.priority.hash(state);
        // Hash the sets by converting to sorted vectors
        let mut enabled: Vec<_> = self.enabled_actions.iter().collect();
        enabled.sort();
        enabled.hash(state);

        let mut disabled: Vec<_> = self.disabled_actions.iter().collect();
        disabled.sort();
        disabled.hash(state);
    }
}

impl InputContext {
    pub fn new(name: String, priority: u32) -> Self {
        Self {
            name,
            priority,
            enabled_actions: std::collections::HashSet::new(),
            disabled_actions: std::collections::HashSet::new(),
        }
    }

    pub fn enable_action(mut self, action_id: String) -> Self {
        self.enabled_actions.insert(action_id);
        self
    }

    pub fn disable_action(mut self, action_id: String) -> Self {
        self.disabled_actions.insert(action_id);
        self
    }
}

/// Input event for the event system
#[derive(Clone, Debug)]
pub enum InputEvent {
    ActionTriggered {
        action_id: String,
        intensity: f32,
        timestamp: std::time::Instant,
    },
    ContextChanged {
        old_context: Option<String>,
        new_context: Option<String>,
    },
    InputCombo {
        actions: Vec<String>,
        duration: std::time::Duration,
    },
}
