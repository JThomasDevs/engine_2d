use std::time::{Duration, Instant};
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
    keyboard_input: KeyboardInput,
    config: EngineConfig,
}

impl Engine {
    pub fn new() -> Self {
        Self::new_with_config(EngineConfig::default())
    }
    
    pub fn new_with_config(config: EngineConfig) -> Self {
        let window_manager = WindowManager::new(&config);
        
        Self {
            is_running: false,
            delta_time: Duration::ZERO,
            last_frame_time: Instant::now(),
            window_manager,
            keyboard_input: KeyboardInput::new(),
            config,
        }
    }
    
    pub fn run(&mut self) {
        println!("Starting engine...");
        println!("Window: {} ({}x{})", 
                 self.window_manager.get_title(),
                 self.window_manager.get_size().0,
                 self.window_manager.get_size().1);
        println!("Press 'Q' or 'ESC' to quit");
        self.is_running = true;
        
        while self.is_running {
            self.update_timing();
            self.process_events();
            self.update();
            self.render();
            
            // Frame rate limiting
            if let Some(target_fps) = self.config.target_fps {
                let frame_time = Duration::from_secs_f32(1.0 / target_fps as f32);
                if self.delta_time < frame_time {
                    std::thread::sleep(frame_time - self.delta_time);
                }
            }
        }
        
        println!("Engine stopped.");
    }
    
    fn update_timing(&mut self) {
        let current_time = Instant::now();
        self.delta_time = current_time.duration_since(self.last_frame_time);
        self.last_frame_time = current_time;
    }
    
    fn process_events(&mut self) {
        // Update keyboard input
        self.keyboard_input.update();
        
        // Check if we should quit
        if self.keyboard_input.should_quit() || self.window_manager.should_close() {
            self.quit();
        }
    }
    
    fn update(&mut self) {
        // TODO: Update game logic here
    }
    
    fn render(&mut self) {
        // TODO: Render frame here
        // For now, just print a simple indicator
        if self.delta_time.as_millis() > 0 {
            let fps = 1000 / self.delta_time.as_millis();
            if self.config.show_fps {
                println!("Engine running - FPS: {}", fps);
            }
        }
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
