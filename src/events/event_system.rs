use crate::events::event_types::*;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

/// Simplified event system for basic rendering and input
#[derive(Clone)]
pub struct EventSystem {
    render_sender: Sender<RenderEvent>,
    render_receiver: Arc<Mutex<Receiver<RenderEvent>>>,
    input_sender: Sender<InputEvent>,
    input_receiver: Arc<Mutex<Receiver<InputEvent>>>,
}

impl EventSystem {
    /// Create a new event system
    pub fn new() -> Self {
        let (render_sender, render_receiver) = mpsc::channel();
        let (input_sender, input_receiver) = mpsc::channel();
        
        Self {
            render_sender,
            render_receiver: Arc::new(Mutex::new(render_receiver)),
            input_sender,
            input_receiver: Arc::new(Mutex::new(input_receiver)),
        }
    }
    
    /// Send a render event
    pub fn send_render_event(&self, event: RenderEvent) -> Result<(), String> {
        self.render_sender.send(event).map_err(|_| {
            "Failed to send render event".to_string()
        })
    }

    /// Send an input event
    pub fn send_input_event(&self, event: InputEvent) -> Result<(), String> {
        self.input_sender.send(event).map_err(|_| {
            "Failed to send input event".to_string()
        })
    }
    
    /// Get the render event sender (for other systems to use)
    pub fn get_render_sender(&self) -> Sender<RenderEvent> {
        self.render_sender.clone()
    }
    
    /// Get the render event receiver (for render service to use)
    pub fn get_render_receiver(&self) -> Arc<Mutex<Receiver<RenderEvent>>> {
        Arc::clone(&self.render_receiver)
    }

    /// Get the input event sender (for other systems to use)
    pub fn get_input_sender(&self) -> Sender<InputEvent> {
        self.input_sender.clone()
    }

    /// Get the input event receiver (for input service to use)
    pub fn get_input_receiver(&self) -> Arc<Mutex<Receiver<InputEvent>>> {
        Arc::clone(&self.input_receiver)
    }

    /// Receive an input event (non-blocking)
    pub fn receive_input_event(&self) -> Option<InputEvent> {
        match self.input_receiver.try_lock() {
            Ok(receiver) => receiver.try_recv().ok(),
            Err(std::sync::TryLockError::Poisoned(poisoned)) => poisoned.into_inner().try_recv().ok(),
            Err(std::sync::TryLockError::WouldBlock) => None,
        }
    }
}