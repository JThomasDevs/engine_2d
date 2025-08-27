use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub struct KeyboardInput {
    // Track if we should quit
    should_quit: bool,
}

impl KeyboardInput {
    pub fn new() -> Self {
        Self {
            should_quit: false,
        }
    }
    
    pub fn update(&mut self) {
        // Check for keyboard events (non-blocking)
        if event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        self.should_quit = true;
                    }
                    _ => {}
                }
            }
        }
    }
    
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    
    pub fn reset_quit_flag(&mut self) {
        self.should_quit = false;
    }
}
