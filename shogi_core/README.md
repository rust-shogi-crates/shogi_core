# Rust shogi crates: Core (`rlib`)
[![crate](https://img.shields.io/crates/v/shogi_core)](https://crates.io/crates/shogi_core)
[![docs](https://docs.rs/shogi_core/badge.svg)](https://docs.rs/shogi_core)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

This crate defines fundamental types and functions for shogi (Japanese chess).
The documentation of this crate may be used for the reference of detailed rules of shogi, although it is not intended for introduction to rules.

## Supported use cases
This crate supports users that follow shogi rules or something stricter than them, such as shogi engines, capture-only variant shogi engines, mate solvers, helpmate solvers, stalemate solvers.
This crate does not support fairy mate problems in general. One might be able to make use of this crate for such shogi variants, but it is not guaranteed.

## Provided functionalities
This crate provides fundamental data types and functions used in shogi.

This crate does not provide legality checking. There are many ways to check legality, so it is responsibility of other crates.

This crate supports output of SEFN + moves format. This helps easy testing of positions.
This crate does not support reading of SEFN + moves format because of its complexity. Other crates are responsible for this.

## Dependencies
This crate depends only on `core::*` and `alloc::*`. This crate does not depend on `std::*`.

There are environments where depending on `alloc` is impossible. In order to support such environments, items in this crate depend only on `core` as much as possible, and items that must depend on `alloc` are separated by `alloc` feature.

This crate does not depend on any other crates.

## Panicking
Functions in this crate that do not depend on `alloc` do not panic.
Functions that depend on `alloc` can panic because they require memory allocation, which can fail due to out of memory. Otherwise, they do not panic.

## Safety
This crate may provide unsafe items under the following conditions:

- Only bare minimum unsafe items are provided.
- Unsafe items contain `_unchecked` in their name so that unsafety of them is obvious.
- In the `Safety` section in the document, the conditions imposed on arguments for an unsafe item not to cause undefined behavior are described.
- For each unsafe item, this crate provides its safe counterpart.

## Types
This crate defines types representing the following entities. An entity below can depend on entities above.
- player to move
- square
- piece
- piece + player to move
- move
- hand
- subset of all squares (bitboard)
- position (pieces on the board, player to move, moves made so far)
- position with information (moves can have additional information)
- kinds of illegal moves

Types are defined so that [discriminant elision](https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html#discriminant-elision-on-option-like-enums) applies for as many types as possible. Some types even have guarantees about discriminant elision.

## Trait implementations
### Traits for comparison: [`Eq`], [`PartialEq`], [`Ord`] and [`PartialOrd`]
[`Eq`] and [`PartialEq`] are implemented for all types because we need equality testing in tests anyway, and because if we need equality testing in tests, there are assumed to be other use cases.
[`Ord`] and [`PartialOrd`] are implemented when `ord` feature is enabled (which is disabled by default), so that types in this crate can be used as keys of e.g. [`BTreeMap`].

[`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
[`PartialEq`]: https://doc.rust-lang.org/core/cmp/trait.PartialEq.html
[`Ord`]: https://doc.rust-lang.org/core/cmp/trait.Ord.html
[`PartialOrd`]: https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html
[`BTreeMap`]: https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html

### Traits for copying: [`Clone`] and [`Copy`]
[`Clone`] is implemented for every type. [`Copy`] is implemented if we can assume that a type will be `Copy` forever. For example, player to move, square, piece and hand implement `Copy`, but position doesn't.

[`Clone`]: https://doc.rust-lang.org/core/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/core/marker/trait.Copy.html

### [`Hash`](https://doc.rust-lang.org/core/hash/trait.Hash.html)
There are assumed to be two main use cases when one wants to take a hash value of data of shogi:
1. Accessing transposition table using hash values for fast shogi engines.
2. Using data structures like [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) in order to casually create maps whose keys are positions.

For `1.`, it is not the place of implementations in the library because one needs to make their own hash function. For `2.`, some but not all users may need these implementations. They are made available if the `hash` feature is enabled.

### [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html)
Bitboard and position implement `Default`. The other types don't implement `Default` because there are no values suitable for default.

### [`Debug`](https://doc.rust-lang.org/core/fmt/trait.Debug.html)
Always implemented.

### [`Display`](https://doc.rust-lang.org/core/fmt/trait.Display.html)
Not implemented because there are multiple string representations and no canonical string representation among them. (For example, a pawn can be represented as `歩`, `FU`, `P` or `Pawn`.)
This crate defines the trait `ToUsi` which defines `to_usi` method, which handles conversion to string representations in USI format.

### [`From`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html), [`TryFrom`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html)
Implemented when necessary.

### [`AsRef`](https://doc.rust-lang.org/core/convert/trait.AsRef.html), [`AsMut`](https://doc.rust-lang.org/core/convert/trait.AsMut.html)
Not implemented.

## Available features
- `alloc`: `alloc`-related functionalities are made available. Enabled by default.
- `std`: `std`-related functionalities are made available. Implies `alloc`. Enabled by default.
- `hash`: implements [`Hash`](https://doc.rust-lang.org/core/hash/trait.Hash.html) for every type it exports.
- `ord`: implements [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html) and [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html) for every type it exports.
