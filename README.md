# Engine 2D

A cross-platform 2D game engine written in Rust with modular feature support.

## Features

The engine supports multiple feature configurations to allow for different use cases:

### Default Configuration
By default, the engine includes window support with OpenGL rendering:
```bash
cargo run  # Same as cargo run --features window
```

### Feature Matrix

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `window` | Full window support with OpenGL rendering (default) | `gl`, `glfw` |
| `gl` | OpenGL bindings for rendering | `gl` |
| `glfw` | GLFW window management | `glfw` |

### Build Configurations

#### Windowed Mode (Default)
```bash
cargo run --features window
# or simply
cargo run
```

#### Headless Mode (Math/Image Processing Only)
```bash
cargo run --no-default-features
```

#### GLFW Only (Context Creation)
```bash
cargo run --features glfw
```

#### OpenGL Only (Rendering Without Window)
```bash
cargo run --features gl
```

### Examples

#### Basic Window Example
```bash
cargo run --example basic_window
```

#### Basic Renderer Example
```bash
cargo run --example basic_renderer
```

#### Basic Sprite Example
```bash
cargo run --example basic_sprite
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
- **glfw**: Window management (optional)
- **gl**: OpenGL bindings (optional)

## Building

### Prerequisites
- Rust 1.70 or later
- OpenGL 3.3+ (for rendering features)
- GLFW 3.3+ (for window features)

### Build Commands

```bash
# Full build with all features
cargo build

# Headless build (math/image processing only)
cargo build --no-default-features

# Specific feature combinations
cargo build --features gl
cargo build --features glfw
cargo build --features window
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with specific features
cargo test --features window
cargo test --no-default-features
```

## License

This project is licensed under the MIT License.
