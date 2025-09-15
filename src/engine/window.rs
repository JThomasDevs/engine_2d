// TODO: Window Implementation Game Plan
// =====================================
//
// CURRENT STATUS:
// - Basic window creation with GLFW ✅
// - Cross-platform compatibility setup ✅
// - Window configuration system ✅
// - Event loop integration with Engine ✅
// - Basic window event handling (close, resize) ✅
// - Frame rate limiting ✅
// - Performance monitoring (FPS display) ✅
// - Window resizing support ✅
// - Safe OpenGL wrapper implementation ✅
// - OpenGL context creation integrated ✅
//
// NEXT STEPS (in order):
// 1. INTEGRATE RENDERER WITH ENGINE
//    - Add renderer to Engine struct
//    - Add rendering calls to game loop
//    - Create visual test example
//
// 2. IMPLEMENT BASIC RENDERING
//    - Create a simple renderer that can draw colored rectangles
//    - Add sprite rendering capabilities
//    - Implement basic texture loading
//
// 3. ENHANCE INPUT SYSTEM
//    - Integrate GLFW's keyboard/mouse events with existing input system
//    - Add mouse input handling
//    - Add gamepad support (optional)
//
// 4. ADD ADVANCED WINDOW FEATURES
//    - Fullscreen toggle
//    - VSync support
//    - Window positioning
//    - Error handling for window creation failures
//
// TECHNICAL NOTES:
// - Event loop is now fully integrated with Engine::run()
// - Window events (close, resize) are properly handled
// - Frame rate limiting and FPS monitoring are implemented
// - OpenGL context is created using GLFW + gl (unified approach)
// - All unsafe OpenGL code is contained in safe wrappers
// - Plan for WebAssembly support in future

#[cfg(feature = "glfw")]
use glfw::{Glfw, Context, WindowMode, WindowHint};
use super::config::EngineConfig;
use crate::render::gl_wrapper::GlWrapper;

// Common event enum that works across all feature configurations
#[derive(Debug, Clone)]
pub enum WindowEvent {
    #[cfg(feature = "glfw")]
    Glfw(glfw::WindowEvent),
    #[cfg(not(feature = "glfw"))]
    Noop,
}

#[cfg(feature = "glfw")]
pub struct WindowManager {
    pub glfw: Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub should_close: bool,
    pub title: String,
}

#[cfg(not(feature = "glfw"))]
pub struct WindowManager {
    pub should_close: bool,
    pub title: String,
}

impl WindowManager {
    #[cfg(feature = "glfw")]
    pub fn new(config: &EngineConfig, gl_wrapper: &mut GlWrapper) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Creating window: {}x{}", config.window_width, config.window_height);
        println!("Window title: {}", config.window_title);
        
        // Initialize GLFW
        let mut glfw = glfw::init(|_, _| {})?;
        
        // Configure GLFW for OpenGL
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        
        // Create window
        let (mut window, events) = glfw.create_window(
            config.window_width as u32,
            config.window_height as u32,
            &config.window_title,
            WindowMode::Windowed
        ).ok_or_else(|| {
            format!(
                "Failed to create GLFW window with parameters: width={}, height={}, title='{}'",
                config.window_width, config.window_height, config.window_title
            )
        })?;
        
        // Make the context current
        window.make_current();
        
        // Set up event callbacks
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);
        
        // Initialize the GlWrapper passed from Engine
        if let Err(e) = gl_wrapper.initialize(&mut window) {
            return Err(format!("Failed to initialize OpenGL context: {}", e).into());
        }
        println!("OpenGL context initialized successfully!");
        
        Ok(Self {
            glfw,
            window,
            events,
            should_close: false,
            title: config.window_title.clone(),
        })
    }

    #[cfg(not(feature = "glfw"))]
    pub fn new(config: &EngineConfig, _gl_wrapper: &mut GlWrapper) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Creating headless window manager for: {}", config.window_title);
        
        Ok(Self {
            should_close: false,
            title: config.window_title.clone(),
        })
    }
    
    pub fn request_close(&mut self) {
        self.should_close = true;
    }
    
    #[cfg(feature = "glfw")]
    pub fn should_close(&self) -> bool {
        self.should_close || self.window.should_close()
    }

    #[cfg(not(feature = "glfw"))]
    pub fn should_close(&self) -> bool {
        self.should_close
    }
    
    #[cfg(feature = "glfw")]
    pub fn get_size(&self) -> (u32, u32) {
        let (width, height) = self.window.get_framebuffer_size();
        (width as u32, height as u32)
    }

    #[cfg(not(feature = "glfw"))]
    pub fn get_size(&self) -> (u32, u32) {
        // Return default size for headless mode
        (800, 600)
    }
    
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    
    #[cfg(feature = "glfw")]
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.window.set_title(title);
    }

    #[cfg(not(feature = "glfw"))]
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    
    #[cfg(feature = "glfw")]
    pub fn poll_events(&mut self) {
        // Use poll_events for non-blocking event processing
        self.glfw.poll_events();
    }

    #[cfg(not(feature = "glfw"))]
    pub fn poll_events(&mut self) {
        // No-op for headless mode
    }
    
    #[cfg(feature = "glfw")]
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    #[cfg(not(feature = "glfw"))]
    pub fn swap_buffers(&mut self) {
        // No-op for headless mode
    }
    
    #[cfg(feature = "glfw")]
    pub fn process_events<F>(&mut self, mut callback: F)
    where
        F: FnMut(&WindowEvent) -> bool,
    {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Close => {
                    self.should_close = true;
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // Handle window resize - update viewport
                    // TODO: Send viewport update event to render system
                    println!("Window resized to {}x{}", width, height);
                }
                glfw::WindowEvent::Size(width, height) => {
                    // Handle window size change
                    println!("Window size changed to {}x{}", width, height);
                }
                _ => {
                    if !callback(&WindowEvent::Glfw(event)) {
                        self.should_close = true;
                    }
                }
            }
        }
    }

    #[cfg(not(feature = "glfw"))]
    pub fn process_events<F>(&mut self, _callback: F)
    where
        F: FnMut(&WindowEvent) -> bool,
    {
        // No-op for headless mode
    }
    
}
