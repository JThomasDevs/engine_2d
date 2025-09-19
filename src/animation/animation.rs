#[cfg(feature = "opengl")]
use crate::render::sprite::SpriteRenderer;
#[cfg(feature = "opengl")]
use crate::engine::window::WindowEvent;

/// Trait for defining custom animations
/// 
/// This trait allows game makers to implement their own animation logic
/// without modifying the engine core. The engine will call update() each frame
/// with access to the sprite renderer (when available) and elapsed time, allowing you to create
/// and animate sprites as needed.
#[cfg(feature = "opengl")]
pub trait Animation {
    /// Update the animation
    /// 
    /// # Arguments
    /// * `sprite_renderer` - Optional access to sprite renderer for creating/rendering sprites (None in headless mode)
    /// * `elapsed_time` - Time in seconds since the animation started
    /// * `delta_time` - Time in seconds since the last frame
    fn update(&mut self, sprite_renderer: Option<&mut SpriteRenderer>, elapsed_time: f32, delta_time: f32);
    
    /// Handle input events
    /// 
    /// # Arguments
    /// * `event` - Window event (keyboard, mouse, etc.)
    fn handle_event(&mut self, event: &WindowEvent) {
        // Default implementation does nothing
        // Animations can override this to handle input
    }
    
    /// Get the name of the animation (for debugging/logging purposes)
    fn name(&self) -> &str;
}

#[cfg(not(feature = "opengl"))]
pub trait Animation {
    /// Update the animation (headless mode)
    /// 
    /// # Arguments
    /// * `elapsed_time` - Time in seconds since the animation started
    /// * `delta_time` - Time in seconds since the last frame
    fn update(&mut self, elapsed_time: f32, delta_time: f32);
    
    /// Get the name of the animation (for debugging/logging purposes)
    fn name(&self) -> &str;
}

/// A simple default animation that does nothing
/// 
/// This can be used as a placeholder or when you want sprites to remain static
pub struct NoAnimation {
    pub name: String,
}

impl NoAnimation {
    pub fn new() -> Self {
        Self {
            name: "No Animation".to_string(),
        }
    }
}

#[cfg(feature = "opengl")]
impl Animation for NoAnimation {
    fn update(&mut self, _sprite_renderer: Option<&mut SpriteRenderer>, _elapsed_time: f32, _delta_time: f32) {
        // Do nothing - no sprites are created or animated
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(not(feature = "opengl"))]
impl Animation for NoAnimation {
    fn update(&mut self, _elapsed_time: f32, _delta_time: f32) {
        // Do nothing - headless mode
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}
