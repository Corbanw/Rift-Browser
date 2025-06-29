# Custom Web Browser Engine

A fully custom web browser engine built from scratch using Flutter, Rust, and Go.

## Architecture

This project demonstrates building a complete web browser engine with three main components:

- **Flutter (Dart)**: Canvas-based UI and event handling
- **Rust**: HTML parsing, CSS parsing, layout engine, and render logic
- **Go**: Raw HTTP networking (DNS, TLS, HTTP/1.1)

## Project Structure

```
my_browser/
├── flutter_ui/                # Flutter app
│   ├── lib/
│   │   ├── main.dart          # Main Flutter app
│   │   ├── engine_bridge.dart # FFI bridge to Rust
│   │   ├── network_bridge.dart # HTTP wrapper to Go
│   │   ├── dom_renderer.dart  # Render boxes to canvas
│   │   └── models/
│   │       └── layout_box.dart
│   └── pubspec.yaml
│
├── rust_engine/               # Rust engine crate
│   ├── src/
│   │   ├── lib.rs            # FFI exports
│   │   ├── html_parser.rs    # HTML tokenizer and DOM builder
│   │   ├── css_parser.rs     # CSS parser (inline styles)
│   │   ├── layout_engine.rs  # Box model layout engine
│   │   └── types.rs          # Core data structures
│   └── Cargo.toml
│
├── go_net/                    # Go networking module
│   ├── main.go               # HTTP server for fetching pages
│   └── go.mod
│
└── README.md
```

## Communication Flow

1. **Flutter UI** starts and loads homepage
2. **Dart** sends HTTP request to **Go** server (`http://localhost:8081/fetch?url=...`)
3. **Go** fetches raw HTML and returns it
4. **Dart** passes HTML to **Rust** via FFI
5. **Rust** returns layout tree with positioned boxes
6. **Flutter** paints DOM tree using CustomPainter

## Setup Instructions

### Prerequisites

- Flutter SDK (latest stable)
- Rust (latest stable)
- Go (latest stable)

### 1. Build Rust Engine

```bash
cd my_browser/rust_engine
cargo build --release
```

This creates a dynamic library:
- Windows: `target/release/rust_engine.dll`
- macOS: `target/release/librust_engine.dylib`
- Linux: `target/release/librust_engine.so`

### 2. Start Go Server

```bash
cd my_browser/go_net
go run main.go
```

The Go server will start on `http://localhost:8081` and provide:
- `GET /fetch?url=<url>` - Fetch HTML from any HTTP URL

### 3. Run Flutter App

```bash
cd my_browser/flutter_ui
flutter pub get
flutter run
```

## Features

### Current Implementation

- ✅ Basic HTML parsing (tags, text, attributes)
- ✅ Simple CSS parsing (inline styles only)
- ✅ Box model layout engine
- ✅ Canvas-based rendering
- ✅ HTTP page fetching
- ✅ Scroll support
- ✅ URL bar and navigation

### HTML Parser (Rust)

The HTML parser tokenizes HTML and builds a DOM tree:
- Handles opening/closing tags
- Parses text content
- Extracts attributes
- Supports self-closing tags
- Skips HTML comments

### CSS Parser (Rust)

Currently supports inline styles:
- `background-color`
- `color`
- `font-size`
- `font-family`
- `border-width`
- `border-color`
- `padding`
- `margin`
- `width`
- `height`
- `display`

### Layout Engine (Rust)

Converts DOM tree to positioned layout boxes:
- Block-level layout
- Text sizing and positioning
- Margin and padding calculations
- Viewport-aware sizing
- Tag-specific defaults (h1, h2, p, div, etc.)

### Network Layer (Go)

Low-level HTTP client:
- Manual URL parsing
- HTTP/1.1 requests
- Realistic User-Agent headers
- Error handling
- JSON response format

### UI Layer (Flutter)

Canvas-based rendering:
- CustomPainter for DOM rendering
- Color parsing (hex, rgb, named colors)
- Text rendering with fonts
- Scroll support
- Loading indicators
- Status messages

## Development Notes

### FFI Bridge

The Flutter-Rust bridge uses FFI (Foreign Function Interface):
- Rust exports C-compatible functions with `#[no_mangle]`
- Dart loads the dynamic library and calls functions
- Memory management handled with proper cleanup

### Mock Implementation

Currently, the Flutter app uses a mock HTML parser for demonstration:
- Real FFI integration requires proper library loading
- Mock creates sample layout boxes for testing
- Can be replaced with actual Rust engine calls

### Limitations

- CSS parsing limited to inline styles
- No external stylesheet support
- Basic layout algorithm (no flexbox/grid)
- Limited font support
- No JavaScript execution
- No image rendering

## Future Enhancements

- [ ] External CSS stylesheet parsing
- [ ] Flexbox and Grid layout
- [ ] JavaScript engine integration
- [ ] Image loading and rendering
- [ ] Better font handling
- [ ] Advanced CSS selectors
- [ ] Performance optimizations
- [ ] Web standards compliance

## Troubleshooting

### Go Server Not Running
If you see "Go server not running" error:
1. Make sure you're in the `go_net` directory
2. Run `go run main.go`
3. Check that port 8081 is available

### Rust Library Not Found
If Flutter can't load the Rust library:
1. Build the Rust engine: `cargo build --release`
2. Copy the library to the Flutter app directory
3. Check library naming conventions for your platform

### Network Issues
If pages don't load:
1. Check internet connection
2. Verify the Go server is running
3. Try a simple URL like `http://example.com`
4. Check browser console for errors

## Contributing

This is a learning project demonstrating browser engine concepts. Feel free to:
- Improve the HTML parser
- Add more CSS features
- Enhance the layout engine
- Optimize performance
- Add new rendering features

## License

This project is for educational purposes. Feel free to use and modify as needed. 