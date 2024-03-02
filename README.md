# Simple Rust Axum API
It was mainly built for learning purposes.

## How to run the server?
Install `cargo-watch`
```
cargo install cargo-watch
```
This runs the server.
```
cargo watch -q -c -w src/ -x run 
```

On the other terminal you can run 
```
cargo watch -q -c -w tests -x "test -q quick_dev -- --nocapture"
```
With this, you can test the API endpoints. Save the `quick_dev.rs` to re-run the tests.
