use std::collections::HashMap;
use crate::input::types::*;

/// Gamepad input handler for the game engine
/// 
/// This module provides gamepad input handling including:
/// - Button state tracking (pressed, held, released)
/// - Analog stick and trigger values
/// - Multiple gamepad support
/// - Integration with the InputManager
pub struct GamepadInput {
    /// Connected gamepads by ID
    gamepads: HashMap<u32, GamepadState>,
}

/// State of a single gamepad
#[derive(Debug, Clone)]
pub struct GamepadState {
    /// Gamepad ID
    pub id: u32,
    
    /// Whether the gamepad is connected
    pub connected: bool,
    
    /// Button states
    pub button_states: HashMap<GamepadButton, bool>,
    
    /// Previous button states for detecting press/release
    pub previous_button_states: HashMap<GamepadButton, bool>,
    
    /// Analog stick and trigger values (-1.0 to 1.0)
    pub axis_values: HashMap<GamepadAxis, f32>,
    
    /// Deadzone for analog sticks
    pub deadzone: f32,
    
    /// Gamepad name/type
    pub name: String,
}

impl GamepadState {
    /// Create a new gamepad state
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            connected: true,
            button_states: HashMap::new(),
            previous_button_states: HashMap::new(),
            axis_values: HashMap::new(),
            deadzone: 0.1,
            name,
        }
    }
    
    /// Update gamepad state (call each frame)
    pub fn update(&mut self) {
        // Store previous button states
        self.previous_button_states = self.button_states.clone();
    }
    
    /// Set button state
    pub fn set_button(&mut self, button: GamepadButton, pressed: bool) {
        self.button_states.insert(button, pressed);
    }
    
    /// Set axis value
    pub fn set_axis(&mut self, axis: GamepadAxis, value: f32) {
        // Apply deadzone
        let deadzone_value = if value.abs() < self.deadzone {
            0.0
        } else {
            value
        };
        self.axis_values.insert(axis, deadzone_value);
    }
    
    /// Check if button is pressed
    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        self.button_states.get(&button).copied().unwrap_or(false)
    }
    
    /// Check if button was just pressed this frame
    pub fn is_button_just_pressed(&self, button: GamepadButton) -> bool {
        let current = self.button_states.get(&button).copied().unwrap_or(false);
        let previous = self.previous_button_states.get(&button).copied().unwrap_or(false);
        current && !previous
    }
    
    /// Check if button was just released this frame
    pub fn is_button_just_released(&self, button: GamepadButton) -> bool {
        let current = self.button_states.get(&button).copied().unwrap_or(false);
        let previous = self.previous_button_states.get(&button).copied().unwrap_or(false);
        !current && previous
    }
    
    /// Get axis value
    pub fn get_axis(&self, axis: GamepadAxis) -> f32 {
        self.axis_values.get(&axis).copied().unwrap_or(0.0)
    }
    
    /// Set deadzone
    pub fn set_deadzone(&mut self, deadzone: f32) {
        self.deadzone = deadzone.clamp(0.0, 1.0);
    }
}

impl GamepadInput {
    /// Create a new gamepad input handler
    pub fn new() -> Self {
        Self {
            gamepads: HashMap::new(),
        }
    }
    
    /// Update all gamepads (call each frame)
    pub fn update(&mut self) {
        for gamepad in self.gamepads.values_mut() {
            gamepad.update();
        }
    }
    
    /// Add a connected gamepad
    pub fn add_gamepad(&mut self, id: u32, name: String) {
        let gamepad = GamepadState::new(id, name.clone());
        self.gamepads.insert(id, gamepad);
        println!("🎮 Gamepad {} connected: {}", id, name);
    }
    
    /// Remove a disconnected gamepad
    pub fn remove_gamepad(&mut self, id: u32) {
        if let Some(gamepad) = self.gamepads.remove(&id) {
            println!("🎮 Gamepad {} disconnected: {}", id, gamepad.name);
        }
    }
    
    /// Get a gamepad by ID
    pub fn get_gamepad(&self, id: u32) -> Option<&GamepadState> {
        self.gamepads.get(&id)
    }
    
    /// Get a mutable gamepad by ID
    pub fn get_gamepad_mut(&mut self, id: u32) -> Option<&mut GamepadState> {
        self.gamepads.get_mut(&id)
    }
    
    /// Get all connected gamepads
    pub fn connected_gamepads(&self) -> Vec<&GamepadState> {
        self.gamepads.values().filter(|g| g.connected).collect()
    }
    
    /// Get the first connected gamepad (for single-player games)
    pub fn primary_gamepad(&self) -> Option<&GamepadState> {
        self.connected_gamepads().first().copied()
    }
    
    /// Handle gamepad button event
    pub fn handle_button_event(&mut self, gamepad_id: u32, button: GamepadButton, pressed: bool) {
        if let Some(gamepad) = self.gamepads.get_mut(&gamepad_id) {
            gamepad.set_button(button, pressed);
        }
    }
    
    /// Handle gamepad axis event
    pub fn handle_axis_event(&mut self, gamepad_id: u32, axis: GamepadAxis, value: f32) {
        if let Some(gamepad) = self.gamepads.get_mut(&gamepad_id) {
            gamepad.set_axis(axis, value);
        }
    }
    
