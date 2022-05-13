# Rust shogi crates: Core (`cdylib`)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

This crate defines C bindings to [`shogi_core`](../shogi_core/). A C header file for this crate is placed in `include/` and generated with `make`.

## Publishing to [crates.io](https://crates.io/)
This crate is for automated testing of the size after the compilation of the `rlib` crate. Therefore, it is not intended for publishing to [crates.io](https://crates.io/). Most users' need should be satisfied by the `rlib` crate only, because the `cdylib` crate simply re-exports what the `rlib` crate exports. If you truly want to depend on the `cdylib` crate (i.e., use the generated cdylib and header files), clone this repository and manually build it with `cargo`.

If the `rlib` crate also had `crate_type = "cdylib"`, the `rlib` crate would define `no_std`-related functions (e.g., a panic handler, memory-related functions, an alloc handler), which prevents other crates from depending on the `rlib` crate and define their own `no_std`-related functions (if they are e.g. `cdylib` crates). Therefore, separating a `cdylib` crate from an `rlib` crate is inevitable.

## Size of the artifact
For x86_64-unknown-linux-gnu, the resulting shared object's size does not exceed 40KiB.

## Available features
- `alloc`: `alloc`-related features are available. Enabled by default.
- `std`: `std`-related features are available. Implies `alloc`. Enabled by default.
