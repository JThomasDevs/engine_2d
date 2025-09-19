use std::collections::HashMap;
use std::time::Instant;

use crate::input::types::*;

/// Main input manager for handling game actions and input state
/// 
/// The InputManager provides a centralized system for:
/// - Registering and managing game actions
/// - Tracking input state (pressed, held, released)
/// - Context-aware input processing
/// - Action value retrieval for analog inputs
pub struct InputManager {
    /// Registered actions by ID
    actions: HashMap<String, GameAction>,
    
    /// Current state of each action
    action_states: HashMap<String, InputState>,
    
    /// Raw input states for physical inputs
    raw_inputs: HashMap<PhysicalInput, bool>,
    
    /// Raw input values for analog inputs
    raw_values: HashMap<PhysicalInput, f32>,
    
    /// Active input contexts (stack-based)
    active_contexts: Vec<InputContext>,
    
    /// Input event history for debugging
    input_history: Vec<InputEvent>,
    
    /// Maximum history size
    max_history_size: usize,
}

impl InputManager {
    /// Create a new InputManager
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
            action_states: HashMap::new(),
            raw_inputs: HashMap::new(),
            raw_values: HashMap::new(),
            active_contexts: Vec::new(),
            input_history: Vec::new(),
            max_history_size: 1000,
        }
    }
    
    /// Register a game action
    pub fn register_action(&mut self, action: GameAction) {
        let action_id = action.id.clone();
        self.actions.insert(action_id.clone(), action);
        self.action_states.insert(action_id, InputState::Idle);
    }
    
    /// Register multiple actions at once
    pub fn register_actions(&mut self, actions: Vec<GameAction>) {
        for action in actions {
            self.register_action(action);
        }
    }
    
    /// Update the input manager (call each frame)
    pub fn update(&mut self, _delta_time: f32) {
        // Update action states based on current raw inputs
        for (action_id, action) in &self.actions.clone() {
            self.update_action_state(action_id, action);
        }
        
        // Generate events for state changes
        self.generate_action_events();
        
        // Clean up old history
        if self.input_history.len() > self.max_history_size {
            self.input_history.drain(0..self.input_history.len() - self.max_history_size);
        }
    }
    
    /// Update the state of a specific action
    fn update_action_state(&mut self, action_id: &str, action: &GameAction) {
        let current_state = self.action_states.get(action_id).cloned().unwrap_or(InputState::Idle);
        let new_state = self.calculate_action_state(action);
        
        // Update state if it changed
        if current_state != new_state {
            self.action_states.insert(action_id.to_string(), new_state);
        }
    }
    
    /// Calculate the new state for an action based on its bindings
    fn calculate_action_state(&self, action: &GameAction) -> InputState {
        // Check if any binding for this action is active
        let any_binding_active = action.default_bindings.iter()
            .any(|binding| self.is_binding_active(binding));
        
        let current_state = self.action_states.get(&action.id).cloned().unwrap_or(InputState::Idle);
        
        match (current_state, any_binding_active) {
            (InputState::Idle, true) => InputState::Pressed,
            (InputState::Pressed, true) => InputState::Held,
            (InputState::Held, true) => InputState::Held,
            (InputState::Released, true) => InputState::Pressed,
            (_, false) => InputState::Released,
        }
    }
    
    /// Check if a binding is currently active
    fn is_binding_active(&self, binding: &InputBinding) -> bool {
        match binding {
            InputBinding::Single(input) => {
                self.is_physical_input_active(input)
            }
            
            InputBinding::Modified { modifier, key } => {
                self.is_physical_input_active(modifier) && 
                self.is_physical_input_active(key)
            }
            
            InputBinding::Combo(inputs) => {
                inputs.iter().all(|input| self.is_physical_input_active(input))
            }
            
            InputBinding::Analog { input, threshold, .. } => {
                self.get_physical_input_value(input).abs() > *threshold
            }
        }
    }
    
    /// Check if a physical input is currently active
    fn is_physical_input_active(&self, input: &PhysicalInput) -> bool {
        match input {
            PhysicalInput::Keyboard(_) | 
            PhysicalInput::Mouse(_) | 
            PhysicalInput::Gamepad(_) => {
                self.raw_inputs.get(input).copied().unwrap_or(false)
            }
            PhysicalInput::MouseAxis(_) | 
            PhysicalInput::GamepadAxis(_) => {
                // Analog inputs are "active" if they have a non-zero value
                self.get_physical_input_value(input).abs() > 0.0
            }
        }
    }
    
    /// Get the current value of a physical input
    fn get_physical_input_value(&self, input: &PhysicalInput) -> f32 {
        self.raw_values.get(input).copied().unwrap_or(0.0)
    }
    
    /// Set the state of a physical input
    pub fn set_physical_input_state(&mut self, input: PhysicalInput, active: bool) {
        self.raw_inputs.insert(input, active);
    }
    
    /// Set the value of a physical input (for analog inputs)
    pub fn set_physical_input_value(&mut self, input: PhysicalInput, value: f32) {
        self.raw_values.insert(input, value);
    }
    
    /// Check if an action is currently pressed (just pressed this frame)
    pub fn is_action_pressed(&self, action_id: &str) -> bool {
        if !self.is_action_enabled(action_id) {
            return false;
        }
        
        self.action_states.get(action_id)
            .map(|state| matches!(state, InputState::Pressed))
            .unwrap_or(false)
    }
    
    /// Check if an action is currently held
    pub fn is_action_held(&self, action_id: &str) -> bool {
        if !self.is_action_enabled(action_id) {
            return false;
        }
        
        self.action_states.get(action_id)
            .map(|state| matches!(state, InputState::Held))
            .unwrap_or(false)
    }
    
    /// Check if an action was just released
    pub fn is_action_released(&self, action_id: &str) -> bool {
        if !self.is_action_enabled(action_id) {
            return false;
        }
        
        self.action_states.get(action_id)
            .map(|state| matches!(state, InputState::Released))
            .unwrap_or(false)
    }
    
    /// Get the current value of an action (for analog inputs)
    pub fn get_action_value(&self, action_id: &str) -> f32 {
        if !self.is_action_enabled(action_id) {
            return 0.0;
        }
        
        if let Some(action) = self.actions.get(action_id) {
            match action.input_type {
                InputType::Digital => {
                    if self.is_action_pressed(action_id) || self.is_action_held(action_id) {
                        1.0
                    } else {
                        0.0
                    }
                }
                InputType::Analog => {
                    // Get analog value from bindings
                    for binding in &action.default_bindings {
                        if let Some(value) = self.get_binding_value(binding) {
                            return value;
                        }
                    }
                    0.0
                }
                InputType::Hybrid => {
                    // Can be both digital and analog
                    if self.is_action_pressed(action_id) || self.is_action_held(action_id) {
                        1.0
                    } else {
                        // Check for analog value
                        for binding in &action.default_bindings {
                            if let Some(value) = self.get_binding_value(binding) {
                                return value;
                            }
                        }
                        0.0
                    }
                }
            }
        } else {
            0.0
        }
    }
    
    /// Get the value of a binding
    fn get_binding_value(&self, binding: &InputBinding) -> Option<f32> {
        match binding {
            InputBinding::Analog { input, threshold, deadzone } => {
                let raw_value = self.get_physical_input_value(input);
                let abs_value = raw_value.abs();
                
                if abs_value < *deadzone {
                    Some(0.0)
                } else if abs_value > *threshold {
                    Some(raw_value.signum())
                } else {
                    Some(raw_value)
                }
            }
            _ => None,
        }
    }
    
    /// Check if an action is enabled in the current context
    pub fn is_action_enabled(&self, action_id: &str) -> bool {
        if let Some(action) = self.actions.get(action_id) {
            // Check if context is required
            if let Some(required_context) = &action.metadata.context_required {
                if !self.active_contexts.iter().any(|ctx| ctx.name == *required_context) {
                    return false;
                }
            }
            
            // Check context restrictions
            for context in &self.active_contexts {
                if context.disabled_actions.contains(action_id) {
                    return false;
                }
                if !context.enabled_actions.is_empty() && !context.enabled_actions.contains(action_id) {
                    return false;
                }
            }
        }
        true
    }
    
    /// Push a new input context
    pub fn push_context(&mut self, context: InputContext) {
        self.active_contexts.push(context);
        self.active_contexts.sort_by_key(|c| c.priority);
    }
    
    /// Pop the current input context
    pub fn pop_context(&mut self) -> Option<InputContext> {
        self.active_contexts.pop()
    }
    
    /// Clear all input contexts
    pub fn clear_contexts(&mut self) {
        self.active_contexts.clear();
    }
    
    /// Get all registered actions
    pub fn get_actions(&self) -> Vec<&GameAction> {
        self.actions.values().collect()
    }
    
    /// Get actions by category
    pub fn get_actions_by_category(&self, category: ActionCategory) -> Vec<&GameAction> {
        self.actions.values()
            .filter(|action| action.category == category)
            .collect()
    }
    
    /// Get action by ID
    pub fn get_action(&self, action_id: &str) -> Option<&GameAction> {
        self.actions.get(action_id)
    }
    
    /// Generate input events for state changes
    fn generate_action_events(&mut self) {
        let now = Instant::now();
        
        for (action_id, state) in &self.action_states.clone() {
            if let Some(action) = self.actions.get(action_id) {
                let intensity = match action.input_type {
                    InputType::Digital => {
                        if matches!(state, InputState::Pressed | InputState::Held) { 1.0 } else { 0.0 }
                    }
                    InputType::Analog => self.get_action_value(action_id),
                    InputType::Hybrid => {
                        if matches!(state, InputState::Pressed | InputState::Held) {
                            1.0
                        } else {
                            self.get_action_value(action_id)
                        }
                    }
                };
                
                if intensity > 0.0 {
                    let event = InputEvent::ActionTriggered {
                        action_id: action_id.clone(),
                        intensity,
                        timestamp: now,
                    };
                    self.input_history.push(event);
                }
            }
        }
    }
    
    /// Get recent input events
    pub fn get_recent_events(&self, count: usize) -> Vec<&InputEvent> {
        self.input_history.iter().rev().take(count).collect()
    }
    
    /// Clear input history
    pub fn clear_history(&mut self) {
        self.input_history.clear();
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}
