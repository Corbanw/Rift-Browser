# Rift Browser Engine

## 🏗️ Architecture (2024)

```
my_browser/
├── flutter_ui/                # Flutter app (UI, FFI bridge)
│   ├── lib/
│   │   ├── main.dart          # App entrypoint
│   │   ├── browser_app.dart   # App widget
│   │   ├── browser_screen.dart# Main browser screen
│   │   ├── engine_bridge.dart # FFI bridge to Rust
│   │   ├── content_area.dart  # Renders content
│   │   ├── url_bar.dart       # URL input
│   │   ├── performance_info_bar.dart # Perf overlay
│   │   ├── page_renderer.dart # Draws layout boxes
│   │   └── models/
│   │       └── layout_box.dart
│   └── pubspec.yaml
│
├── rust_engine/               # Rust browser engine
│   ├── src/
│   │   ├── dom/               # DOM node, arena, types
│   │   ├── parser/            # HTML, CSS, JS parsers
│   │   ├── layout/            # Layout engine
│   │   ├── paint/             # Paint/rendering
│   │   ├── compositor/        # Compositor pipeline
│   │   ├── ffi/
│   │   │   ├── functions/     # Modular FFI: html_parsing.rs, layout_api.rs, draw_commands.rs, dom_api.rs, memory_management.rs, js_api.rs, mod.rs
│   │   │   └── mod.rs         # FFI module root
│   │   └── lib.rs             # Engine entrypoint, FFI exports
│   └── Cargo.toml
│
├── go_net/                    # Go networking module
│   ├── main.go                # HTTP fetcher
│   └── go.mod
└── README.md
```

## Current Feature Support

| Feature                | Supported? |
|------------------------|------------|
| HTML parsing           | ✅ (basic, robust to errors)
| DOM tree building      | ❌ (only root node, needs fix)
| CSS parsing            | ✅ (inline, no external)
| Layout engine          | ❌ (no boxes generated)
| Rendering pipeline     | ❌ (no visual output)
| Text rendering         | ❌
| Image support          | ❌
| JavaScript execution   | ❌ (stub only)
| Event handling         | ❌
| Navigation             | ✅ (URL bar, fetches pages)
| Error handling         | ✅ (robust, no panics)

## Current Implementation
- Modular FFI in Rust for HTML parsing, layout, DOM, and memory management
- Flutter UI with FFI bridge and basic navigation
- Go networking module for HTTP(S) fetches
- Robust error handling in parser and layout

## Limitations
- Only inline CSS supported (no external stylesheets)
- DOM tree building incomplete (no parent/child relationships)
- No rendering of text, images, or elements
- No JavaScript execution or event handling
- No advanced layout (flexbox, grid, etc.)

## Future Work
- [ ] Fix DOM tree building (parent/child relationships)
- [ ] Implement layout box generation
- [ ] Add rendering of text and elements
- [ ] Add image and external CSS support
- [ ] Integrate JavaScript engine
- [ ] Implement event handling and interactivity

## Future Enhancements

- [ ] External CSS stylesheet parsing
- [ ] Flexbox and Grid layout
- [ ] JavaScript engine integration
- [ ] Image loading and rendering
- [ ] Better font handling
- [ ] Advanced CSS selectors
- [ ] Performance optimizations
- [ ] Web standards compliance

## Contributing

This is a learning project demonstrating browser engine concepts. Feel free to:
- Improve the HTML parser
- Add more CSS features
- Enhance the layout engine
- Optimize performance
- Add new rendering features
