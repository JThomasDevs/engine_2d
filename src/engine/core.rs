use std::time::{Duration, Instant};
use super::window::WindowManager;
use super::config::EngineConfig;
use crate::events::event_system::EventSystem;
use crate::events::event_types::RenderEvent;
use crate::render::renderer::Renderer;
use crate::render::gl_wrapper::GlWrapper;
use glam::Vec2;
#[cfg(feature = "glfw")]
use glfw::{Action, Key};

pub struct Engine {
    // Engine state
    is_running: bool,
    delta_time: Duration,
    last_frame_time: Instant,
    
    // OpenGL context is managed by the renderer
    
    // Window and input systems
    window_manager: WindowManager,
    config: EngineConfig,
    
    // Event system
    event_system: EventSystem,
    
    // Rendering system
    renderer: Renderer,
}

impl Engine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_config(EngineConfig::default())
    }
    
    pub fn new_with_config(config: EngineConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Create GlWrapper first
        let mut gl_wrapper = GlWrapper::new();
        
        // Create window manager with GlWrapper
        let window_manager = WindowManager::new(&config, &mut gl_wrapper)?;
        
        // Create event system
        let event_system = EventSystem::new();
        
        // Create renderer with GlWrapper
        let mut renderer = Renderer::new_with_gl(gl_wrapper);
        if let Err(e) = renderer.initialize() {
            return Err(format!("Failed to initialize renderer: {}", e).into());
        }
        
        Ok(Self {
            is_running: false,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            window_manager,
            config,
            event_system,
            renderer,
        })
    }
    
    // Getter methods for testing
    pub fn get_window_manager(&self) -> &WindowManager {
        &self.window_manager
    }
    
    pub fn get_config(&self) -> &EngineConfig {
        &self.config
    }
    
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
            
            // Process window events
            self.window_manager.poll_events();
            
            // Handle keyboard input for quit
            self.window_manager.process_events(|event| {
                match event {
                    #[cfg(feature = "glfw")]
                    super::window::WindowEvent::Glfw(glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _)) |
                    super::window::WindowEvent::Glfw(glfw::WindowEvent::Key(Key::Q, _, Action::Press, _)) => {
                        false // Return false to close window
                    }
                    _ => true, // Continue processing other events
                }
            });
            
            // Send render event to draw red rectangle
            let render_event = RenderEvent::DrawRectangle {
                x: 100.0,
                y: 100.0,
                width: 200.0,
                height: 150.0,
                color: (1.0, 0.0, 0.0), // Red
                timestamp: Instant::now(),
            };
            
            if let Err(e) = self.event_system.send_render_event(render_event) {
                eprintln!("Failed to send render event: {}", e);
            }
            
            // Render directly using the renderer
            if let Err(e) = self.renderer.clear(0.0, 0.0, 0.0, 1.0) {
                eprintln!("Renderer clear error: {}", e);
            }
            
            // Draw red rectangle in the center of the screen
            // Convert pixel coordinates to normalized coordinates (-1 to 1)
            let _window_size = self.window_manager.get_size();
            let center_x = 0.0; // Center horizontally
            let center_y = 0.0; // Center vertically
            let rect_width = 0.4;  // 40% of screen width
            let rect_height = 0.3; // 30% of screen height
            
            if let Err(e) = self.renderer.draw_rect(
                Vec2::new(center_x, center_y), 
                Vec2::new(rect_width, rect_height), 
                (1.0, 0.0, 0.0)
            ) {
                eprintln!("Renderer draw error: {}", e);
            } else {
                // Only print this once to avoid spam
                static mut PRINTED: bool = false;
                unsafe {
                    if !PRINTED {
                        println!("Successfully drew red rectangle at center with size (0.4, 0.3)");
                        PRINTED = true;
                    }
                }
            }
            
            // Swap buffers
            self.window_manager.swap_buffers();
        }
        
        println!("Engine shutting down...");
        Ok(())
    }
    
    pub fn quit(&mut self) {
        self.is_running = false;
        self.window_manager.request_close();
    }
}

// This allows Engine::try_from(config) syntax for fallible conversion
impl TryFrom<EngineConfig> for Engine {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: EngineConfig) -> Result<Self, Self::Error> {
        Self::new_with_config(config)
    }
}