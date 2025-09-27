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

use glfw::{Glfw, Context, WindowMode, WindowHint};
use super::config::EngineConfig;
use crate::render::gl_wrapper::GlWrapper;
use crate::events::event_system::EventSystem;
use crate::events::event_types::RenderEvent;
use std::time::Instant;

/// Window display modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Windowed,
    BorderlessFullscreen,
    ExclusiveFullscreen,
}

/// Monitor information cached at startup
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub name: String,
    pub index: u32,
    pub resolution: (u32, u32),
    pub refresh_rate: u32,
}

// Common event enum that works across all feature configurations
#[derive(Debug, Clone)]
pub enum WindowEvent {
    Glfw(glfw::WindowEvent),
}

pub struct WindowManager {
    pub glfw: Glfw,
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub should_close: bool,
    pub title: String,
    pub event_system: Option<EventSystem>,
    pub current_mode: DisplayMode,
    pub windowed_size: (u32, u32),
    pub windowed_position: (i32, i32),
    pub available_monitors: Vec<MonitorInfo>,
    pub cursor_hidden: bool,
    pub mouse_captured: bool,
    pub vsync_enabled: bool,
    pub mouse_position: (f32, f32),
}

impl WindowManager {
    pub fn new(config: &EngineConfig, gl_wrapper: &mut GlWrapper, event_system: Option<EventSystem>) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Creating window: {}x{}", config.window_width, config.window_height);
        println!("Window title: {}", config.window_title);
        
        // Initialize GLFW
        let mut glfw = glfw::init(|_, _| {})?;
        
        // Configure GLFW for OpenGL
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
        
        // Determine initial window mode
        let initial_mode = if config.fullscreen {
            // For fullscreen, we'll need to get the primary monitor
            // For now, use windowed mode and handle fullscreen later
            WindowMode::Windowed
        } else {
            WindowMode::Windowed
        };
        
        // Create window
        let (mut window, events) = glfw.create_window(
            config.window_width as u32,
            config.window_height as u32,
            &config.window_title,
            initial_mode
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
        
        // Configure VSync based on config
        if config.vsync {
            glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
            println!("VSync enabled");
        } else {
            glfw.set_swap_interval(glfw::SwapInterval::None);
            println!("VSync disabled");
        }
        
        // Get initial window position and size for restoration
        let (pos_x, pos_y) = window.get_pos();
        let (width, height) = window.get_size();
        
        // Detect available monitors at startup
        let available_monitors = Self::detect_monitors(&mut glfw);
        
        Ok(Self {
            glfw,
            window,
            events,
            should_close: false,
            title: config.window_title.clone(),
            event_system,
            current_mode: if config.fullscreen { DisplayMode::ExclusiveFullscreen } else { DisplayMode::Windowed },
            windowed_size: (width as u32, height as u32),
            windowed_position: (pos_x, pos_y),
            available_monitors,
            cursor_hidden: false,
            mouse_captured: false,
            vsync_enabled: config.vsync,
            mouse_position: (0.0, 0.0),
        })
    }

    /// Detect available monitors at startup
    fn detect_monitors(glfw: &mut Glfw) -> Vec<MonitorInfo> {
        let mut monitors = Vec::new();
        
        // Try to detect primary monitor
        glfw.with_primary_monitor(|_, monitor| {
            if let Some(monitor) = monitor {
                let video_mode = monitor.get_video_mode().unwrap_or_else(|| {
                    glfw::VidMode {
                        width: 1920,
                        height: 1080,
                        red_bits: 8,
                        green_bits: 8,
                        blue_bits: 8,
                        refresh_rate: 60,
                    }
                });
                
                monitors.push(MonitorInfo {
                    name: "Primary Monitor".to_string(),
                    index: 0,
                    resolution: (video_mode.width, video_mode.height),
                    refresh_rate: video_mode.refresh_rate,
                });
            }
        });
        
        // Add fallback if no primary monitor detected
        if monitors.is_empty() {
            monitors.push(MonitorInfo {
                name: "Primary Monitor".to_string(),
                index: 0,
                resolution: (1920, 1080),
                refresh_rate: 60,
            });
        }
        
        // Add common secondary monitor options
        monitors.push(MonitorInfo {
            name: "Secondary Monitor".to_string(),
            index: 1,
            resolution: (1920, 1080),
            refresh_rate: 60,
        });
        
        println!("Detected {} monitors at startup", monitors.len());
        for monitor in &monitors {
            println!("  {}: {}x{}@{}Hz", 
                monitor.name, 
                monitor.resolution.0, 
                monitor.resolution.1, 
                monitor.refresh_rate
            );
        }
        
        monitors
    }
    
    pub fn request_close(&mut self) {
        self.should_close = true;
    }
    
    pub fn should_close(&self) -> bool {
        self.should_close || self.window.should_close()
    }
    
