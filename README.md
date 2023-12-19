
# Project README

## Introduction
This README provides instructions on how to compile and run the Rust and Scala source files included in this project. Each file is treated as a separate entity and can be compiled and executed individually.

## Prerequisites
- Rust: Install Rust from [the official Rust website](https://www.rust-lang.org/tools/install).
- Scala: Ensure Scala and Java (JRE) are installed. Scala can be installed from [the official Scala website](https://www.scala-lang.org/download/).

## Rust Files in the Project
The following Rust files are included:
1. `init_lifetime.rs`
2. `inter_arc.rs`
3. `inter_lifetime.rs`
4. `main_concurrence.rs`

## Scala Files in the Project
The following Scala files are included:
1. `initial`
2. `listBuffer`

## Compiling and Running Rust Files
To compile and run each Rust file, use the following steps:

1. Open your terminal or command prompt.
2. Navigate to the directory containing the Rust file.
3. Compile the file with `rustc <filename>.rs`.
4. Run the compiled executable with `./<filename>`.

Example for `init_lifetime.rs`:
```bash
rustc init_lifetime.rs
./init_lifetime
```

## Compiling and Running Scala Files
For Scala files, use the `scalac` command to compile and `scala` command to run:

1. Open your terminal or command prompt.
2. Navigate to the directory containing the Scala file.
3. Compile the file with `scalac <filename>.scala`.
4. Run the compiled program with `scala <filename>`.

Example for `initial`:
```bash
scalac initial.scala
scala initial
```

## Notes
- Ensure you are in the correct directory where the files are located before running the compile commands.
- The instructions provided are for Unix-like environments. For Windows, the executable will have a `.exe` extension for Rust programs.
- For windows users, you can edit the `script.bat` file to compile all the rust files at once, simply change the path in the script to use the path to your directory
