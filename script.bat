@echo off

REM Path to the directory containing your Rust files
SET RUST_FILES_DIR=C:\Users\tbrot\Desktop\Livrable\rust_src

REM Change to the directory containing the Rust files
cd %RUST_FILES_DIR%

REM Compile each Rust file
rustc init_lifetime.rs
rustc inter_arc.rs
rustc inter_lifetime.rs
rustc main_concurrence.rs

echo Compilation complete.
pause