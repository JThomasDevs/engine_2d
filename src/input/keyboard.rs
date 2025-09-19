use std::collections::HashMap;
use crate::input::types::*;

/// Keyboard input handler for the game engine
/// 
/// This module provides keyboard input handling including:
/// - Key state tracking (pressed, held, released)
/// - Key repeat handling
/// - Integration with the InputManager
pub struct KeyboardInput {
    /// Current key states
    key_states: HashMap<KeyCode, bool>,
    
    /// Previous key states for detecting press/release
    previous_key_states: HashMap<KeyCode, bool>,
    
    /// Track if we should quit
    should_quit: bool,
    
    /// Key repeat settings
    repeat_enabled: bool,
    repeat_delay: f32,
    repeat_rate: f32,
    
    /// Time since key was first pressed (for repeat)
    key_times: HashMap<KeyCode, f32>,
}

impl KeyboardInput {
    /// Create a new keyboard input handler
    pub fn new() -> Self {
        Self {
            key_states: HashMap::new(),
            previous_key_states: HashMap::new(),
            should_quit: false,
            repeat_enabled: true,
            repeat_delay: 0.5, // 500ms delay before repeat starts
            repeat_rate: 0.05, // 50ms between repeats
            key_times: HashMap::new(),
        }
    }
    
    /// Update keyboard input (call each frame)
    pub fn update(&mut self, delta_time: f32) {
        // Store previous states
        self.previous_key_states = self.key_states.clone();
        
        // Update key repeat times
        if self.repeat_enabled {
            for (key, &pressed) in &self.key_states {
                if pressed {
                    let current_time = self.key_times.get(key).copied().unwrap_or(0.0);
                    self.key_times.insert(*key, current_time + delta_time);
                } else {
                    self.key_times.remove(key);
                }
            }
        }
    }
    
    /// Handle key press event
    pub fn handle_key_press(&mut self, key: KeyCode) {
        // Check for quit keys
        match key {
            KeyCode::Escape => {
                self.should_quit = true;
            }
            _ => {}
        }
        
        self.key_states.insert(key, true);
    }
    
    /// Handle key release event
    pub fn handle_key_release(&mut self, key: KeyCode) {
        self.key_times.remove(&key);
        self.key_states.insert(key, false);
    }
    
    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.key_states.get(&key).copied().unwrap_or(false)
    }
    
    /// Check if a key was just pressed this frame
    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        let current = self.key_states.get(&key).copied().unwrap_or(false);
        let previous = self.previous_key_states.get(&key).copied().unwrap_or(false);
        current && !previous
    }
    
    /// Check if a key was just released this frame
    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        let current = self.key_states.get(&key).copied().unwrap_or(false);
        let previous = self.previous_key_states.get(&key).copied().unwrap_or(false);
        !current && previous
    }
    
    /// Check if a key should repeat (for text input, etc.)
    pub fn should_key_repeat(&self, key: KeyCode) -> bool {
        if !self.repeat_enabled {
            return false;
        }
        
        if !self.is_key_pressed(key) {
            return false;
        }
        
        if let Some(&time) = self.key_times.get(&key) {
            if time >= self.repeat_delay {
                // Check if enough time has passed for the next repeat
                let repeat_time = time - self.repeat_delay;
                return (repeat_time / self.repeat_rate).floor() != 
                       ((repeat_time - 0.016) / self.repeat_rate).floor(); // 0.016 ≈ 1/60fps
            }
        }
        
        false
    }
    
    /// Get all currently pressed keys
    pub fn pressed_keys(&self) -> Vec<KeyCode> {
        self.key_states.iter()
            .filter(|(_, pressed)| **pressed)
            .map(|(key, _)| *key)
            .collect()
    }
    
    /// Check if we should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    
    /// Reset quit flag
    pub fn reset_quit_flag(&mut self) {
        self.should_quit = false;
    }
    
    /// Set key repeat settings
    pub fn set_repeat_settings(&mut self, enabled: bool, delay: f32, rate: f32) {
        self.repeat_enabled = enabled;
        self.repeat_delay = delay;
        self.repeat_rate = rate;
    }
    
    /// Update the InputManager with current keyboard state
    pub fn update_input_manager(&self, input_manager: &mut crate::input::manager::InputManager) {
        // Update all key states
        for (key, pressed) in &self.key_states {
            let physical_input = PhysicalInput::Keyboard(*key);
            input_manager.set_physical_input_state(physical_input, *pressed);
        }
    }
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Keyboard input event types for integration with window systems
#[derive(Debug, Clone)]
pub enum KeyboardEvent {
    KeyPress { key: KeyCode },
    KeyRelease { key: KeyCode },
    TextInput { text: String },
}

impl KeyboardInput {
    /// Handle a keyboard event
    pub fn handle_event(&mut self, event: KeyboardEvent) {
        match event {
            KeyboardEvent::KeyPress { key } => {
                self.handle_key_press(key);
            }
            KeyboardEvent::KeyRelease { key } => {
                self.handle_key_release(key);
            }
            KeyboardEvent::TextInput { text } => {
                // Handle text input (for text fields, etc.)
                // This could be used for chat, console input, etc.
                println!("Text input: {}", text);
            }
        }
    }
}
