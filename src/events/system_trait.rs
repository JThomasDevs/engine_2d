use crate::events::event_types::*;
use std::time::Duration;

/// Result type for system operations
pub type SystemResult<T> = Result<T, SystemError>;

/// Errors that can occur in game systems
#[derive(Debug, Clone)]
pub enum SystemError {
    InitializationFailed(String),
    ProcessingFailed(String),
    ResourceNotFound(String),
    InvalidState(String),
    ThreadingError(String),
}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::InitializationFailed(msg) => write!(f, "Initialization failed: {msg}"),
            SystemError::ProcessingFailed(msg) => write!(f, "Processing failed: {msg}"),
            SystemError::ResourceNotFound(msg) => write!(f, "Resource not found: {msg}"),
            SystemError::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            SystemError::ThreadingError(msg) => write!(f, "Threading error: {msg}"),
        }
    }
}

impl std::error::Error for SystemError {}

impl From<String> for SystemError {
    fn from(error: String) -> Self {
        SystemError::ProcessingFailed(error)
    }
}

/// System priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// System state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    Uninitialized,
    Initialized,
    Running,
    Paused,
    Stopped,
    Error,
}

/// Trait that all game systems must implement
pub trait GameSystem: Send + Sync {
    /// Get the name of this system
    fn name(&self) -> &str;

    /// Get the priority of this system
    fn priority(&self) -> SystemPriority {
        SystemPriority::Normal
    }

    /// Get the current state of this system
    fn state(&self) -> SystemState;

    /// Initialize the system
    fn initialize(&mut self) -> SystemResult<()>;

    /// Shutdown the system
    fn shutdown(&mut self) -> SystemResult<()>;

    /// Update the system (called every frame)
    fn update(&mut self, delta_time: Duration) -> SystemResult<()>;

    /// Process events for this system
    fn process_events(&mut self, events: &[Box<dyn Event>]) -> SystemResult<()>;

    /// Get the maximum time this system should take per frame (for performance monitoring)
    fn max_frame_time(&self) -> Duration {
        Duration::from_millis(16) // Default to 60 FPS budget
    }

    /// Check if this system can run in parallel with other systems
    fn can_run_parallel(&self) -> bool {
        true
    }

    /// Get dependencies (systems that must run before this one)
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Helper macro to implement common system functionality
#[macro_export]
macro_rules! impl_game_system {
    ($struct_name:ident, $system_name:expr, $priority:expr) => {
        impl GameSystem for $struct_name {
            fn name(&self) -> &str {
                $system_name
            }

            fn priority(&self) -> SystemPriority {
                $priority
            }

            fn state(&self) -> SystemState {
                self.state
            }

            fn max_frame_time(&self) -> Duration {
                Duration::from_millis(16)
            }
        }
    };
}
