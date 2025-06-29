# Changelog

All notable changes to Rift Browser will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Rift Browser
- Rust-powered HTML parsing with streaming support
- Advanced CSS selector engine based on Servo's selectors crate
- Real-time layout generation from DOM nodes
- Modular architecture with paint and compositor pipelines
- Enhanced FFI interface with performance tracking
- Robust error handling to prevent crashes
- Cross-platform support (Windows, Linux, macOS)

### Changed
- Renamed from "Custom Browser Engine" to "Rift Browser"
- Improved HTML token extraction in streaming parser
- Enhanced layout box generation and rendering
- Better memory management and cleanup

### Fixed
- Fixed HTML token extraction issue in streaming parser
- Resolved crashes when rendering large numbers of layout boxes
- Fixed FFI function pointer initialization errors
- Improved error handling and fallback mechanisms

## [0.1.0] - 2025-01-XX

### Added
- Basic HTML parsing and DOM construction
- Simple CSS parsing and style application
- Basic layout engine for block and inline elements
- Flutter UI with URL input and page rendering
- FFI bridge between Rust engine and Flutter
- Developer console and logging system
- Memory usage tracking and performance monitoring

### Technical Details
- **HTML Parser**: Token-based streaming parser with support for comments, doctype, and scripts
- **CSS Engine**: Basic selector matching for tag, class, and ID selectors
- **Layout Engine**: Block and inline layout algorithms with CSS box model
- **FFI Interface**: High-performance data transfer with batch processing
- **Error Handling**: Graceful degradation and error recovery

---

## Version History

### Version 0.1.0 (Initial Release)
- **Release Date**: January 2025
- **Status**: Alpha
- **Features**: Basic HTML/CSS rendering, streaming parser, layout engine
- **Platforms**: Windows (primary), Linux/macOS (experimental)

### Future Versions

#### Version 0.2.0 (Planned)
- JavaScript engine integration
- Enhanced CSS support (flexbox, grid)
- Tab management
- Bookmark system

#### Version 0.3.0 (Planned)
- Network layer improvements
- Developer tools
- Mobile platform support
- Performance optimizations

#### Version 1.0.0 (Planned)
- Full web standards compliance
- Production-ready stability
- Comprehensive testing suite
- Complete documentation

---

## Migration Guide

### From Pre-release Versions
- Update Rust engine to latest version
- Rebuild and copy `rust_engine.dll` to Flutter project
- Update Flutter dependencies with `flutter pub get`
- Test with known working URLs

### Breaking Changes
- None in current version
- Future versions may introduce breaking changes in FFI interface

---

## Contributing to Changelog

When adding new entries to the changelog:
1. Add entries under the appropriate version section
2. Use clear, concise descriptions
3. Include technical details for significant changes
4. Link to relevant issues or pull requests
5. Update version numbers and dates as needed 