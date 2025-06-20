# Proton

Proton is a simple math engine I am working on to learn more about parsing and evaluation of expressions.

## Requirements

- `Rust` (edition 2024 or later)
- `cargo` (for building, running, and testing)

To check if you have Rust and Cargo installed, run:

```bash
rustc --version
cargo --version
```

`Rust` and `cargo` can be installed by following the instructions at [rustup.rs](https://rustup.rs/).

## Usage

Example usage can be found in the [`examples/`](/examples/) directory. To run the main example, use:

```bash
cargo run --example example
```

This demonstrates how to build, evaluate, and manipulate mathematical expressions using the Proton engine.

## Testing

Automated tests are located in the [`tests/`](/tests/) directory. To run all tests, use:

```bash
cargo test
```

This will execute the test suite to verify the correctness of expression evaluation and other core features.


> Currently, the [`main.rs`](/src/main.rs) is a placeholder. It is kept for potential future use if this project is extended as a CLI or an API service.


## License
Licensed under the [MIT License](./LICENSE)
