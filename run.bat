@echo off
echo Running Cargo Build... Please wait.
set CARGO_TARGET_DIR=target_cli
cargo build -p canvas-app -j 1
if %errorlevel% neq 0 (
    echo.
    echo Build failed. Please check the errors above.
    pause
    exit /b %errorlevel%
)
echo.
echo Build succeeded! Launching Canvas MVP...
cargo run -p canvas-app
pause
