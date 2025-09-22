use std::time::{Duration, Instant};
#[cfg(feature = "opengl")]
use std::rc::Rc;
#[cfg(feature = "opengl")]
use super::window::WindowManager;
use super::config::EngineConfig;
#[cfg(feature = "opengl")]
use crate::events::event_system::EventSystem;
#[cfg(feature = "opengl")]
use crate::render::renderer::Renderer;
#[cfg(feature = "opengl")]
use crate::render::sprite::SpriteRenderer;
    #[cfg(feature = "opengl")]
    use crate::render::simple_text::SimpleTextRenderer;
#[cfg(feature = "opengl")]
use crate::render::gl_wrapper::GlWrapper;
use crate::animation::Animation;
#[cfg(feature = "opengl")]
use glfw::{Action, Key};

pub struct Engine {
    // Engine state
    is_running: bool,
    // Frame timing for device-agnostic animations (used in run() methods)
    #[allow(dead_code)] // False positive: fields are used in conditional compilation blocks
    delta_time: Duration,
    #[allow(dead_code)] // False positive: fields are used in conditional compilation blocks
    last_frame_time: Instant,
    // Total elapsed time since engine start (accumulated from delta_time)
    elapsed_time: f32,
    
    // OpenGL context is managed by the renderer
    
    // Window and input systems
    #[cfg(feature = "opengl")]
    window_manager: WindowManager,
    config: EngineConfig,
    
    // Rendering system
    #[cfg(feature = "opengl")]
    renderer: Renderer,
    #[cfg(feature = "opengl")]
    sprite_renderer: SpriteRenderer,
    #[cfg(feature = "opengl")]
    text_renderer: SimpleTextRenderer,
    
    // Current animation
    animation: Box<dyn Animation>,
}

