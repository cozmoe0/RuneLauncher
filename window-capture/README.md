# Window Capture - JagRenderView to egui

A high-performance Windows application that captures OpenGL windows (specifically with class `JagRenderView`) and displays them inside an egui application with minimal overhead.

## Features

- **Window Detection**: Automatically finds windows with class name `JagRenderView`
- **High Performance Capture**: Uses Windows GDI BitBlt for efficient screen capture
- **Real-time Display**: Renders captured frames in egui with texture streaming
- **FPS Control**: Adjustable capture rate from 1-144 FPS
- **Auto/Manual Modes**: Toggle between automatic continuous capture and manual capture
- **Performance Stats**: Real-time FPS counter and resolution display
- **Aspect Ratio Preservation**: Automatically scales the captured window while maintaining aspect ratio

## Performance Optimizations

1. **Texture Reuse**: Textures are updated in-place rather than recreated each frame
2. **Efficient Memory Layout**: Direct RGBA conversion during capture
3. **Configurable Capture Rate**: Adjust FPS to balance performance and responsiveness
4. **Zero-copy Where Possible**: Minimizes buffer copies in the capture pipeline

## Building

```bash
cd window-capture
cargo build --release
```

## Running

```bash
cargo run --release
```

### Usage

1. Start the application
2. Click "🔍 Find Window" to locate a JagRenderView window
3. The window will automatically start capturing at the configured FPS
4. Adjust settings:
   - Toggle "Auto Refresh" for manual capture control
   - Adjust FPS slider to change capture rate
   - Toggle "Show Stats" to hide/show performance information

## Requirements

- Windows 10 or later
- A process running with a window class `JagRenderView` (e.g., RuneScape client)

## Technical Details

### Architecture

```
┌─────────────────┐
│  JagRenderView  │ (External Process)
│  OpenGL Window  │
└────────┬────────┘
         │
         │ Windows GDI BitBlt
         ▼
┌─────────────────┐
│ WindowCapture   │
│ (Rust)          │
└────────┬────────┘
         │
         │ RGBA Buffer
         ▼
┌─────────────────┐
│ egui Texture    │
│ (GPU)           │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ egui Display    │
└─────────────────┘
```

### Capture Method

The application uses Windows GDI's `BitBlt` function which is:
- **Fast**: Hardware-accelerated when possible
- **Compatible**: Works with OpenGL, DirectX, and GDI windows
- **Reliable**: Well-tested API with predictable behavior

### Alternative Approaches

For even better performance, consider:
- **Windows Graphics Capture API**: Modern capture API (Windows 10 1903+) with GPU acceleration
- **DirectX Shared Textures**: Zero-copy if both processes use DirectX
- **OpenGL FBO Sharing**: Direct OpenGL context sharing (requires process injection)

## Troubleshooting

### Window Not Found
- Ensure the target application is running
- Verify the window class is exactly `JagRenderView`
- Check that the window is visible (not minimized)

### Low FPS
- Reduce the target FPS using the slider
- Check CPU usage in Task Manager
- Ensure the target window is not minimized
- Try running as administrator

### Black Screen
- The target window may be using exclusive fullscreen
- Try switching the target application to windowed mode
- Some DRM-protected content may appear black

## Future Enhancements

- [ ] Support for multiple window classes
- [ ] Window selection UI
- [ ] Capture region selection
- [ ] Mouse/keyboard input forwarding
- [ ] Recording to video file
- [ ] Screenshot functionality
