# Rust shogi crates: Core (`rlib`)
[![crate](https://img.shields.io/crates/v/shogi_core)](https://crates.io/crates/shogi_core)
[![docs](https://docs.rs/shogi_core/badge.svg)](https://docs.rs/shogi_core)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

This crate defines fundamental types and functions for shogi (Japanese chess).
The documentation of this crate may be used for the reference of detailed rules of shogi, although it is not intended for introduction to rules.

## Panicking
Functions in this crate does not panic.

## Available features
- `alloc`: `alloc`-related features are available.
- `std`: `std`-related features are available. Implies `alloc`.
- `hash`: implements [`Hash`](https://doc.rust-lang.org/core/hash/trait.Hash.html) for every type it exports.
- `ord`: implements [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html) and [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html) for every type it exports.
