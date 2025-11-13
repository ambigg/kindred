# Kindred Programming Language

A modern programming language compiler written in Rust.

## Overview

Im writing a compiler named Kindred with `.kin` file extension, i know im not using the best practices but for now its on develop stage since im making this for school and for learning

## Usage

### Compile a Kindred Program

To compile a `.kin` source file:

```bash
cargo run -- make
```

This command builds your Kindred program in **Release mode** by default for optimal performance.

### Clean Build Artifacts

To remove compiled executables and build artifacts:

```bash
cargo run -- clean
```

## File Extension

Kindred source files use the `.kin` extension:

```
my_program.kin
```

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

---

**Status:** In Development

The compiler does not yet accept file paths as command-line arguments. For development purposes, it is currently hardcoded to compile the main.kin file located in the project's root directory.
