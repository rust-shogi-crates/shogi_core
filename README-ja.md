# Rust shogi crates: Core
[![Rust](https://github.com/rust-shogi-crates/shogi_core/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rust-shogi-crates/shogi_core/actions/workflows/rust.yml?query=branch%3Amain)
[![C bindings](https://github.com/rust-shogi-crates/shogi_core/actions/workflows/c-bindings.yml/badge.svg?branch=main)](https://github.com/rust-shogi-crates/shogi_core/actions/workflows/c-bindings.yml?query=branch%3Amain)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

このリポジトリは将棋 (日本式チェス) に関する基本的なデータ型と関数を定義します。このリポジトリは 2 個のクレイトからなります: アイテムを定義するライブラリクレイト (`rlib` クレイト) と、それらのアイテムへの C バインディングを定義するライブラリクレイト (`cdylib` クレイト) です。
このリポジトリに含まれているクレイトは標準ライブラリを要求しません。つまり、`no_std` クレイトから使用可能です。組み込みシステムに適しており、もちろん普通のアプリケーションにも適しています。

## 利用可能なフィーチャ
- `alloc`: `alloc` 関連のフィーチャが利用可能になります。デフォルトで有効化されています。
- `std`: `std` 関連のフィーチャが利用可能になります。有効化すると `alloc` も有効化されます。デフォルトで有効化されています。
- `hash`: エクスポートするすべての型に [`Hash`](https://doc.rust-lang.org/core/hash/trait.Hash.html) を実装します。
- `ord`: エクスポートするすべての型に [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html) と [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html) を実装します。
