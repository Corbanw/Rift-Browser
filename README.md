# Rift Browser

A high-performance web browser built with Flutter and Rust, featuring a custom HTML/CSS rendering engine.

## 🌟 Features

- **Rust-powered HTML parsing** with streaming support for large documents
- **Advanced CSS selector engine** based on Servo's selectors crate
- **Real-time layout generation** from DOM nodes
- **Modular architecture** with paint and compositor pipelines
- **Enhanced FFI interface** with performance tracking
- **Robust error handling** to prevent crashes
- **Cross-platform support** (Windows, Linux, macOS)

## 🏗️ Architecture

```
Rift Browser/
├── flutter_ui/          # Flutter frontend application
│   ├── lib/            # Dart source code
│   ├── android/        # Android platform files
│   ├── ios/           # iOS platform files
│   ├── windows/       # Windows platform files
│   └── pubspec.yaml   # Flutter dependencies
├── rust_engine/        # Rust backend engine
│   ├── src/           # Rust source code
│   │   ├── dom/       # DOM node implementation
│   │   ├── parser/    # HTML and CSS parsers
│   │   ├── layout/    # Layout engine
│   │   ├── paint/     # Paint and rendering
│   │   ├── compositor/# Compositor pipeline
│   │   └── ffi/       # FFI interface
│   └── Cargo.toml     # Rust dependencies
└── README.md          # This file
```

## 🚀 Quick Start

### Prerequisites

- **Flutter SDK** (3.0 or higher)
- **Rust** (1.70 or higher)
- **Cargo** (Rust package manager)
- **Visual Studio Build Tools** (for Windows)
- **Git**

### Building the Rust Engine

1. Navigate to the Rust engine directory:
   ```bash
   cd rust_engine
   ```

2. Build the engine in release mode:
   ```bash
   cargo build --release
   ```

3. Copy the DLL to the Flutter project:
   ```bash
   copy target\release\rust_engine.dll ..\flutter_ui\
   ```

### Running the Flutter App

1. Navigate to the Flutter project:
   ```bash
   cd flutter_ui
   ```

2. Get dependencies:
   ```bash
   flutter pub get
   ```

3. Run the application:
   ```bash
   flutter run -d windows
   ```

## 🔧 Development

### Project Structure

#### Flutter UI (`flutter_ui/`)
- **`lib/main.dart`** - Main application entry point
- **`lib/engine_bridge.dart`** - FFI bridge to Rust engine
- **`lib/web_renderer.dart`** - Flutter widget renderer
- **`lib/dom_renderer.dart`** - DOM rendering utilities
- **`lib/dev_console.dart`** - Developer console

#### Rust Engine (`rust_engine/`)
- **`src/dom/`** - DOM node implementation and tree structure
- **`src/parser/`** - HTML and CSS parsing with streaming support
- **`src/layout/`** - Layout engine for positioning elements
- **`src/paint/`** - Paint and rendering pipeline
- **`src/compositor/`** - Compositor for final rendering
- **`src/ffi/`** - Foreign Function Interface for Flutter integration

### Key Components

#### HTML Parser
- Streaming HTML parser that processes chunks incrementally
- Token-based parsing with support for comments, doctype, and scripts
- DOM tree construction with proper parent-child relationships

#### CSS Engine
- CSS selector matching using Servo's selectors crate
- Support for tag, class, and ID selectors
- Inline and external stylesheet processing

#### Layout Engine
- Block and inline layout algorithms
- CSS box model implementation
- Viewport-based positioning

#### FFI Interface
- High-performance data transfer between Rust and Dart
- Batch processing for large numbers of layout boxes
- Memory management and cleanup

## 🐛 Troubleshooting

### Common Issues

1. **DLL not found**: Ensure `rust_engine.dll` is copied to `flutter_ui/`
2. **Build errors**: Make sure you have the correct Rust and Flutter versions
3. **Memory issues**: The browser limits rendering to 500 elements by default

### Debug Mode

Enable debug logging by checking the console output. The browser logs:
- HTML parsing progress
- CSS selector matching
- Layout box generation
- Performance metrics

## 📝 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🎯 Roadmap

- [ ] JavaScript engine integration
- [ ] Network layer improvements
- [ ] Tab management
- [ ] Bookmark system
- [ ] Developer tools
- [ ] Mobile platform support

## 📊 Performance

The browser is optimized for:
- **Large document processing** (up to 10MB HTML)
- **Real-time rendering** with streaming parsers
- **Memory efficiency** with batch processing
- **Cross-platform performance** with native Rust code

---

**Rift Browser** - Bridging the gap between modern web standards and high-performance rendering! 🌉 