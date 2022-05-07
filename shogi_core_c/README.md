# Rust shogi crates: Core (`cdylib`)
[![crate](https://img.shields.io/crates/v/shogi_core_c)](https://crates.io/crates/shogi_core_c)
[![docs]( https://docs.rs/shogi_core_c/badge.svg)](https://docs.rs/shogi_core_c)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

This crate defines C bindings to [`shogi_core`](../shogi_core/). A C header file for this crate is placed in `include/` and generated with `make`.

## Size of the artifact
For x86_64-unknown-linux-gnu, the resulting shared object's size does not exceed 40KiB.
