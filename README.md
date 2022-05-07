# Rust shogi crates: Core
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

This repository defines fundamental data types and functions. It contains two crates: a library crate that defines items (`rlib` crate), and a library crate that defines C bindings to them (`cdylib` crate).

Crates in this repository do not require the standard library (i.e., can be used by `no_std` crates) and suitable for embedded systems.
