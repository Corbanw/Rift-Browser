# Contributing to Rift Browser

Thank you for your interest in contributing to Rift Browser! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in the [Issues](https://github.com/yourusername/rift-browser/issues) section
2. Create a new issue with a clear and descriptive title
3. Include steps to reproduce the bug
4. Provide system information (OS, Flutter version, Rust version)
5. Include error messages and logs if applicable

### Suggesting Features

1. Check if the feature has already been suggested
2. Create a new issue with the "enhancement" label
3. Describe the feature and its benefits
4. Provide use cases and examples

### Code Contributions

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Ensure all tests pass
6. Commit your changes with a clear message
7. Push to your branch
8. Create a Pull Request

## 🏗️ Development Setup

### Prerequisites

- Flutter SDK (3.0 or higher)
- Rust (1.70 or higher)
- Git
- IDE (VS Code, IntelliJ IDEA, etc.)

### Local Development

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rift-browser.git
   cd rift-browser
   ```

2. Build the Rust engine:
   ```bash
   cd rust_engine
   cargo build --release
   copy target\release\rust_engine.dll ..\flutter_ui\
   ```

3. Set up Flutter:
   ```bash
   cd ../flutter_ui
   flutter pub get
   ```

4. Run the application:
   ```bash
   flutter run -d windows
   ```

## 📝 Code Style

### Dart/Flutter

- Follow the [Dart style guide](https://dart.dev/guides/language/effective-dart/style)
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions small and focused
- Use proper error handling

### Rust

- Follow the [Rust style guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use meaningful variable and function names
- Add documentation comments (`///`)
- Handle errors properly with `Result` and `Option`
- Write tests for new functionality

## 🧪 Testing

### Flutter Tests

Run Flutter tests:
```bash
cd flutter_ui
flutter test
```

### Rust Tests

Run Rust tests:
```bash
cd rust_engine
cargo test
```

### Integration Tests

Run integration tests:
```bash
cd flutter_ui
flutter test integration_test/
```

## 📋 Pull Request Guidelines

1. **Title**: Use a clear and descriptive title
2. **Description**: Explain what the PR does and why
3. **Tests**: Include tests for new functionality
4. **Documentation**: Update documentation if needed
5. **Screenshots**: Include screenshots for UI changes

### PR Template

```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

## 🐛 Debugging

### Flutter Debugging

1. Enable debug mode:
   ```bash
   flutter run --debug
   ```

2. Check console output for error messages
3. Use Flutter DevTools for performance analysis

### Rust Debugging

1. Enable debug logging in the Rust engine
2. Use `println!` for debugging output
3. Check FFI function calls and memory management

## 📚 Documentation

- Update README.md for new features
- Add inline documentation for complex code
- Update API documentation if needed
- Include examples and usage instructions

## 🚀 Release Process

1. Update version numbers in:
   - `flutter_ui/pubspec.yaml`
   - `rust_engine/Cargo.toml`
   - `README.md`

2. Create a release tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

3. Create a GitHub release with:
   - Release notes
   - Binary downloads
   - Installation instructions

## 📞 Getting Help

- Create an issue for bugs or questions
- Join our community discussions
- Check existing documentation
- Review previous issues and PRs

## 🎯 Areas for Contribution

- **HTML Parser**: Improve parsing accuracy and performance
- **CSS Engine**: Add more selector types and properties
- **Layout Engine**: Enhance layout algorithms
- **UI/UX**: Improve user interface and experience
- **Testing**: Add more comprehensive tests
- **Documentation**: Improve and expand documentation
- **Performance**: Optimize rendering and parsing
- **Platform Support**: Add support for more platforms

Thank you for contributing to Rift Browser! 🌉 