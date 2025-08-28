use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use winit::event_loop::EventLoop;
use winit::event::{Event, WindowEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::input::KeyboardInput;
use super::window::WindowManager;
use super::config::EngineConfig;



pub struct Engine {
    // Engine state
    is_running: bool,
    delta_time: Duration,
    last_frame_time: Instant,
    
    // Window and input systems
    window_manager: WindowManager,
    event_loop: Option<EventLoop<()>>,
    keyboard_input: KeyboardInput,
    config: EngineConfig,
}

impl Engine {
    pub fn new() -> Self {
        Self::new_with_config(EngineConfig::default())
    }
    
    pub fn new_with_config(config: EngineConfig) -> Self {
        let (window_manager, event_loop) = WindowManager::new(&config);
        
        Self {
            is_running: false,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            window_manager,
            event_loop: Some(event_loop),
            keyboard_input: KeyboardInput::new(),
            config,
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
        
        // Create shared state for the event loop
        let is_running = Arc::new(Mutex::new(true));
        let delta_time = Arc::new(Mutex::new(Duration::ZERO));
        let last_frame_time = Arc::new(Mutex::new(Instant::now()));
        let config = self.config.clone();
        
        // Run the event loop
        if let Some(event_loop) = self.event_loop.take() {
            let is_running_clone = Arc::clone(&is_running);
            let delta_time_clone = Arc::clone(&delta_time);
            let last_frame_time_clone = Arc::clone(&last_frame_time);
            
            WindowManager::run_event_loop(event_loop, move |event| {
                // Handle different event types
                match event {
                    Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                        *is_running_clone.lock().unwrap() = false;
                    }
                    Event::WindowEvent { event: WindowEvent::KeyboardInput { event: winit::event::KeyEvent { physical_key, state: key_state, .. }, .. }, .. } => {
                        // Handle keyboard input
                        match (physical_key, key_state) {
                            (PhysicalKey::Code(KeyCode::KeyQ), ElementState::Pressed) |
                            (PhysicalKey::Code(KeyCode::Escape), ElementState::Pressed) => {
                                *is_running_clone.lock().unwrap() = false;
                            }
                            _ => {}
                        }
                    }
                    Event::AboutToWait => {
                        // This is called before the event loop waits for events
                        // We can use this for our game loop
                        let running = *is_running_clone.lock().unwrap();
                        if running {
                            // Update timing
                            let current_time = Instant::now();
                            let mut last_time = last_frame_time_clone.lock().unwrap();
                            let mut delta = delta_time_clone.lock().unwrap();
                            *delta = current_time.duration_since(*last_time);
                            *last_time = current_time;
                            drop(last_time); // Release lock early
                            drop(delta); // Release lock early
                            
                            // Process events (simplified for now)
                            // TODO: Integrate with existing input system
                            
                            // Update game logic
                            // TODO: Call existing update method
                            
                            // Render frame
                            let delta = *delta_time_clone.lock().unwrap();
                            if delta.as_millis() > 0 {
                                let fps = 1000 / delta.as_millis();
                                if config.show_fps {
                                    println!("Engine running - FPS: {}", fps);
                                }
                            }
                            
                            // Frame rate limiting
                            if let Some(target_fps) = config.target_fps {
                                let frame_time = Duration::from_secs_f32(1.0 / target_fps as f32);
                                let delta = *delta_time_clone.lock().unwrap();
                                if delta < frame_time {
                                    std::thread::sleep(frame_time - delta);
                                }
                            }
                        }
                    }
                    _ => {
                        // Handle other events if needed
                    }
                }
                
                *is_running_clone.lock().unwrap()
            });
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