impl Engine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_config_and_animation(EngineConfig::default(), Box::new(crate::animation::NoAnimation::new()))
    }
    
    pub fn new_with_config(config: EngineConfig) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_config_and_animation(config, Box::new(crate::animation::NoAnimation::new()))
    }
    
    #[cfg(feature = "opengl")]
    pub fn new_with_config_and_animation(config: EngineConfig, animation: Box<dyn Animation>) -> Result<Self, Box<dyn std::error::Error>> {
        // Create GlWrapper first
        let mut gl_wrapper = GlWrapper::new();
        
        // Create event system for window manager
        let event_system = EventSystem::new();
        
        // Create window manager with GlWrapper and event system
        let window_manager = WindowManager::new(&config, &mut gl_wrapper, Some(event_system))?;
        
        // Wrap GlWrapper in Rc for shared ownership
        let gl_wrapper_rc = Rc::new(gl_wrapper);
        
        // Create renderer with shared GlWrapper
        let mut renderer = Renderer::new_with_gl(Rc::clone(&gl_wrapper_rc));
        if let Err(e) = renderer.initialize() {
            return Err(format!("Failed to initialize renderer: {}", e).into());
        }
        
        // Create sprite renderer with the same shared GlWrapper
        let mut sprite_renderer = SpriteRenderer::new(Rc::clone(&gl_wrapper_rc));
        if let Err(e) = sprite_renderer.initialize() {
            return Err(format!("Failed to initialize sprite renderer: {}", e).into());
        }
        
        // Create text renderer with the same shared GlWrapper
        let mut text_renderer = SimpleTextRenderer::new(Rc::clone(&gl_wrapper_rc), config.fallback_font_path.clone())?;
        if let Err(e) = text_renderer.initialize() {
            return Err(format!("Failed to initialize text renderer: {}", e).into());
        }
        
        // Configure viewport for text rendering using the config
        let viewport_config = &config.viewport;
        text_renderer.viewport_mut().logical_bounds = viewport_config.logical_bounds;
        
        // Set text height fraction from config
        if let Err(e) = text_renderer.viewport_mut().set_text_height_fraction(viewport_config.text_height_fraction) {
            println!("Warning: Failed to set text height fraction: {}", e);
        }
        
        // Set base font size from config
        if let Err(e) = text_renderer.viewport_mut().set_base_font_size(viewport_config.base_font_size) {
            println!("Warning: Failed to set base font size: {}", e);
        }
        
        // Set viewport independence from config
        text_renderer.set_viewport_independent_text(viewport_config.viewport_independent_text);
        
        Ok(Self {
            is_running: false,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            elapsed_time: 0.0,
            window_manager,
            config,
            renderer,
            sprite_renderer,
            text_renderer,
            animation,
        })
    }
    
    #[cfg(not(feature = "opengl"))]
    pub fn new_with_config_and_animation(config: EngineConfig, animation: Box<dyn Animation>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            is_running: false,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            elapsed_time: 0.0,
            config,
            animation,
        })
    }
    
    // Getter methods for testing
    #[cfg(feature = "opengl")]
    pub fn get_window_manager(&self) -> &WindowManager {
        &self.window_manager
    }
    
    #[cfg(feature = "opengl")]
    pub fn get_window_manager_mut(&mut self) -> &mut WindowManager {
        &mut self.window_manager
    }
    
    pub fn get_config(&self) -> &EngineConfig {
        &self.config
    }
    
    /// Get access to the sprite renderer for creating sprites
    #[cfg(feature = "opengl")]
    pub fn get_sprite_renderer(&mut self) -> &mut SpriteRenderer {
        &mut self.sprite_renderer
    }
    
    #[cfg(feature = "opengl")]
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting engine...");
        println!("Window: {} ({}x{})", 
                 self.window_manager.get_title(),
                 self.window_manager.get_size().0,
                 self.window_manager.get_size().1);
        println!("Press 'Q' or 'ESC' to quit");
        
        // Renderer is already initialized in the constructor
        
        // Main game loop
        while !self.window_manager.should_close() {
            // Update timing
            let current_time = Instant::now();
            self.delta_time = current_time.duration_since(self.last_frame_time);
            self.last_frame_time = current_time;
            
            // Accumulate delta time for animations (total elapsed time since start)
            self.elapsed_time += self.delta_time.as_secs_f32();
            
            // Process window events
            self.window_manager.poll_events();
            
            // Handle keyboard input for quit and forward other events to animation
            self.window_manager.process_events(|event| {
                match event {
                    super::window::WindowEvent::Glfw(glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _)) |
                    super::window::WindowEvent::Glfw(glfw::WindowEvent::Key(Key::Q, _, Action::Press, _)) => {
                        false // Return false to close window
                    }
                    _ => {
                        // Forward all other events to the animation
                        self.animation.handle_event(event);
                        true // Continue processing other events
                    }
                }
            });
            
            // Clear screen with dark background
            if let Err(e) = self.renderer.clear(0.1, 0.1, 0.1, 1.0) {
                eprintln!("Renderer clear error: {}", e);
            }
            
            // Update animation (animation is responsible for creating and rendering sprites and text)
            self.animation.update(Some(&mut self.sprite_renderer), self.elapsed_time, self.delta_time.as_secs_f32(), Some(&mut self.window_manager), Some(&mut self.text_renderer));
            
            // Print success message once
            static PRINTED: std::sync::Once = std::sync::Once::new();
            PRINTED.call_once(|| {
                println!("Successfully running animation: {}", self.animation.name());
            });
            
            // Swap buffers
            self.window_manager.swap_buffers();
        }
        
        println!("Engine shutting down...");
        Ok(())
    }
    
    #[cfg(not(feature = "opengl"))]
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting headless engine...");
        println!("Running animation: {}", self.animation.name());
        
        self.is_running = true;
        
        // Simple headless game loop - just run the animation logic
        let mut last_frame_time = Instant::now();
        let mut frame_count = 0;
        
        while self.is_running && frame_count < 1000 { // Limit frames for headless mode
            // Update timing for frame-independent animation
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(last_frame_time);
            last_frame_time = current_time;
            
            // Accumulate delta time for animations (total elapsed time since start)
            self.elapsed_time += delta_time.as_secs_f32();
            
            // Update animation (headless mode - no rendering)
            // Note: In headless mode, animations can still process game logic
            // but won't render anything
            self.animation.update(self.elapsed_time, delta_time.as_secs_f32());
            
            frame_count += 1;
            
            // Small delay to prevent busy waiting
            std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
        }
        
        println!("Headless engine shutting down...");
        Ok(())
    }
    
    #[cfg(feature = "opengl")]
    pub fn quit(&mut self) {
        self.is_running = false;
        self.window_manager.request_close();
    }
    
    #[cfg(not(feature = "opengl"))]
    pub fn quit(&mut self) {
        self.is_running = false;
    }
    
    /// Get a reference to the text renderer
    #[cfg(feature = "opengl")]
    pub fn text_renderer(&self) -> &SimpleTextRenderer {
        &self.text_renderer
    }
    
    /// Get a mutable reference to the text renderer
    #[cfg(feature = "opengl")]
    pub fn text_renderer_mut(&mut self) -> &mut SimpleTextRenderer {
        &mut self.text_renderer
    }
}

// This allows Engine::try_from(config) syntax for fallible conversion
impl TryFrom<EngineConfig> for Engine {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: EngineConfig) -> Result<Self, Self::Error> {
        Self::new_with_config(config)
    }
}