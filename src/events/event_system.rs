use crate::events::event_types::*;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

/// Simplified event system for basic rendering
#[derive(Clone)]
pub struct EventSystem {
    render_sender: Sender<RenderEvent>,
    render_receiver: Arc<Mutex<Receiver<RenderEvent>>>,
}

impl EventSystem {
    /// Create a new event system
    pub fn new() -> Self {
        let (render_sender, render_receiver) = mpsc::channel();
        
        Self {
            render_sender,
            render_receiver: Arc::new(Mutex::new(render_receiver)),
        }
    }
    
    /// Send a render event
    pub fn send_render_event(&self, event: RenderEvent) -> Result<(), String> {
        self.render_sender.send(event).map_err(|_| {
            "Failed to send render event".to_string()
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
}