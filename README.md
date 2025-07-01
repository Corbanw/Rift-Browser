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