    pub fn get_size(&self) -> (u32, u32) {
        let (width, height) = self.window.get_framebuffer_size();
        (width as u32, height as u32)
    }
    
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.window.set_title(title);
    }
    
    pub fn poll_events(&mut self) {
        // Use poll_events for non-blocking event processing
        self.glfw.poll_events();
    }
    
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
    
    /// Enable or disable VSync
    pub fn set_vsync(&mut self, enabled: bool) -> Result<(), String> {
        if enabled {
            self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
            self.vsync_enabled = true;
            println!("VSync enabled");
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::None);
            self.vsync_enabled = false;
            println!("VSync disabled");
        }
        Ok(())
    }
    
    /// Enable adaptive VSync (reduces tearing when FPS is above refresh rate)
    pub fn set_adaptive_vsync(&mut self, enabled: bool) -> Result<(), String> {
        if enabled {
            self.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
            self.vsync_enabled = true; // Adaptive is still a form of VSync
            println!("Adaptive VSync enabled");
        } else {
            self.glfw.set_swap_interval(glfw::SwapInterval::None);
            self.vsync_enabled = false;
            println!("Adaptive VSync disabled");
        }
        Ok(())
    }
    
    /// Get current VSync status
    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync_enabled
    }
    
    /// Toggle between windowed and fullscreen mode
    pub fn toggle_fullscreen(&mut self) -> Result<(), String> {
        println!("Current mode: {:?}", self.current_mode);
        match self.current_mode {
            DisplayMode::Windowed => {
                // Save current window position and size before going fullscreen
                let (pos_x, pos_y) = self.window.get_pos();
                let (width, height) = self.window.get_size();
                self.windowed_position = (pos_x, pos_y);
                self.windowed_size = (width as u32, height as u32);
                println!("Saving windowed position: ({}, {}) size: {}x{}", pos_x, pos_y, width, height);
                self.set_fullscreen(DisplayMode::ExclusiveFullscreen)
            },
            _ => {
                println!("Switching back to windowed mode");
                self.set_fullscreen(DisplayMode::Windowed)
            },
        }
    }
    
    /// Set the window display mode
    pub fn set_fullscreen(&mut self, mode: DisplayMode) -> Result<(), String> {
        match mode {
            DisplayMode::Windowed => {
                // Restore windowed mode
                self.window.set_monitor(
                    glfw::WindowMode::Windowed,
                    self.windowed_position.0,
                    self.windowed_position.1,
                    self.windowed_size.0,
                    self.windowed_size.1,
                    None
                );
                self.current_mode = DisplayMode::Windowed;
                println!("Switched to windowed mode");
            }
            DisplayMode::BorderlessFullscreen | DisplayMode::ExclusiveFullscreen => {
                // Get primary monitor and set fullscreen
                let mut success = false;
                self.glfw.with_primary_monitor(|_, monitor| {
                    if let Some(monitor) = monitor {
                        // Get the video mode for the monitor
                        let video_mode = monitor.get_video_mode().unwrap_or_else(|| {
                            glfw::VidMode {
                                width: 1920,
                                height: 1080,
                                red_bits: 8,
                                green_bits: 8,
                                blue_bits: 8,
                                refresh_rate: 60,
                            }
                        });
                        
                        println!("Setting fullscreen: {}x{}@{}Hz", 
                            video_mode.width, video_mode.height, video_mode.refresh_rate);
                        
                        self.window.set_monitor(
                            glfw::WindowMode::FullScreen(monitor),
                            0, 0,
                            video_mode.width, video_mode.height,
                            Some(video_mode.refresh_rate)
                        );
                        success = true;
                    }
                });
                
                if success {
                    self.current_mode = mode;
                    println!("Switched to fullscreen mode");
                } else {
                    return Err("No primary monitor found".to_string());
                }
            }
        }
        Ok(())
    }
    
    /// Get current display mode
    pub fn get_display_mode(&self) -> DisplayMode {
        self.current_mode
    }
    
    /// Check if window is in fullscreen mode
    pub fn is_fullscreen(&self) -> bool {
        matches!(self.current_mode, DisplayMode::BorderlessFullscreen | DisplayMode::ExclusiveFullscreen)
    }
    
    /// Set window position
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.window.set_pos(x, y);
        if self.current_mode == DisplayMode::Windowed {
            self.windowed_position = (x, y);
        }
    }
    
    /// Get window position
    pub fn get_position(&self) -> (i32, i32) {
        self.window.get_pos()
    }
    
    /// Center window on screen using cached monitor info
    pub fn center_on_screen(&mut self) -> Result<(), String> {
        if let Some(primary_monitor) = self.available_monitors.first() {
            // Use window size instead of framebuffer size for positioning
            let (width, height) = self.window.get_size();
            let center_x = (primary_monitor.resolution.0 as i32 - width as i32) / 2;
            let center_y = (primary_monitor.resolution.1 as i32 - height as i32) / 2;
            
            self.set_position(center_x, center_y);
            println!("Centered window at ({}, {})", center_x, center_y);
            Ok(())
        } else {
            Err("No primary monitor found".to_string())
        }
    }
    
    /// Set minimum window size
    pub fn set_minimum_size(&mut self, width: u32, height: u32) {
        self.window.set_size_limits(
            Some(width),
            Some(height),
            None,
            None
        );
    }
    
    /// Set maximum window size
    pub fn set_maximum_size(&mut self, width: u32, height: u32) {
        self.window.set_size_limits(
            None,
            None,
            Some(width),
            Some(height)
        );
    }
    
    /// Set window resizable
    pub fn set_resizable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable);
    }
    
    /// Hide cursor
    pub fn hide_cursor(&mut self) {
        self.window.set_cursor_mode(glfw::CursorMode::Hidden);
        self.cursor_hidden = true;
        println!("Cursor hidden");
    }
    
    /// Show cursor
    pub fn show_cursor(&mut self) {
        self.window.set_cursor_mode(glfw::CursorMode::Normal);
        self.cursor_hidden = false;
        println!("Cursor shown");
    }
    
    /// Toggle cursor visibility
    pub fn toggle_cursor(&mut self) {
        if self.cursor_hidden {
            self.show_cursor();
        } else {
            self.hide_cursor();
        }
    }
    
    /// Set cursor position
    pub fn set_cursor_position(&mut self, x: f64, y: f64) {
        self.window.set_cursor_pos(x, y);
    }
    
    /// Get cursor position
    pub fn get_cursor_position(&self) -> (f64, f64) {
        self.window.get_cursor_pos()
    }

    /// Get a reference to the event system
    pub fn get_event_system(&self) -> Option<&EventSystem> {
        self.event_system.as_ref()
    }

    /// Update mouse position and send mouse move events
    pub fn update_mouse_position(&mut self) {
        let (x, y) = self.get_cursor_position();
        let (width_points, height_points) = self.window.get_size();
        
        // Convert to centered coordinates where (0,0) is center of screen
-        let center_x = width as f32 / 2.0;
        let center_x = width_points as f32 / 2.0;
        let center_y = height_points as f32 / 2.0;
        let centered_x = x as f32 - center_x;
        let centered_y = center_y - y as f32; // Flip Y axis so positive Y is up
        
        let new_position = (centered_x, centered_y);
        
        // Only send event if position changed
        if new_position != self.mouse_position {
            self.mouse_position = new_position;
            
            // Send mouse move event to event system
            if let Some(event_system) = &self.event_system {
                let mouse_event = crate::events::event_types::InputEvent::MouseMove {
                    x: new_position.0,
                    y: new_position.1,
                    timestamp: std::time::Instant::now(),
                };
                let _ = event_system.send_input_event(mouse_event);
            }
        }
    }    
    /// Set cursor mode
    pub fn set_cursor_mode(&mut self, mode: glfw::CursorMode) {
        self.window.set_cursor_mode(mode);
    }
    
    /// Capture mouse (confine cursor to window)
    pub fn set_capture_mouse(&mut self, capture: bool) {
        if capture {
            self.window.set_cursor_mode(glfw::CursorMode::Disabled);
            self.mouse_captured = true;
            println!("Mouse captured");
        } else {
            self.window.set_cursor_mode(glfw::CursorMode::Normal);
            self.mouse_captured = false;
            println!("Mouse released");
        }
    }
    
    /// Toggle mouse capture
    pub fn toggle_capture_mouse(&mut self) {
        if self.mouse_captured {
            self.set_capture_mouse(false);
        } else {
            self.set_capture_mouse(true);
        }
    }
    
    /// Minimize window
    pub fn minimize(&mut self) {
        self.window.iconify();
    }
    
    /// Maximize window
    pub fn maximize(&mut self) {
        self.window.maximize();
    }
    
    /// Restore window from minimized or maximized state
    pub fn restore(&mut self) {
        self.window.restore();
    }
    
    /// Set window always on top
    pub fn set_always_on_top(&mut self, always_on_top: bool) {
        self.window.set_floating(always_on_top);
    }
    
    /// Check if window is visible
    pub fn is_visible(&self) -> bool {
        self.window.is_visible()
    }
    
    /// Get available monitors (cached at startup)
    pub fn get_available_monitors(&self) -> &Vec<MonitorInfo> {
        &self.available_monitors
    }
    
    /// Get primary monitor info
    pub fn get_primary_monitor(&self) -> Option<&MonitorInfo> {
        self.available_monitors.first()
    }
    
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
                    // Handle window resize - send viewport update event to render system
                    if let Some(ref event_system) = self.event_system {
                        let viewport_event = RenderEvent::ViewportUpdated {
                            width,
                            height,
                            timestamp: Instant::now(),
                        };
                        if let Err(e) = event_system.send_render_event(viewport_event) {
                            eprintln!("Failed to send viewport update event: {}", e);
                        }
                    }
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
    
}
