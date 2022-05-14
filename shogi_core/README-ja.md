# Rust shogi crates: Core (`rlib`)
[![crate](https://img.shields.io/crates/v/shogi_core)](https://crates.io/crates/shogi_core)
[![docs](https://docs.rs/shogi_core/badge.svg)](https://docs.rs/shogi_core)
![Rust Version](https://img.shields.io/badge/rustc-1.60+-blue.svg)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/mit-license.php)

このクレイトは将棋 (日本式チェス) に関する基本的なデータ型と関数を定義します。
このクレイトのドキュメントは将棋の詳細なルールについての参考として使えます。しかしルールの入門を意図したものではありません。

## サポートする利用者
将棋エンジン、取る一手将棋エンジン、詰将棋ソルバー、ばか詰 (協力詰) ソルバー、ステイルメイトソルバーなど、本将棋のルールまたはそれより制限の強いルールに従う利用者をサポートします。
安南将棋、一般のフェアリー詰将棋などはサポートしません。そのような用途に使うことも可能かもしれませんが保証しません。

## 提供する機能
将棋で利用される基本的なデータ型と関数を提供します。

合法手判定は提供しません。合法手判定には多数のやり方が存在するため、別のクレイトで対応します。

SFEN + moves 形式の出力をサポートします。これにより盤面のテストは楽にできます。
SFEN + moves 形式の読み取りは提供しません。複雑であるため、別のクレイトで対応します。

## 依存ライブラリ
`core::*` と `alloc::*` にのみ依存し、`std::*` には依存しません。

`alloc` に依存できない環境が存在します。そのような環境をサポートするため、このクレイトのアイテムは可能な限り `core` だけに依存するようにし、`alloc` に依存する必要があるアイテムは `alloc` フィーチャを使って分けます。

また、それ以外のクレイトには依存しません。

## パニック
このクレイトの関数のうち、`alloc` に依存しないものはパニックしません。
`alloc` に依存する関数はメモリ確保を要求するので、メモリ不足によりパニックし得ます。それ以外ではパニックしません。

## 安全性
以下の条件のもとで、場合により unsafe なアイテムを提供します。

- 提供する unsafe なアイテムは必要最小限になるようにする。
- 名前に `_unchecked` を含むようにし、unsafe であることが明らかであるようにします。
- 未定義動作を起こさないための引数の条件を、ドキュメントの `Safety` セクションで記述します。
- safe なバージョンも提供する。

## 型
以下のエンティティを表現する型を定義します。下のものは上のものに依存します。
- 手番
- マス目
- 駒
- 駒 + 手番
- 指し手
- 持ち駒
- 盤上のマスの部分集合 (ビットボード)
- 盤面 (盤上の駒・持ち駒・手番・今までの指し手列)
- 情報付き盤面 (指し手に情報を持たせられるようにしたもの)
- 非合法手の種類

[discriminant elision](https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html#discriminant-elision-on-option-like-enums) がなるべく適用されるように型定義を行います。discriminant elision についての保証を持つ型もあります。

## トレイト実装
### 比較用トレイト: [`Eq`], [`PartialEq`], [`Ord`] , [`PartialOrd`]
盤面の同一性判定はテストで行いたく、テストで必要なのであれば他にも必要な場面があると思われるので、[`Eq`], [`PartialEq`] は実装します。
[`Ord`], [`PartialOrd`] は `#[cfg(feature = "ord")]` が有効な場合のみ実装します (デフォルトは無効)。[`BTreeMap`] などのキーとして利用することを想定して。

[`Eq`]: https://doc.rust-lang.org/core/cmp/trait.Eq.html
[`PartialEq`]: https://doc.rust-lang.org/core/cmp/trait.PartialEq.html
[`Ord`]: https://doc.rust-lang.org/core/cmp/trait.Ord.html
[`PartialOrd`]: https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html
[`BTreeMap`]: https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html

### コピー用トレイト: [`Clone`], [`Copy`]
すべての型に [`Clone`] を実装します。[`Copy`] については、今後永久に `Copy` であろうと思われる場合に実装します。例えば手番、マス目、駒、持ち駒などには実装し、盤面には実装しません。

[`Clone`]: https://doc.rust-lang.org/core/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/core/marker/trait.Copy.html

### [`Hash`](https://doc.rust-lang.org/core/hash/trait.Hash.html)
将棋のデータのハッシュ値を取りたい場合は以下の 2 通りに分類されると思われます。
1. 高速な将棋エンジンを作るため、ハッシュ値を使って置換表にアクセスする。
2. カジュアルに盤面をキーとするマップを作るために [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) などのデータ構造を使う。

`1.` は自分でハッシュ関数を作ることになるので、ライブラリでの実装の出る幕はありません。`2.` については必要な場合もあるが常に必要ではないと思われるので、`hash` フィーチャを有効化することで利用できるようになります。

### [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html)
ビットボードと盤面には `Default` を実装します。それ以外のものには `Default` を実装しません。

### [`Debug`](https://doc.rust-lang.org/core/fmt/trait.Debug.html)
実装します。

### [`Display`](https://doc.rust-lang.org/core/fmt/trait.Display.html)
文字列表現が複数あり標準的なものが定まらないので定義しません。(例えば、歩の表現には `歩`・`FU`・`P`・`Pawn` などがあります。)
このクレイトは `to_usi` メソッドを定義する `ToUsi` トレイトを定義します。USI フォーマットにおける文字列表現への変換を行います。

### [`From`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html), [`TryFrom`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html)
必要な場合に実装します。

### [`AsRef`](https://doc.rust-lang.org/core/convert/trait.AsRef.html), [`AsMut`](https://doc.rust-lang.org/core/convert/trait.AsMut.html)
実装しません。

## 利用可能なフィーチャ
- `alloc`: `alloc` 関連の機能が利用可能になります。デフォルトで有効化されています。
- `std`: `std` 関連の機能が利用可能になります。有効化すると `alloc` も有効化されます。デフォルトで有効化されています。
- `hash`: エクスポートするすべての型に [`Hash`](https://doc.rust-lang.org/core/hash/trait.Hash.html) を実装します。
- `ord`: エクスポートするすべての型に [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html) と [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html) を実装します。
