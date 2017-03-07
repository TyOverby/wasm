# WASM in Rust

This project is *not* about compiling rust programs to WASM.

This project *is* about using/producing WASM from inside of Rust programs.

## Crates

### Finished
(crickets...)

### In Progress
* wasm-ast: A simple representation of the WASM module and instructions.
* wasm-binary-format: A parser and serializer for the official WASM binary format.

### Planned
* wasm-sexpr-format: A parser and serializer for the *unofficial* WASM sexpr format used in the reference implementation.
* wasm-interp: A reference implementation (slow) of the WASM vm.
* wasm-validate: An implementation of a WASM module validator.
