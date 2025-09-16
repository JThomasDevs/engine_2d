use crate::render::gl_service::{GlService, GlCommand, GlResult};
use crate::events::event_types::RenderEvent;
use crate::events::system_trait::{GameSystem, SystemError, SystemResult, SystemState, SystemPriority};
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use log::error;
#[cfg(feature = "gl")]
use gl;

// Shader type constants for non-GL builds
#[cfg(not(feature = "gl"))]
const GL_VERTEX_SHADER: u32 = 0x8B31;
#[cfg(not(feature = "gl"))]
const GL_FRAGMENT_SHADER: u32 = 0x8B30;

/// High-level rendering service that processes render events
pub struct RenderService {
    gl_service: GlService,
    event_sender: Option<Sender<RenderEvent>>,
    event_receiver: Option<Arc<Mutex<Receiver<RenderEvent>>>>,
    state: SystemState,
    
    // Rendering state
    basic_shader: Option<u32>,
    rect_vao: Option<u32>,
    rect_vbo: Option<u32>,
    initialized: bool,
    
    // Cached uniform locations
    uniform_rect_position: Option<i32>,
    uniform_rect_size: Option<i32>,
    uniform_color: Option<i32>,
}

impl RenderService {
    /// Create a new RenderService
    pub fn new() -> Self {
        Self {
            gl_service: GlService::new(),
            event_sender: None,
            event_receiver: None,
            state: SystemState::Uninitialized,
            basic_shader: None,
            rect_vao: None,
            rect_vbo: None,
            initialized: false,
            uniform_rect_position: None,
            uniform_rect_size: None,
            uniform_color: None,
        }
    }
    
    /// Create a new RenderService with event system
    pub fn new_with_event_system(event_sender: Sender<RenderEvent>, event_receiver: Arc<Mutex<Receiver<RenderEvent>>>) -> Self {
        Self {
            gl_service: GlService::new(),
            event_sender: Some(event_sender),
            event_receiver: Some(event_receiver),
            state: SystemState::Uninitialized,
            basic_shader: None,
            rect_vao: None,
            rect_vbo: None,
            initialized: false,
            uniform_rect_position: None,
            uniform_rect_size: None,
            uniform_color: None,
        }
    }
    
    /// Get the event sender for this service
    pub fn get_event_sender(&self) -> Option<Sender<RenderEvent>> {
        self.event_sender.clone()
    }
    
    /// Initialize the render service
    fn initialize_rendering(&mut self) -> SystemResult<()> {
        if self.initialized {
            return Ok(());
        }
        
        // Create basic shader
        #[cfg(feature = "gl")]
        let vertex_shader = self.create_shader(include_str!("shaders/basic.vert"), gl::VERTEX_SHADER)?;
        #[cfg(not(feature = "gl"))]
        let vertex_shader = self.create_shader(include_str!("shaders/basic.vert"), GL_VERTEX_SHADER)?;
        
        #[cfg(feature = "gl")]
        let fragment_shader = self.create_shader(include_str!("shaders/basic.frag"), gl::FRAGMENT_SHADER)?;
        #[cfg(not(feature = "gl"))]
        let fragment_shader = self.create_shader(include_str!("shaders/basic.frag"), GL_FRAGMENT_SHADER)?;
        let shader_program = self.create_shader_program(vertex_shader, fragment_shader)?;
        
        // Cache uniform locations
        self.cache_uniform_locations(shader_program)?;
        
        // Create rectangle geometry
        let (vao, vbo) = self.create_rectangle_geometry()?;
        
        self.basic_shader = Some(shader_program);
        self.rect_vao = Some(vao);
        self.rect_vbo = Some(vbo);
        self.initialized = true;
        
        Ok(())
    }
    
    /// Create a shader from source
    fn create_shader(&self, source: &str, shader_type: u32) -> SystemResult<u32> {
        // Send create shader command
        self.gl_service.send_command(GlCommand::CreateShader { shader_type })?;
        
        // Wait for result
        let result = self.gl_service.receive_result()?;
        match result {
            GlResult::ShaderCreated { shader } => {
                // Set shader source
                self.gl_service.send_command(GlCommand::SetShaderSource { 
                    shader, 
                    source: source.to_string() 
                })?;
                
                // Compile shader
                self.gl_service.send_command(GlCommand::CompileShader { shader })?;
                
                Ok(shader)
            }
            GlResult::Error { message } => Err(SystemError::ProcessingFailed(message)),
            _ => Err(SystemError::ProcessingFailed("Unexpected result".to_string())),
        }
    }
    
