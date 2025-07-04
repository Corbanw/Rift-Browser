# Velox Browser Engine

## 🏗️ Architecture (2024)

```
velox_browser/
├── flutter_ui/                # Flutter app (UI, FFI bridge)
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
