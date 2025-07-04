@echo off
echo ========================================
echo    Velox Browser Build Script
echo ========================================
echo.

echo [1/4] Checking prerequisites...
where flutter >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Flutter not found in PATH
    echo Please install Flutter SDK and add it to your PATH
    pause
    exit /b 1
)

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Rust/Cargo not found in PATH
    echo Please install Rust and add it to your PATH
    pause
    exit /b 1
)

echo ✓ Flutter and Rust found
echo.

echo [2/4] Building Rust engine...
cd rust_engine
cargo build --release
if %errorlevel% neq 0 (
    echo ERROR: Failed to build Rust engine
    pause
    exit /b 1
)

echo ✓ Rust engine built successfully
echo.

echo [3/4] Copying DLL to Flutter project...
copy target\release\rust_engine.dll ..\flutter_ui\
if %errorlevel% neq 0 (
    echo ERROR: Failed to copy rust_engine.dll
    pause
    exit /b 1
)

echo ✓ DLL copied successfully
echo.

echo [4/4] Setting up Flutter dependencies...
cd ..\flutter_ui
flutter pub get
if %errorlevel% neq 0 (
    echo ERROR: Failed to get Flutter dependencies
    pause
    exit /b 1
)

echo ✓ Flutter dependencies installed
echo.

echo ========================================
echo    Build completed successfully!
echo ========================================
echo.
echo To run Velox Browser:
echo   flutter run -d windows
echo.
echo To build for release:
echo   flutter build windows --release
echo.
pause 