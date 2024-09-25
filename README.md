# Raikiri Edge - Rust Hello World Example

This is a simple "Hello World" example, ready to be deployed to Raikiri Edge.

To build:

```sh
cargo install cargo-component
cargo component build --release
```

The generated WASM should be at `./target/wasm32-wasi/release/raikiri_hello_world.wasm`.

Register the component and run it with raikiri:

```sh
raikiri component add --name hello --path target/wasm32-wasi/release/raikiri_hello_world.wasm
raikiri raikiri component run --request "`cat request.json`"

# Output:
# Stdout from vinicius.hello: Data from stdout

# Successfully invoked vinicius.hello
# Response: Hello World!
```