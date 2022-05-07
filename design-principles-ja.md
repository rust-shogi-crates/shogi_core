# shogi-core ライブラリの設計原理
no-std 環境にやさしく
いらない機能を載せない
必要な機能をすべて載せる

# 個別の項目

## サポートする利用者
将棋エンジン、取る一手将棋エンジン、詰将棋ソルバー、ばか詰 (協力詰) ソルバー、ステイルメイトソルバーなど、本将棋のルールまたはそれより強いルールに従う利用者をサポートする。
安南将棋、一般のフェアリー詰将棋などはサポートしない。

## 提供する機能
将棋で利用される基本的なデータ型を提供する。
合法手判定は提供しない。合法手判定には多数のやり方が存在するため、別のクレイトで対応する。
SFEN + moves 形式の読み取り、出力をサポートする。これはデータの構築を楽に行うため。

## 依存ライブラリ
`core::*` と `alloc::*` にのみ依存し、`std::*` には依存しない。
[`std::cmp::Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) などの std に存在するアイテムは、core > alloc の順でそのアイテムに置き換える (この場合は [`core::cmp::Eq`](https://doc.rust-lang.org/core/cmp/trait.Eq.html))
さらに、alloc に依存できない環境のことも考え、可能な限り core だけで済むような構造にし、alloc に依存する部分は feature を使って分ける。

また、それ以外のクレイトには依存しない。

## パニック
生成される cdylib について、alloc 必須の機能を使わない限り、Safe Rust の範囲で可能などんな入力に対してもパニックしない。これは no_std 環境での問題を防ぐため。alloc はメモリ確保の失敗などがあるのでパニックせざるを得ない場合があるが、この場合は強制終了するか無限ループに陥る。

## 安全性
場合により unsafe なアイテムを提供するが、以下の条件を守るようにする。

- 提供する unsafe なアイテムは必要最小限になるようにする
- UB を起こさないための引数の条件を `Safety` セクションで定義する。
- safe なバージョンも提供する。

## 型
以下のエンティティを表現する型を定義する。下のものは上のものに依存する。
- 手番
- マス目
- 駒
- 駒 + 手番
- 指し手
- 持ち駒
- 盤上のマスの部分集合
- 盤面 (盤上の駒・持ち駒・手番・今までの指し手列)
- 情報付き盤面 (指し手に情報を持たせられるようにしたもの)
- 非合法手の種類

[discriminant elision](https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html#discriminant-elision-on-option-like-enums) をなるべく起こすようにする。

## トレイト
### 比較用トレイト: [`Eq`](https://doc.rust-lang.org/core/cmp/trait.Eq.html), [`PartialEq`](https://doc.rust-lang.org/core/cmp/trait.PartialEq.html), [`Ord`](https://doc.rust-lang.org/core/cmp/trait.Ord.html), [`PartialOrd`](https://doc.rust-lang.org/core/cmp/trait.PartialOrd.html)
盤面の同一性判定はテストで行いたく、テストで必要なのであれば他にも必要な場面があると思われるので、`Eq`, `PartialEq` は実装する。
`Ord`, `PartialOrd` は `#[cfg(feature = "ord")]` が有効な場合のみ実装する (デフォルトは無効)。`BTreeMap` などのキーとして利用することを想定して。

### コピー用トレイト: [`Clone`](https://doc.rust-lang.org/core/clone/trait.Clone.html), [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html)
特に理由がない限り `Clone` を実装する。`Copy` については、今後永久に `Copy` であろうと思われる場合に実装する。例えば手番、マス目、駒、持ち駒などには実装し、盤面には実装しない。

### [`Hash`](https://doc.rust-lang.org/core/hash/trait.Hash.html)
将棋のデータのハッシュ値を取りたい場合は以下の 2 通りに分類されると思われる。
1. 高速な将棋エンジンを作るため、ハッシュ値を使って置換表にアクセスする。
2. カジュアルに盤面をキーとするマップを作るために `HashMap` などのデータ構造を使う。

`1.` は自分でハッシュ関数を作ることになるので `#[derive(Hash)]` で作られる実装の出る幕はない。2. については必要な場合もあるが常に必要ではないと思われるため、`#[cfg(feature = "hash")]` で有効化できるようにする。
### [`Default`](https://doc.rust-lang.org/core/default/trait.Default.html)
盤面には `Default` を実装する。それ以外のものには `Default` を実装しない。
### [`Debug`](https://doc.rust-lang.org/core/fmt/trait.Debug.html)
実装する。
### [`Display`](https://doc.rust-lang.org/core/fmt/trait.Display.html)
妥当な文字列表現が複数あり最良のものが一意には定まらないので定義しない。文字列表現は `to_usi` のような名前の関数、およびそれを要請する `ToUsi` のようなトレイトで定義する。

### [`From`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html), [`TryFrom`](https://doc.rust-lang.org/core/convert/trait.TryFrom.html), [`AsRef`](https://doc.rust-lang.org/core/convert/trait.AsRef.html), [`AsMut`](https://doc.rust-lang.org/core/convert/trait.AsMut.html)