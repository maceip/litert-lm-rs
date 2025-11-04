# litert-lm-rs

Safe, idiomatic Rust bindings for the [LiteRT-LM](https://github.com/google-ai-edge/LiteRT-LM) C API.

## About

LiteRT-LM is Google's lightweight runtime for on-device large language models. This crate provides Rust bindings that allow you to use LiteRT-LM from Rust applications with a safe, ergonomic API.

**Upstream Repository**: https://github.com/google-ai-edge/LiteRT-LM

## Use Cases

- **On-device AI applications**: Run language models locally without cloud dependencies
- **Privacy-focused apps**: Keep user data on-device with local inference
- **Embedded systems**: Deploy LLMs on resource-constrained devices
- **Rust applications**: Integrate LiteRT-LM into Rust-based services and tools
- **Cross-platform development**: Build portable AI applications for Linux and macOS

## Interfaces

### Engine

The primary interface for loading and managing a language model. An `Engine` owns the loaded model and can create multiple sessions.

**API**:
- `Engine::new(model_path, backend)` - Load a .tflite model file with CPU or GPU backend
- `engine.create_session()` - Create a new conversation session

**Use when**: You need to load a model once and create multiple independent conversation contexts.

### Session

Represents a stateful conversation context. Each session maintains its own history and can generate text responses.

**API**:
- `session.generate(prompt)` - Generate text from a prompt
- `session.get_benchmark_info()` - Get performance metrics

**Use when**: You need to maintain conversation history or generate multiple related responses.

### Backend

Specifies the hardware backend for model execution.

**Options**:
- `Backend::Cpu` - CPU-based inference
- `Backend::Gpu` - GPU-accelerated inference (if available)

## Features

- **Automatic FFI generation**: Uses `bindgen` to auto-generate bindings from `c/engine.h`
- **Safe API**: Memory-safe Rust wrappers around C FFI
- **Thread-safe**: Engine and Session types implement Send/Sync where appropriate
- **Idiomatic Rust**: Result-based error handling, RAII resource management
- **Portable**: Works on Linux and macOS with simple build process
- **Zero maintenance**: Bindings stay in sync with C API automatically

## Building

### Prerequisites

This crate requires the LiteRT-LM C library to be built first.

1. **Clone LiteRT-LM**:
   ```bash
   git clone https://github.com/google-ai-edge/LiteRT-LM
   cd LiteRT-LM
   ```

2. **Build the C API library**:
   ```bash
   bazel build //c:engine
   ```

   This creates:
   - `bazel-bin/c/libengine.so` (Linux)
   - `bazel-bin/c/libengine.dylib` (macOS)

### Build the Rust bindings

```bash
cargo build --release
```

The build script will automatically:
- Run `bindgen` to generate FFI bindings from `c/engine.h`
- Link against `bazel-bin/c/libengine.so`

### Runtime library path

When running your application, ensure the dynamic linker can find `libengine.so`:

```bash
export LD_LIBRARY_PATH=/path/to/LiteRT-LM/bazel-bin/c:$LD_LIBRARY_PATH
```

Or build with rpath:
```bash
RUSTFLAGS="-C link-args=-Wl,-rpath,/path/to/LiteRT-LM/bazel-bin/c" cargo build
```

## Usage

### Basic example

```rust
use litert_lm::{Engine, Backend};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load model
    let engine = Engine::new("model.tflite", Backend::Cpu)?;

    // Create conversation session
    let session = engine.create_session()?;

    // Generate text
    let response = session.generate("Hello, how are you?")?;
    println!("Response: {}", response);

    Ok(())
}
```

### Interactive chat example

```rust
use litert_lm::{Engine, Backend};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::new("model.tflite", Backend::Cpu)?;
    let session = engine.create_session()?;

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().eq_ignore_ascii_case("quit") {
            break;
        }

        match session.generate(input.trim()) {
            Ok(response) => println!("Assistant: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
```

### Running examples

```bash
# Simple interactive chat
cargo run --example simple_chat -- /path/to/model.tflite

# Batch inference
cargo run --example batch_inference -- /path/to/model.tflite
```

## API Overview

### Engine

The `Engine` is the main entry point for loading and managing a language model.

```rust
let engine = Engine::new("model.tflite", Backend::Cpu)?;
```

**Methods:**
- `new(model_path: &str, backend: Backend) -> Result<Engine>` - Create engine from model file
- `create_session(&self) -> Result<Session>` - Create a new conversation session

### Session

A `Session` represents a conversation context with history.

```rust
let session = engine.create_session()?;
let response = session.generate("Hello!")?;
```

**Methods:**
- `generate(&self, prompt: &str) -> Result<String>` - Generate response to prompt
- `get_benchmark_info(&self) -> Result<BenchmarkInfo>` - Get performance metrics

### Backend

```rust
pub enum Backend {
    Cpu,  // CPU backend
    Gpu,  // GPU backend (if available)
}
```

### Error Handling

All operations return `Result<T, Error>` for proper error handling:

```rust
match session.generate("prompt") {
    Ok(response) => println!("Success: {}", response),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Memory Management

The wrapper uses RAII (Resource Acquisition Is Initialization) for automatic cleanup:

- `Engine` automatically calls `litert_lm_engine_delete` on drop
- `Session` automatically calls `litert_lm_session_delete` on drop
- Generated strings are automatically freed

No manual memory management required.

## Thread Safety

- `Engine`: Implements `Send + Sync` - can be shared between threads
- `Session`: Implements `Send` - can be moved between threads, but not shared

## Build Process Details

The build process is designed to be simple and portable:

1. **`build.rs` runs bindgen**:
   - Reads `c/engine.h` from LiteRT-LM
   - Generates Rust FFI bindings
   - Writes to `$OUT_DIR/bindings.rs`

2. **Links against minimal dependencies**:
   - `libengine.so` (or `.dylib` on macOS)
   - C++ standard library (`stdc++` on Linux, `c++` on macOS)
   - No need to manually link dozens of libraries

3. **Your code uses the safe wrapper**:
   - `src/lib.rs` provides safe Rust API
   - Automatically includes generated bindings
   - Users never see unsafe code

## Troubleshooting

### "cannot find -lengine"

The C API library is not in your library path.

**Solution**: Build the C library first:
```bash
cd /path/to/LiteRT-LM
bazel build //c:engine
```

### "cannot find libengine.so at runtime"

At runtime, the dynamic linker can't find the library.

**Solution**: Set `LD_LIBRARY_PATH`:
```bash
export LD_LIBRARY_PATH=/path/to/LiteRT-LM/bazel-bin/c:$LD_LIBRARY_PATH
```

Or build with rpath:
```bash
RUSTFLAGS="-C link-args=-Wl,-rpath,/path/to/bazel-bin/c" cargo build
```

## License

Apache-2.0

## Contributing

Contributions welcome! Please ensure:
- Code follows Rust conventions (`cargo fmt`)
- All tests pass (`cargo test`)
- New features include tests and documentation
