use std::time::{Duration, Instant};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use glfw::{WindowEvent, Action, Key as GlfwKey};
use crate::input::KeyboardInput;
use crate::render::renderer::Renderer;
use super::window::WindowManager;
use super::config::EngineConfig;

#[derive(Debug)]
enum GameCommand {
    Quit,
}

#[derive(Debug)]
enum RenderCommand {
    Render,
}

pub struct Engine {
    // Engine state
    is_running: bool,
    delta_time: Duration,
    last_frame_time: Instant,
    
    // Window and input systems
    window_manager: WindowManager,
    keyboard_input: KeyboardInput,
    config: EngineConfig,
    
    // Rendering system
    renderer: Renderer,
}

impl Engine {
    pub fn new() -> Self {
        Self::new_with_config(EngineConfig::default())
    }
    
    pub fn new_with_config(config: EngineConfig) -> Self {
        let window_manager = WindowManager::new(&config);
        
        let renderer = Renderer::new();
        // TODO: Pass the GlWrapper from window_manager to renderer
        
        Self {
            is_running: false,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            window_manager,
            keyboard_input: KeyboardInput::new(),
            config,
            renderer,
        }
    }
    
    // Getter methods for testing
    pub fn get_window_manager(&self) -> &WindowManager {
        &self.window_manager
    }
    
    pub fn get_config(&self) -> &EngineConfig {
        &self.config
    }
    
    pub fn run(&mut self) {
        println!("Starting engine...");
        println!("Window: {} ({}x{})", 
                 self.window_manager.get_title(),
                 self.window_manager.get_size().0,
                 self.window_manager.get_size().1);
        println!("Press 'Q' or 'ESC' to quit");
        
        // Create shared shutdown flag
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown);
        
        // Create channels for communication between threads
        let (game_sender, game_receiver): (Sender<GameCommand>, Receiver<GameCommand>) = channel();
        let (render_sender, render_receiver): (Sender<RenderCommand>, Receiver<RenderCommand>) = channel();
        
        // Start game loop thread
        let config = self.config.clone();
        let game_thread = thread::spawn(move || {
            let mut delta_time = Duration::ZERO;
            let mut last_frame_time = Instant::now();
            
            while !shutdown_clone.load(Ordering::Relaxed) {
                // Update timing
                let current_time = Instant::now();
                delta_time = current_time.duration_since(last_frame_time);
                last_frame_time = current_time;
                
                // Update game logic
                // TODO: Call existing update method
                
                // Send render command
                if render_sender.send(RenderCommand::Render).is_err() {
                    break; // Main thread closed
                }
                
                // Frame rate limiting and FPS calculation
                if let Some(target_fps) = config.target_fps {
                    let frame_time = Duration::from_secs_f32(1.0 / target_fps as f32);
                    if delta_time < frame_time {
                        thread::sleep(frame_time - delta_time);
                        // Use target FPS for display when limiting
                        if config.show_fps {
                            println!("Engine running - FPS: {}", target_fps);
                        }
                    } else {
                        // Use actual FPS when not limiting
                        if delta_time.as_millis() > 0 && config.show_fps {
                            let fps = 1000 / delta_time.as_millis();
                            println!("Engine running - FPS: {}", fps);
                        }
                    }
                } else {
                    // No frame rate limiting, use actual FPS
                    if delta_time.as_millis() > 0 && config.show_fps {
                        let fps = 1000 / delta_time.as_millis();
                        println!("Engine running - FPS: {}", fps);
                    }
                }
                
                // Small delay to prevent busy waiting
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        // Main thread handles GLFW events and rendering
        while !self.window_manager.should_close() {
            // Poll GLFW events (this can block, but that's OK in main thread)
            self.window_manager.poll_events();
            
            // Process events
            let mut should_close = false;
            self.window_manager.process_events(|event| {
                match event {
                    WindowEvent::Key(GlfwKey::Escape, _, Action::Press, _) |
                    WindowEvent::Key(GlfwKey::Q, _, Action::Press, _) => {
                        should_close = true;
                        false
                    }
                    _ => true,
                }
            });
            
            if should_close {
                shutdown.store(true, Ordering::Relaxed);
                self.window_manager.request_close();
                break; // Exit immediately after sending quit command
            }
            
            // Handle render commands from game thread
            if let Ok(cmd) = render_receiver.try_recv() {
                match cmd {
                    RenderCommand::Render => {
                        // TODO: Add actual rendering calls here when OpenGL context is ready
                        self.window_manager.swap_buffers();
                    }
                }
            }
        }
        
        // Signal shutdown and wait briefly for game thread
        shutdown.store(true, Ordering::Relaxed);
        
        // Wait for game thread with timeout
        let timeout = Duration::from_millis(100);
        let start = Instant::now();
        while start.elapsed() < timeout {
            if game_thread.is_finished() {
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        
        println!("Engine stopped.");
    }
    
    pub fn quit(&mut self) {
        self.is_running = false;
        self.window_manager.request_close();
    }
}

// This allows Engine::from(config) syntax
impl From<EngineConfig> for Engine {
    fn from(config: EngineConfig) -> Self {
        Self::new_with_config(config)
    }
}
