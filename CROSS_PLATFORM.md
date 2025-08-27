# Cross-Platform Compatibility Guide

## 🖥️ **Supported Platforms**

This engine is designed to be cross-platform and supports:

- **Windows** (Windows 10/11, Windows Server)
- **macOS** (10.15+)
- **Linux** (Ubuntu 18.04+, Debian 10+, Fedora 30+, etc.)
- **Web** (via WebAssembly - planned)

## 📦 **Dependencies & Cross-Platform Support**

### **Core Dependencies**

| Dependency | Purpose | Cross-Platform Support |
|------------|---------|------------------------|
| `winit` | Window management | ✅ Windows, macOS, Linux, Web |
| `gl` | OpenGL bindings | ✅ Windows, macOS, Linux |
| `crossterm` | Terminal input | ✅ Windows, macOS, Linux |
| `glam` | Math library | ✅ All platforms |
| `image` | Image loading | ✅ All platforms |

### **Platform-Specific Considerations**

#### **Windows**
- Uses Win32 API via `winit`
- OpenGL via `gl` crate
- Terminal input via `crossterm`

#### **macOS**
- Uses Cocoa via `winit`
- OpenGL via `gl` crate
- Terminal input via `crossterm`

#### **Linux**
- Uses X11/Wayland via `winit`
- OpenGL via `gl` crate
- Terminal input via `crossterm`

## 🔧 **Building for Different Platforms**

### **Windows**
```bash
cargo build --release
```

### **macOS**
```bash
cargo build --release
```

### **Linux**
```bash
# Install OpenGL development libraries
sudo apt-get install libgl1-mesa-dev  # Ubuntu/Debian
sudo dnf install mesa-libGL-devel     # Fedora

cargo build --release
```

### **Cross-Compilation**
```bash
# For Windows from Linux
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu

# For Linux from Windows
rustup target add x86_64-unknown-linux-gnu
cargo build --target x86_64-unknown-linux-gnu
```

## 🎯 **Testing Cross-Platform Compatibility**

### **Local Testing**
```bash
# Test basic window
cargo run --example basic_window

# Test engine
cargo run
```

### **CI/CD Testing**
The engine should be tested on:
- Windows (GitHub Actions)
- macOS (GitHub Actions)
- Linux (GitHub Actions)

## 🚀 **Platform-Specific Features**

### **Window Management**
- **Windows**: Native Win32 windows
- **macOS**: Native Cocoa windows
- **Linux**: X11/Wayland windows

### **Input Handling**
- **Keyboard**: Cross-platform via `crossterm`
- **Mouse**: Cross-platform via `winit`
- **Gamepad**: Planned via `gilrs`

### **Rendering**
- **OpenGL**: Cross-platform via `gl` crate
- **Vulkan**: Planned for future versions

## 📋 **Best Practices**

1. **Use Cross-Platform Crates**: Always prefer crates that work on all platforms
2. **Test Regularly**: Test on all target platforms
3. **Handle Paths Correctly**: Use `std::path::Path` for file operations
4. **Avoid Platform-Specific Code**: Use abstractions when possible
5. **Document Platform Differences**: Note any platform-specific behavior

## 🔮 **Future Enhancements**

- **WebAssembly Support**: For web deployment
- **Mobile Support**: iOS and Android
- **Vulkan Rendering**: Modern graphics API
- **Metal Support**: macOS-specific graphics API
