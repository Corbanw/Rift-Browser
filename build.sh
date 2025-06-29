#!/bin/bash

echo "========================================"
echo "    Rift Browser Build Script"
echo "========================================"
echo

echo "[1/4] Checking prerequisites..."

# Check if Flutter is installed
if ! command -v flutter &> /dev/null; then
    echo "ERROR: Flutter not found in PATH"
    echo "Please install Flutter SDK and add it to your PATH"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust/Cargo not found in PATH"
    echo "Please install Rust and add it to your PATH"
    exit 1
fi

echo "✓ Flutter and Rust found"
echo

echo "[2/4] Building Rust engine..."
cd rust_engine
cargo build --release
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to build Rust engine"
    exit 1
fi

echo "✓ Rust engine built successfully"
echo

echo "[3/4] Copying library to Flutter project..."

# Determine the correct library extension based on OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    cp target/release/librust_engine.so ../flutter_ui/
    if [ $? -ne 0 ]; then
        echo "ERROR: Failed to copy librust_engine.so"
        exit 1
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    cp target/release/librust_engine.dylib ../flutter_ui/
    if [ $? -ne 0 ]; then
        echo "ERROR: Failed to copy librust_engine.dylib"
        exit 1
    fi
else
    # Windows (Git Bash)
    cp target/release/rust_engine.dll ../flutter_ui/
    if [ $? -ne 0 ]; then
        echo "ERROR: Failed to copy rust_engine.dll"
        exit 1
    fi
fi

echo "✓ Library copied successfully"
echo

echo "[4/4] Setting up Flutter dependencies..."
cd ../flutter_ui
flutter pub get
if [ $? -ne 0 ]; then
    echo "ERROR: Failed to get Flutter dependencies"
    exit 1
fi

echo "✓ Flutter dependencies installed"
echo

echo "========================================"
echo "    Build completed successfully!"
echo "========================================"
echo
echo "To run Rift Browser:"
echo "  flutter run -d windows  # or your platform"
echo
echo "To build for release:"
echo "  flutter build windows --release  # or your platform"
echo 