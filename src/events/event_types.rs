use std::time::Instant;

/// Base event trait that all events must implement
pub trait Event: Send + Sync + 'static {
    /// Get the timestamp when this event was created
    fn timestamp(&self) -> Instant;
    
    /// Get the priority of this event (higher = more important)
    fn priority(&self) -> EventPriority {
        EventPriority::Normal
    }
    
    /// Get a reference to this event as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Event priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Input events from keyboard, mouse, gamepad, etc.
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress { key: String, timestamp: Instant },
    KeyRelease { key: String, timestamp: Instant },
    MouseMove { x: f32, y: f32, timestamp: Instant },
    MouseClick { button: u32, x: f32, y: f32, timestamp: Instant },
    GamepadButton { controller_id: u32, button: u32, pressed: bool, timestamp: Instant },
}

impl Event for InputEvent {
    fn timestamp(&self) -> Instant {
        match self {
            InputEvent::KeyPress { timestamp, .. } => *timestamp,
            InputEvent::KeyRelease { timestamp, .. } => *timestamp,
            InputEvent::MouseMove { timestamp, .. } => *timestamp,
            InputEvent::MouseClick { timestamp, .. } => *timestamp,
            InputEvent::GamepadButton { timestamp, .. } => *timestamp,
        }
    }
    
    fn priority(&self) -> EventPriority {
        EventPriority::High // Input events are high priority
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Rendering events for drawing operations
#[derive(Debug, Clone)]
pub enum RenderEvent {
    ClearScreen { r: f32, g: f32, b: f32, a: f32, timestamp: Instant },
    DrawRectangle { x: f32, y: f32, width: f32, height: f32, color: (f32, f32, f32), timestamp: Instant },
    DrawSprite { x: f32, y: f32, texture_id: u32, timestamp: Instant },
    PresentFrame { timestamp: Instant },
}

impl Event for RenderEvent {
    fn timestamp(&self) -> Instant {
        match self {
            RenderEvent::ClearScreen { timestamp, .. } => *timestamp,
            RenderEvent::DrawRectangle { timestamp, .. } => *timestamp,
            RenderEvent::DrawSprite { timestamp, .. } => *timestamp,
            RenderEvent::PresentFrame { timestamp, .. } => *timestamp,
        }
    }
    
    fn priority(&self) -> EventPriority {
        EventPriority::Critical // Rendering events are critical for frame rate
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Game logic events for AI, physics, game state, etc.
#[derive(Debug, Clone)]
pub enum LogicEvent {
    UpdateGameState { delta_time: f32, timestamp: Instant },
    EntityMoved { entity_id: u32, x: f32, y: f32, timestamp: Instant },
    CollisionDetected { entity1: u32, entity2: u32, timestamp: Instant },
    GameStateChanged { new_state: String, timestamp: Instant },
}

impl Event for LogicEvent {
    fn timestamp(&self) -> Instant {
        match self {
            LogicEvent::UpdateGameState { timestamp, .. } => *timestamp,
            LogicEvent::EntityMoved { timestamp, .. } => *timestamp,
            LogicEvent::CollisionDetected { timestamp, .. } => *timestamp,
            LogicEvent::GameStateChanged { timestamp, .. } => *timestamp,
        }
    }
    
    fn priority(&self) -> EventPriority {
        EventPriority::Normal
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Audio events for sound and music
#[derive(Debug, Clone)]
pub enum AudioEvent {
    PlaySound { sound_id: u32, volume: f32, timestamp: Instant },
    PlayMusic { music_id: u32, volume: f32, timestamp: Instant },
    StopSound { sound_id: u32, timestamp: Instant },
    SetVolume { volume: f32, timestamp: Instant },
}

impl Event for AudioEvent {
    fn timestamp(&self) -> Instant {
        match self {
            AudioEvent::PlaySound { timestamp, .. } => *timestamp,
            AudioEvent::PlayMusic { timestamp, .. } => *timestamp,
            AudioEvent::StopSound { timestamp, .. } => *timestamp,
            AudioEvent::SetVolume { timestamp, .. } => *timestamp,
        }
    }
    
    fn priority(&self) -> EventPriority {
        EventPriority::Low // Audio events are low priority
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// System events for engine management
#[derive(Debug, Clone)]
pub enum SystemEvent {
    Shutdown { timestamp: Instant },
    Pause { timestamp: Instant },
    Resume { timestamp: Instant },
    SystemError { system_name: String, error: String, timestamp: Instant },
}

impl Event for SystemEvent {
    fn timestamp(&self) -> Instant {
        match self {
            SystemEvent::Shutdown { timestamp, .. } => *timestamp,
            SystemEvent::Pause { timestamp, .. } => *timestamp,
            SystemEvent::Resume { timestamp, .. } => *timestamp,
            SystemEvent::SystemError { timestamp, .. } => *timestamp,
        }
    }
    
    fn priority(&self) -> EventPriority {
        EventPriority::Critical // System events are critical
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
