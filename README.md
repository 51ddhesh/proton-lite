# Proton

Proton is a simple math engine I am working on to learn more about parsing and evaluation of expressions.

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

## TODO / Upcoming Features

- **Complete the implementation of `differentiate.rs`**: Add full support for symbolic differentiation of mathematical expressions.
- **Complete the implementation of `integrate.rs`**: Implement symbolic integration for supported expressions.
- **Improve `simplify.rs`**: Enhance the simplification logic to handle more complex expressions and edge cases.
- **Implement string parsing**: Add a parser to convert string input into `Expr` ASTs, enabling user-friendly input.
- **Complete `repl.rs`**: Build an interactive Read-Eval-Print Loop (REPL) for evaluating and manipulating expressions from the command line.

> The [`main.rs`](/src/main.rs) is currently a placeholder. It is kept for potential future use if this project is extended as a CLI or an API service.

These tasks will make Proton a more complete and user-friendly math engine for both programmatic and interactive use.
