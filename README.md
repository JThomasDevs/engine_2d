# Engine 2D

A cross-platform 2D game engine written in Rust with optional OpenGL rendering and GLFW window management.

## Features

The engine provides a complete 2D game development environment with:

- **Window Management**: Cross-platform window creation and event handling using GLFW (optional)
- **OpenGL Rendering**: Hardware-accelerated 2D rendering with shader support (optional)
- **Sprite System**: Texture-based sprite rendering with tinting and alpha blending (optional)
- **Animation System**: Flexible animation framework for game objects
- **Math Utilities**: Comprehensive 2D math library for vectors, matrices, and physics
- **Event System**: Event-driven architecture for input and system communication
- **ECS Framework**: Entity-Component-System for game object management

## Features and Build Modes

### Default Build (Headless)
```bash
cargo build
```
- **No graphics dependencies** - faster compilation
- **Smaller binary size** - ideal for servers and CLI tools
- **Includes**: ECS, math utilities, event system, animation framework
- **Use cases**: Game servers, headless testing, CI/CD, embedded systems

### OpenGL Build (Graphics)
```bash
cargo build --features opengl
```
- **Full graphics support** - OpenGL rendering and windowing
- **Larger binary size** - includes GLFW and OpenGL bindings
- **Includes**: Everything from default + rendering, sprites, textures
- **Use cases**: Game clients, level editors, visual development tools

## Quick Start

### For Graphics Applications
```bash
cargo run --features opengl
```

### For Headless Applications
```bash
cargo run
```

### Examples

#### Basic Window Example (requires OpenGL)
```bash
cargo run --example basic_window --features opengl
```

#### Basic Renderer Example (requires OpenGL)
```bash
cargo run --example basic_renderer --features opengl
```

#### Basic Sprite Example (requires OpenGL)
```bash
cargo run --example basic_sprite --features opengl
```

#### Math Utilities Example (headless)
```bash
cargo run --example math_utilities
```

## Architecture

The engine is designed with modularity in mind:

- **Core Engine**: Game loop, timing, and coordination
- **Window Management**: Cross-platform window creation and event handling
- **Rendering**: OpenGL-based rendering system
- **Input**: Keyboard, mouse, and gamepad input handling
- **Audio**: Cross-platform audio playback
- **Physics**: 2D physics simulation
- **ECS**: Entity-Component-System for game objects

## Dependencies

- **glam**: Math library for vectors and matrices
- **image**: Image loading and processing
- **crossterm**: Cross-platform terminal input
- **glfw**: Window management and OpenGL context creation
- **gl**: OpenGL bindings for rendering

## Building

### Prerequisites
- Rust 1.70 or later
- OpenGL 3.3+
- GLFW 3.3+

### Build Commands

```bash
# Build the engine
cargo build

# Run the engine
cargo run

# Run examples
cargo run --example basic_window
cargo run --example basic_renderer
cargo run --example basic_sprite
```

## When to Use Each Mode

### Use Headless Mode (default) for:
- **Game servers** - Process game logic without rendering
- **CI/CD pipelines** - Automated testing without GPU requirements
- **CLI tools** - Asset processing, level validation, content generation
- **Embedded systems** - IoT devices, microcontrollers
- **WebAssembly** - Browser-based applications (use WebGL instead)

### Use OpenGL Mode for:
- **Game clients** - Full visual rendering for players
- **Level editors** - Visual development tools
- **Game development environments** - IDEs with graphics
- **Desktop applications** - Native apps with UI

## Testing

```bash
# Run all tests (headless)
cargo test

# Run tests with OpenGL features
cargo test --features opengl
```

## License

This project is licensed under the MIT License.
