use std::collections::HashMap;
use crate::input::types::*;

/// Mouse input handler for the game engine
/// 
/// This module provides mouse input handling including:
/// - Button state tracking (pressed, held, released)
/// - Position tracking (absolute and relative)
/// - Scroll wheel input
/// - Integration with the InputManager
pub struct MouseInput {
    /// Current mouse position
    position: (f32, f32),
    
    /// Previous mouse position for delta calculation
    previous_position: (f32, f32),
    
    /// Mouse button states
    button_states: HashMap<MouseButton, bool>,
    
    /// Previous button states for detecting press/release
    previous_button_states: HashMap<MouseButton, bool>,
    
    /// Scroll wheel delta
    scroll_delta: (f32, f32),
    
    /// Whether the mouse is captured (relative mode)
    captured: bool,
    
    /// Mouse sensitivity for relative movement
    sensitivity: f32,
}

impl MouseInput {
    /// Create a new mouse input handler
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            previous_position: (0.0, 0.0),
            button_states: HashMap::new(),
            previous_button_states: HashMap::new(),
            scroll_delta: (0.0, 0.0),
            captured: false,
            sensitivity: 1.0,
        }
    }
    
    /// Update mouse input (call each frame)
    pub fn update(&mut self) {
        // Store previous states
        self.previous_position = self.position;
        self.previous_button_states = std::mem::take(&mut self.button_states);
        
        // Reset scroll delta (it's event-driven, not state-based)
        self.scroll_delta = (0.0, 0.0);
    }
    
    /// Handle mouse movement event
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        if self.captured {
            // In captured mode, treat as relative movement
            let delta_x = x * self.sensitivity;
            let delta_y = y * self.sensitivity;
            self.position.0 += delta_x;
            self.position.1 += delta_y;
        } else {
            // In normal mode, use absolute position
            self.position = (x, y);
        }
    }
    
    /// Handle mouse button press event
    pub fn handle_button_press(&mut self, button: MouseButton) {
        self.button_states.insert(button, true);
    }
    
    /// Handle mouse button release event
    pub fn handle_button_release(&mut self, button: MouseButton) {
        self.button_states.insert(button, false);
    }
    
    /// Handle scroll wheel event
    pub fn handle_scroll(&mut self, delta_x: f32, delta_y: f32) {
        self.scroll_delta.0 += delta_x;
        self.scroll_delta.1 += delta_y;
    }
    
    /// Get current mouse position
    pub fn position(&self) -> (f32, f32) {
        self.position
    }
    
    /// Get mouse position delta (movement since last frame)
    pub fn position_delta(&self) -> (f32, f32) {
        (
            self.position.0 - self.previous_position.0,
            self.position.1 - self.previous_position.1,
        )
    }
    
    /// Check if a mouse button is currently pressed
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.button_states.get(&button).copied().unwrap_or(false)
    }
    
    /// Check if a mouse button was just pressed this frame
    pub fn is_button_just_pressed(&self, button: MouseButton) -> bool {
        let current = self.button_states.get(&button).copied().unwrap_or(false);
        let previous = self.previous_button_states.get(&button).copied().unwrap_or(false);
        current && !previous
    }
    
    /// Check if a mouse button was just released this frame
    pub fn is_button_just_released(&self, button: MouseButton) -> bool {
        let current = self.button_states.get(&button).copied().unwrap_or(false);
        let previous = self.previous_button_states.get(&button).copied().unwrap_or(false);
        !current && previous
    }
    
    /// Get scroll wheel delta
    pub fn scroll_delta(&self) -> (f32, f32) {
        self.scroll_delta
    }
    
    /// Set mouse capture mode (relative movement)
    pub fn set_captured(&mut self, captured: bool) {
        self.captured = captured;
    }
    
    /// Check if mouse is captured
    pub fn is_captured(&self) -> bool {
        self.captured
    }
    
    /// Set mouse sensitivity
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }
    
    /// Get mouse sensitivity
    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }
    
    /// Update the InputManager with current mouse state
    pub fn update_input_manager(&self, input_manager: &mut crate::input::manager::InputManager) {
        // Update mouse button states
        for (button, pressed) in &self.button_states {
            let physical_input = PhysicalInput::Mouse(*button);
            input_manager.set_physical_input_state(physical_input, *pressed);
        }
        
        // Update mouse axis values
        let (delta_x, delta_y) = self.position_delta();
        input_manager.set_physical_input_value(PhysicalInput::MouseAxis(MouseAxis::X), delta_x);
        input_manager.set_physical_input_value(PhysicalInput::MouseAxis(MouseAxis::Y), delta_y);
        
        // Update scroll wheel
        let (scroll_x, scroll_y) = self.scroll_delta;
        input_manager.set_physical_input_value(PhysicalInput::MouseAxis(MouseAxis::ScrollX), scroll_x);
        input_manager.set_physical_input_value(PhysicalInput::MouseAxis(MouseAxis::ScrollY), scroll_y);
    }
}

impl Default for MouseInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Mouse input event types for integration with window systems
#[derive(Debug, Clone)]
pub enum MouseEvent {
    Move { x: f32, y: f32 },
    ButtonPress { button: MouseButton },
    ButtonRelease { button: MouseButton },
    Scroll { delta_x: f32, delta_y: f32 },
    Enter,
    Leave,
}

impl MouseInput {
    /// Handle a mouse event
    pub fn handle_event(&mut self, event: MouseEvent) {
        match event {
            MouseEvent::Move { x, y } => {
                self.handle_mouse_move(x, y);
            }
            MouseEvent::ButtonPress { button } => {
                self.handle_button_press(button);
            }
            MouseEvent::ButtonRelease { button } => {
                self.handle_button_release(button);
            }
            MouseEvent::Scroll { delta_x, delta_y } => {
                self.handle_scroll(delta_x, delta_y);
            }
            MouseEvent::Enter => {
                // Mouse entered window
            }
            MouseEvent::Leave => {
                // Mouse left window - could reset states if needed
            }
        }
    }
}