    /// Create a shader program
    fn create_shader_program(&self, vertex_shader: u32, fragment_shader: u32) -> SystemResult<u32> {
        // Create program
        self.gl_service.send_command(GlCommand::CreateProgram)?;
        let result = self.gl_service.receive_result()?;
        
        let program = match result {
            GlResult::ProgramCreated { program } => program,
            GlResult::Error { message } => return Err(SystemError::ProcessingFailed(message)),
            _ => return Err(SystemError::ProcessingFailed("Unexpected result".to_string())),
        };
        
        // Attach shaders
        self.gl_service.send_command(GlCommand::AttachShader { program, shader: vertex_shader })?;
        self.gl_service.send_command(GlCommand::AttachShader { program, shader: fragment_shader })?;
        
        // Link program
        self.gl_service.send_command(GlCommand::LinkProgram { program })?;
        
        // Clean up shaders
        self.gl_service.send_command(GlCommand::DeleteShader { shader: vertex_shader })?;
        self.gl_service.send_command(GlCommand::DeleteShader { shader: fragment_shader })?;
        
        Ok(program)
    }
    
    /// Helper function to fetch uniform location from shader program
    fn fetch_uniform_location(&mut self, program: u32, name: &str) -> SystemResult<i32> {
        self.gl_service.send_command(GlCommand::GetUniformLocation { 
            program, 
            name: name.to_string() 
        })?;
        let result = self.gl_service.receive_result()?;
        match result {
            GlResult::UniformLocation { location, .. } => {
                if location != -1 {
                    Ok(location)
                } else {
                    Err(SystemError::ProcessingFailed(format!("Uniform '{}' not found in shader", name)))
                }
            }
            GlResult::Error { message } => Err(SystemError::ProcessingFailed(message)),
            _ => Err(SystemError::ProcessingFailed("Unexpected result when getting uniform location".to_string())),
        }
    }

    /// Resolve and cache uniform locations for the shader program
    fn cache_uniform_locations(&mut self, program: u32) -> SystemResult<()> {
        self.uniform_rect_position = Some(self.fetch_uniform_location(program, "rect_position")?);
        self.uniform_rect_size = Some(self.fetch_uniform_location(program, "rect_size")?);
        self.uniform_color = Some(self.fetch_uniform_location(program, "color")?);
        
        Ok(())
    }
    
    /// Create rectangle geometry
    fn create_rectangle_geometry(&self) -> SystemResult<(u32, u32)> {
        // Generate VAO
        self.gl_service.send_command(GlCommand::GenVertexArray)?;
        let vao = match self.gl_service.receive_result()? {
            GlResult::VertexArrayGenerated { vao } => vao,
            _ => return Err(SystemError::ProcessingFailed("Failed to generate VAO".to_string())),
        };
        
        // Generate VBO
        self.gl_service.send_command(GlCommand::GenBuffer)?;
        let vbo = match self.gl_service.receive_result()? {
            GlResult::BufferGenerated { buffer } => buffer,
            _ => return Err(SystemError::ProcessingFailed("Failed to generate VBO".to_string())),
        };
        
        // Bind VAO and VBO
        self.gl_service.send_command(GlCommand::BindVertexArray { vao })?;
        self.gl_service.send_command(GlCommand::BindBuffer { target: 0x8892, buffer: vbo })?; // GL_ARRAY_BUFFER
        
        // Set vertex data
        let vertices = vec![
            -0.5, -0.5,  // bottom-left
             0.5, -0.5,  // bottom-right
            -0.5,  0.5,  // top-left
             0.5,  0.5,  // top-right
        ];
        
        self.gl_service.send_command(GlCommand::SetBufferData { 
            target: 0x8892, 
            data: vertices, 
            usage: 0x88E4 // GL_STATIC_DRAW
        })?;
        
        // Set vertex attributes
        self.gl_service.send_command(GlCommand::SetVertexAttribPointer { 
            index: 0, 
            size: 2, 
            data_type: 0x1406, // GL_FLOAT
            normalized: false, 
            stride: 8, 
            offset: 0 
        })?;
        
        self.gl_service.send_command(GlCommand::EnableVertexAttribArray { index: 0 })?;
        
        Ok((vao, vbo))
    }
    
