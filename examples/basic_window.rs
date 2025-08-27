use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::LogicalSize,
};

fn main() {
    println!("Creating a basic window...");
    
    let event_loop = EventLoop::new().unwrap();
    
    let window = WindowBuilder::new()
        .with_title("Rust 2D Engine - Basic Window")
        .with_inner_size(LogicalSize::new(800, 600))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();
    
    println!("Window created! Close the window to exit.");
    
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Window close requested");
                elwt.exit();
            }
            _ => {}
        }
    }).unwrap();
    
    println!("Window closed.");
}
