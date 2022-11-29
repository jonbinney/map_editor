### Running in a browser
Install rust support for web assembly: `rustup target install wasm32-unknown-unknown`

Install a simple server: `cargo install wasm-server-runner`

Run the code: `cargo run --target wasm32-unknown-unknown`

If it doesn't happen automatically, go the URL that cargo tells you it is running the server on.