    /// Process a render event
    fn process_render_event(&mut self, event: &RenderEvent) -> SystemResult<()> {
        match event {
            RenderEvent::ClearScreen { r, g, b, a, .. } => {
                self.gl_service.send_command(GlCommand::SetClearColor { r: *r, g: *g, b: *b, a: *a })?;
                self.gl_service.send_command(GlCommand::ClearColorBuffer)?;
            }
            
            RenderEvent::DrawRectangle { x, y, width, height, color, .. } => {
                if let (Some(shader), Some(vao)) = (self.basic_shader, self.rect_vao) {
                    // Use shader
                    self.gl_service.send_command(GlCommand::UseProgram { program: shader })?;
                    
                    // Set uniforms using cached locations
                    if let Some(location) = self.uniform_rect_position {
                        self.gl_service.send_command(GlCommand::SetUniform2f { 
                            location,
                            x: *x, y: *y 
                        })?;
                    } else {
                        return Err(SystemError::ProcessingFailed("Uniform 'rect_position' location not cached".to_string()));
                    }
                    
                    if let Some(location) = self.uniform_rect_size {
                        self.gl_service.send_command(GlCommand::SetUniform2f { 
                            location,
                            x: *width, y: *height 
                        })?;
                    } else {
                        return Err(SystemError::ProcessingFailed("Uniform 'rect_size' location not cached".to_string()));
                    }
                    
                    if let Some(location) = self.uniform_color {
                        self.gl_service.send_command(GlCommand::SetUniform3f { 
                            location,
                            x: color.0, y: color.1, z: color.2 
                        })?;
                    } else {
                        return Err(SystemError::ProcessingFailed("Uniform 'color' location not cached".to_string()));
                    }
                    
                    // Draw
                    self.gl_service.send_command(GlCommand::BindVertexArray { vao })?;
                    self.gl_service.send_command(GlCommand::DrawArrays { 
                        mode: 0x0005, // GL_TRIANGLE_STRIP
                        first: 0, 
                        count: 4 
                    })?;
                }
            }
            
            RenderEvent::PresentFrame { .. } => {
                // Frame presentation is handled by the window manager
            }
            
            _ => {
                // Handle other render events
            }
        }
        
        Ok(())
    }
}

impl GameSystem for RenderService {
    fn name(&self) -> &str {
        "RenderService"
    }
    
    fn priority(&self) -> SystemPriority {
        SystemPriority::Critical
    }
    
    fn state(&self) -> SystemState {
        self.state
    }
    
    fn initialize(&mut self) -> SystemResult<()> {
        self.state = SystemState::Initialized;
        self.initialize_rendering()?;
        self.state = SystemState::Running;
        Ok(())
    }
    
    fn shutdown(&mut self) -> SystemResult<()> {
        self.state = SystemState::Stopped;
        self.gl_service.shutdown();
        Ok(())
    }
    
    fn update(&mut self, _delta_time: Duration) -> SystemResult<()> {
        // Process all pending render events
        let events: Vec<RenderEvent> = {
            if let Some(ref receiver) = self.event_receiver {
                if let Ok(receiver) = receiver.lock() {
                    let mut events = Vec::new();
                    while let Ok(event) = receiver.try_recv() {
                        events.push(event);
                    }
                    events
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        };
        
        // Process events outside the lock
        for event in events {
            if let Err(e) = self.process_render_event(&event) {
                error!(
                    "RenderService failed to process render event: event_type={:?}, error={}",
                    event,
                    e
                );
            }
        }
        
        Ok(())
    }
    
    fn process_events(&mut self, events: &[Box<dyn crate::events::event_types::Event>]) -> SystemResult<()> {
        // Process events from the event system
        for event in events {
            if let Some(render_event) = event.as_any().downcast_ref::<RenderEvent>() {
                self.process_render_event(render_event)?;
            }
        }
        
        Ok(())
    }
    
    fn max_frame_time(&self) -> Duration {
        Duration::from_millis(8) // Critical system gets more time budget
    }
    
    fn can_run_parallel(&self) -> bool {
        false // Rendering must be single-threaded due to OpenGL context
    }
}
