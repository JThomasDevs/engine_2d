use crate::render::gl_wrapper::GlWrapper;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// OpenGL commands that can be sent to the GlService
#[derive(Debug, Clone)]
pub enum GlCommand {
    // Initialization
    Initialize { window_handle: usize }, // We'll need to handle this differently
    
    // Viewport operations
    SetViewport { x: i32, y: i32, width: i32, height: i32 },
    
    // Clear operations
    SetClearColor { r: f32, g: f32, b: f32, a: f32 },
    ClearColorBuffer,
    
    // Blending
    EnableBlending,
    SetBlendFunc { src: u32, dst: u32 },
    
    // Shader operations
    UseProgram { program: u32 },
    SetUniform2f { location: i32, x: f32, y: f32 },
    SetUniform3f { location: i32, x: f32, y: f32, z: f32 },
    GetUniformLocation { program: u32, name: String },
    
    // Geometry operations
    BindVertexArray { vao: u32 },
    DrawArrays { mode: u32, first: i32, count: i32 },
    
    // Resource management
    CreateShader { shader_type: u32 },
    SetShaderSource { shader: u32, source: String },
    CompileShader { shader: u32 },
    CreateProgram,
    AttachShader { program: u32, shader: u32 },
    LinkProgram { program: u32 },
    DeleteShader { shader: u32 },
    DeleteProgram { program: u32 },
    
    // Buffer operations
    GenVertexArray,
    GenBuffer,
    BindBuffer { target: u32, buffer: u32 },
    SetBufferData { target: u32, data: Vec<f32>, usage: u32 },
    SetVertexAttribPointer { index: u32, size: i32, data_type: u32, normalized: bool, stride: i32, offset: usize },
    EnableVertexAttribArray { index: u32 },
    DeleteVertexArray { vao: u32 },
    DeleteBuffer { buffer: u32 },
    
    // Shutdown
    Shutdown,
}

/// Results from GlService operations
#[derive(Debug, Clone)]
pub enum GlResult {
    // Success cases
    Ok,
    ViewportSet,
    ClearColorSet,
    ColorBufferCleared,
    BlendingEnabled,
    BlendFuncSet,
    ProgramUsed { program: u32 },
    Uniform2fSet { location: i32, x: f32, y: f32 },
    Uniform3fSet { location: i32, x: f32, y: f32, z: f32 },
    UniformLocation { program: u32, name: String, location: i32 },
    VertexArrayBound { vao: u32 },
    ArraysDrawn { mode: u32, first: i32, count: i32 },
    
    // Resource creation
    ShaderCreated { shader: u32 },
    ShaderSourceSet { shader: u32 },
    ShaderCompiled { shader: u32 },
    ProgramCreated { program: u32 },
    ShaderAttached { program: u32, shader: u32 },
    ProgramLinked { program: u32 },
    ShaderDeleted { shader: u32 },
    ProgramDeleted { program: u32 },
    
    // Buffer operations
    VertexArrayGenerated { vao: u32 },
    BufferGenerated { buffer: u32 },
    BufferBound { target: u32, buffer: u32 },
    BufferDataSet { target: u32, data_len: usize },
    VertexAttribPointerSet { index: u32 },
    VertexAttribArrayEnabled { index: u32 },
    VertexArrayDeleted { vao: u32 },
    BufferDeleted { buffer: u32 },
    
    // Error cases
    Error { message: String },
    NotInitialized,
    InvalidShader,
    InvalidProgram,
    InvalidBuffer,
    InvalidVertexArray,
}

/// High-performance OpenGL service that processes commands asynchronously
pub struct GlService {
    command_sender: Sender<GlCommand>,
    result_receiver: Arc<Mutex<Receiver<GlResult>>>,
    running: Arc<AtomicBool>,
}

