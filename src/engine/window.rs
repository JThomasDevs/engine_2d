// TODO: Window Implementation Game Plan
// =====================================
//
// CURRENT STATUS:
// - Basic window creation with winit ✅
// - Cross-platform compatibility setup ✅
// - Window configuration system ✅
//
// NEXT STEPS (in order):
// 1. INTEGRATE EVENT LOOP WITH ENGINE
//    - Modify Engine::run() to use winit's event loop as the main loop
//    - Replace current game loop with winit's event-driven approach
//    - Handle window events (close, resize, focus) properly
//
// 2. ADD OPENGL CONTEXT
//    - Initialize OpenGL context for the window
//    - Set up basic rendering pipeline
//    - Create simple shaders for 2D rendering
//
// 3. IMPLEMENT BASIC RENDERING
//    - Create a simple renderer that can draw colored rectangles
//    - Add sprite rendering capabilities
//    - Implement basic texture loading
//
// 4. ENHANCE INPUT SYSTEM
//    - Integrate winit's keyboard/mouse events with existing input system
//    - Add mouse input handling
//    - Add gamepad support (optional)
//
// 5. ADD WINDOW FEATURES
//    - Window resizing support
//    - Fullscreen toggle
//    - VSync support
//    - Window positioning
//
// 6. OPTIMIZATION & POLISH
//    - Frame rate limiting
//    - Performance monitoring
//    - Error handling for window creation failures
//
// TECHNICAL NOTES:
// - Current approach creates window but doesn't run event loop
// - Need to restructure Engine to work with winit's event loop
// - Consider using glutin or gl-window for easier OpenGL integration
// - Plan for WebAssembly support in future

use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};
use super::config::EngineConfig;

pub struct WindowManager {
    pub window: Window,
    pub should_close: bool,
}

impl WindowManager {
    pub fn new(config: &EngineConfig) -> Self {
        println!("Creating window: {}x{}", config.window_width, config.window_height);
        println!("Window title: {}", config.window_title);
        
        // Create event loop and window
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(&config.window_title)
            .with_inner_size(LogicalSize::new(config.window_width, config.window_height))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();
        
        // For now, we'll just create the window but not run the event loop
        // This is a simplified approach - in a real engine you'd want proper event loop integration
        println!("Window created successfully!");
        
        Self {
            window,
            should_close: false,
        }
    }
    
    pub fn request_close(&mut self) {
        self.should_close = true;
    }
    
    pub fn should_close(&self) -> bool {
        self.should_close
    }
    
    pub fn get_size(&self) -> (u32, u32) {
        let size = self.window.inner_size();
        (size.width, size.height)
    }
    
    pub fn get_title(&self) -> String {
        self.window.title()
    }
}