    /// Update the InputManager with current gamepad states
    pub fn update_input_manager(&self, input_manager: &mut crate::input::manager::InputManager) {
        // For now, we'll use the primary gamepad for input mapping
        // In a full implementation, you might want to support multiple gamepads
        if let Some(gamepad) = self.primary_gamepad() {
            // Update button states
            for (button, pressed) in &gamepad.button_states {
                let physical_input = PhysicalInput::Gamepad(*button);
                input_manager.set_physical_input_state(physical_input, *pressed);
            }
            
            // Update axis values
            for (axis, value) in &gamepad.axis_values {
                let physical_input = PhysicalInput::GamepadAxis(*axis);
                input_manager.set_physical_input_value(physical_input, *value);
            }
        }
    }
    
    /// Set deadzone for a specific gamepad
    pub fn set_gamepad_deadzone(&mut self, gamepad_id: u32, deadzone: f32) {
        if let Some(gamepad) = self.gamepads.get_mut(&gamepad_id) {
            gamepad.set_deadzone(deadzone);
        }
    }
    
    /// Set deadzone for all gamepads
    pub fn set_global_deadzone(&mut self, deadzone: f32) {
        for gamepad in self.gamepads.values_mut() {
            gamepad.set_deadzone(deadzone);
        }
    }
}

impl Default for GamepadInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Gamepad input event types for integration with input libraries
#[derive(Debug, Clone)]
pub enum GamepadEvent {
    Connected { id: u32, name: String },
    Disconnected { id: u32 },
    Button { id: u32, button: GamepadButton, pressed: bool },
    Axis { id: u32, axis: GamepadAxis, value: f32 },
}

impl GamepadInput {
    /// Handle a gamepad event
    pub fn handle_event(&mut self, event: GamepadEvent) {
        match event {
            GamepadEvent::Connected { id, name } => {
                self.add_gamepad(id, name);
            }
            GamepadEvent::Disconnected { id } => {
                self.remove_gamepad(id);
            }
            GamepadEvent::Button { id, button, pressed } => {
                self.handle_button_event(id, button, pressed);
            }
            GamepadEvent::Axis { id, axis, value } => {
                self.handle_axis_event(id, axis, value);
            }
        }
    }
}

/// Gamepad button mapping for common controllers
pub mod button_mapping {
    use super::*;
    
    /// Map Xbox controller buttons to our GamepadButton enum
    pub fn xbox_button_mapping() -> HashMap<u32, GamepadButton> {
        let mut mapping = HashMap::new();
        
        // Face buttons
        mapping.insert(0, GamepadButton::A);      // A button
        mapping.insert(1, GamepadButton::B);      // B button
        mapping.insert(2, GamepadButton::X);      // X button
        mapping.insert(3, GamepadButton::Y);      // Y button
        
        // Shoulder buttons
        mapping.insert(4, GamepadButton::LeftShoulder);   // Left bumper
        mapping.insert(5, GamepadButton::RightShoulder);  // Right bumper
        mapping.insert(6, GamepadButton::LeftTrigger);    // Left trigger
        mapping.insert(7, GamepadButton::RightTrigger);   // Right trigger
        
        // Center buttons
        mapping.insert(8, GamepadButton::Select);  // Back button
        mapping.insert(9, GamepadButton::Start);   // Start button
        mapping.insert(10, GamepadButton::Guide);  // Guide button
        
        // Sticks
        mapping.insert(11, GamepadButton::LeftStick);   // Left stick click
        mapping.insert(12, GamepadButton::RightStick);  // Right stick click
        
        // D-pad
        mapping.insert(13, GamepadButton::DPadUp);    // D-pad up
        mapping.insert(14, GamepadButton::DPadDown);  // D-pad down
        mapping.insert(15, GamepadButton::DPadLeft);  // D-pad left
        mapping.insert(16, GamepadButton::DPadRight); // D-pad right
        
        mapping
    }
    
    /// Map PlayStation controller buttons to our GamepadButton enum
    pub fn playstation_button_mapping() -> HashMap<u32, GamepadButton> {
        let mut mapping = HashMap::new();
        
        // Face buttons (using South/East/West/North for PlayStation)
        mapping.insert(0, GamepadButton::South);  // Cross button
        mapping.insert(1, GamepadButton::East);   // Circle button
        mapping.insert(2, GamepadButton::West);   // Square button
        mapping.insert(3, GamepadButton::North);  // Triangle button
        
        // Shoulder buttons
        mapping.insert(4, GamepadButton::LeftShoulder);   // L1
        mapping.insert(5, GamepadButton::RightShoulder);  // R1
        mapping.insert(6, GamepadButton::LeftTrigger);    // L2
        mapping.insert(7, GamepadButton::RightTrigger);   // R2
        
        // Center buttons
        mapping.insert(8, GamepadButton::Select);  // Share button
        mapping.insert(9, GamepadButton::Start);   // Options button
        mapping.insert(10, GamepadButton::Guide);  // PS button
        
        // Sticks
        mapping.insert(11, GamepadButton::LeftStick);   // L3
        mapping.insert(12, GamepadButton::RightStick);  // R3
        
        // D-pad
        mapping.insert(13, GamepadButton::DPadUp);    // D-pad up
        mapping.insert(14, GamepadButton::DPadDown);  // D-pad down
        mapping.insert(15, GamepadButton::DPadLeft);  // D-pad left
        mapping.insert(16, GamepadButton::DPadRight); // D-pad right
        
        mapping
    }
}