impl GlService {
    /// Create a new GlService
    pub fn new() -> Self {
        let (command_sender, command_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        let running = Arc::new(AtomicBool::new(true));
        
        // Start the OpenGL processing thread
        let running_clone = Arc::clone(&running);
        thread::spawn(move || {
            let gl_wrapper = GlWrapper::new();
            let mut initialized = false;
            
            while running_clone.load(Ordering::Relaxed) {
                // Process commands
                while let Ok(command) = command_receiver.try_recv() {
                    let result = match command {
                        GlCommand::Initialize { window_handle: _ } => {
                            // TODO: Handle window initialization properly
                            if !initialized {
                                // For now, just mark as initialized
                                initialized = true;
                                GlResult::Ok
                            } else {
                                GlResult::Error { message: "Already initialized".to_string() }
                            }
                        }
                        
                        GlCommand::SetViewport { x, y, width, height } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_viewport(x, y, width, height) {
                                    Ok(_) => GlResult::ViewportSet,
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::SetClearColor { r, g, b, a } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_clear_color(r, g, b, a) {
                                    Ok(_) => GlResult::ClearColorSet,
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::ClearColorBuffer => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.clear_color_buffer() {
                                    Ok(_) => GlResult::ColorBufferCleared,
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::EnableBlending => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.enable_blending() {
                                    Ok(_) => GlResult::BlendingEnabled,
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::SetBlendFunc { src, dst } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_blend_func(src, dst) {
                                    Ok(_) => GlResult::BlendFuncSet,
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::UseProgram { program } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.use_program(program) {
                                    Ok(_) => GlResult::ProgramUsed { program },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::SetUniform2f { location, x, y } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_uniform_2f(location, x, y) {
                                    Ok(_) => GlResult::Uniform2fSet { location, x, y },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::SetUniform3f { location, x, y, z } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_uniform_3f(location, x, y, z) {
                                    Ok(_) => GlResult::Uniform3fSet { location, x, y, z },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::GetUniformLocation { program, name } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.get_uniform_location(program, &name) {
                                    Ok(location) => GlResult::UniformLocation { program, name, location },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::BindVertexArray { vao } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.bind_vertex_array(vao) {
                                    Ok(_) => GlResult::VertexArrayBound { vao },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::DrawArrays { mode, first, count } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.draw_arrays(mode, first, count) {
                                    Ok(_) => GlResult::ArraysDrawn { mode, first, count },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::CreateShader { shader_type } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.create_shader(shader_type) {
                                    Ok(shader) => GlResult::ShaderCreated { shader },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::SetShaderSource { shader, source } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_shader_source(shader, &source) {
                                    Ok(_) => GlResult::ShaderSourceSet { shader },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::CompileShader { shader } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.compile_shader(shader) {
                                    Ok(_) => GlResult::ShaderCompiled { shader },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::CreateProgram => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.create_program() {
                                    Ok(program) => GlResult::ProgramCreated { program },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::AttachShader { program, shader } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.attach_shader(program, shader) {
                                    Ok(_) => GlResult::ShaderAttached { program, shader },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::LinkProgram { program } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.link_program(program) {
                                    Ok(_) => GlResult::ProgramLinked { program },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::DeleteShader { shader } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.delete_shader(shader) {
                                    Ok(_) => GlResult::ShaderDeleted { shader },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::DeleteProgram { program } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.delete_program(program) {
                                    Ok(_) => GlResult::ProgramDeleted { program },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::GenVertexArray => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.gen_vertex_array() {
                                    Ok(vao) => GlResult::VertexArrayGenerated { vao },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::GenBuffer => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.gen_buffer() {
                                    Ok(buffer) => GlResult::BufferGenerated { buffer },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::BindBuffer { target, buffer } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.bind_buffer(target, buffer) {
                                    Ok(_) => GlResult::BufferBound { target, buffer },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::SetBufferData { target, data, usage } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_buffer_data(target, &data, usage) {
                                    Ok(_) => GlResult::BufferDataSet { target, data_len: data.len() },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::SetVertexAttribPointer { index, size, data_type, normalized, stride, offset } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.set_vertex_attrib_pointer(index, size, data_type, normalized, stride, offset) {
                                    Ok(_) => GlResult::VertexAttribPointerSet { index },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::EnableVertexAttribArray { index } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.enable_vertex_attrib_array(index) {
                                    Ok(_) => GlResult::VertexAttribArrayEnabled { index },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::DeleteVertexArray { vao } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.delete_vertex_array(vao) {
                                    Ok(_) => GlResult::VertexArrayDeleted { vao },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::DeleteBuffer { buffer } => {
                            if !initialized {
                                GlResult::NotInitialized
                            } else {
                                match gl_wrapper.delete_buffer(buffer) {
                                    Ok(_) => GlResult::BufferDeleted { buffer },
                                    Err(e) => GlResult::Error { message: e },
                                }
                            }
                        }
                        
                        GlCommand::Shutdown => {
                            running_clone.store(false, Ordering::Relaxed);
                            break;
                        }
                    };
                    
                    // Send result back
                    if let Err(_) = result_sender.send(result) {
                        break; // Receiver dropped
                    }
                }
                
                // Small sleep to prevent busy waiting
                thread::sleep(std::time::Duration::from_micros(100));
            }
        });
        
        Self {
            command_sender,
            result_receiver: Arc::new(Mutex::new(result_receiver)),
            running,
        }
    }
    
    /// Send a command to the GlService
    pub fn send_command(&self, command: GlCommand) -> Result<(), String> {
        self.command_sender.send(command).map_err(|_| {
            "Failed to send command to GlService".to_string()
        })
    }
    
    /// Try to receive a result (non-blocking)
    pub fn try_receive_result(&self) -> Option<GlResult> {
        if let Ok(receiver) = self.result_receiver.lock() {
            receiver.try_recv().ok()
        } else {
            None
        }
    }
    
    /// Receive a result (blocking)
    pub fn receive_result(&self) -> Result<GlResult, String> {
        if let Ok(receiver) = self.result_receiver.lock() {
            receiver.recv().map_err(|_| {
                "Failed to receive result from GlService".to_string()
            })
        } else {
            Err("Failed to lock result receiver".to_string())
        }
    }
    
    /// Check if the service is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
    
    /// Shutdown the service
    pub fn shutdown(&self) {
        let _ = self.send_command(GlCommand::Shutdown);
    }
}

impl Drop for GlService {
    fn drop(&mut self) {
        self.shutdown();
    }
}
