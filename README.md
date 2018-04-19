# Polkadot Clone

Cloning the implementation at https://github.com/paritytech/polkadot of a https://polkadot.io node in Rust.

# Setup

* Install Rust

* Install dependencies with Cargo 

    ```bash
    cargo install
    ```

* Run

    ```bash
    cargo run -- --help
    
    RUST_LOG=trace RUST_LOG_STYLE=auto \
    cargo run -- -c -l "default.conf" collator
    
    RUST_LOG=error RUST_LOG_STYLE=auto \
    cargo run -- -c -l "default.conf" validator
    ```
    
# Debug

* Run with `RUST_BACKTRACE=1`