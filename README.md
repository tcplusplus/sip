![Rust Testing](https://github.com/tcplusplus/sip/workflows/Rust%20Testing/badge.svg)



# SIR Simulator

I am making this test project to learn to work with Rust and Web Assembly.
More information of this model can be found on
[Wikipedia](https://en.wikipedia.org/wiki/Compartmental_models_in_epidemiology#The_SIR_model)

Click [here](https://tcplusplus.github.io/sip/dist/) for a demo of the output result.

# Install Rust

Checkout this page to install Rust:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

You should now have the rust compiler installed.
The rust project is located in the `src/rust` directory.
You can test it out by running
```
cargo build
```
from within the root folder. The targets are compiled in the `target` directory.

or run the unit tests by running
```
cargo test
```

## Code coverage
First install the cargo tarpualin:

```
cargo install cargo-tarpaulin
```

Code coverage is measured with:

```
cargo tarpaulin --out Html
```
This generates the file `tarpaulin-report.html` in the root folder.

# Install Rust to  WASM compiler
Wasm-pack the the wasm compiler for rust and can be installed from.

[https://rustwasm.github.io/wasm-pack/installer/](https://rustwasm.github.io/wasm-pack/installer/)

Compile the rust package by running:

```
wasm-back build
```

This will generate the wasm and typescript files in the `pkg` folder.

# Use wasm in you webpage

The demo webpage is located in the root folder.
Go into the folder and run:
```
yarn install
```
to install the webassembly packages and run
```
yarn serve
```
to test the webpage.
