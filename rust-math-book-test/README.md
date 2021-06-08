Rustで数値計算：サンプルコードテスト
======================================

[Rustで数値計算][rust-math-book]の本文中にあるサンプルコードをテストします。

[rust-math-book]: https://zenn.dev/termoshtt/books/b4bce1b9ea5e6853cb07

Requrements
-----------
- Rust 1.54.0 or later

How it works
-------------
Rustの属性：

```rust
#[doc = "homhom"]
mod sub_section {}
```

はドキュメントに関する属性を設定するもので：

```rust
/// homhom
mod sub_section {}
```

と同様に処理されるのですが、Rust 1.54.0よりこの中で`include_str!`が使えるようになるため
これを使ってZenn bookのMarkdownファイルをそのままドキュメントとして挿入します。
するとMarkdown中のRustコードがdocstring中ではdoctestとして認識されるため
`cargo test --doc`でテストされる事になります。
この事を利用して本文中のサンプルコードをテストします。

How to add new section
-----------------------
Book本文中の各章毎にモジュールを作り、それのdocstringとしてテストを行います。
基本的に`mod`でサブもジュールを[lib.rs](./src/lib.rs)中に作成して、そこに`include_str!`でZenn本文を挿入してください。

```rust
#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/num_traits.md")]
pub mod num_traits {}
```

これによって`cargo doc`でドキュメントが生成されますが、ZennとrustdocのMarkdown処理系は互換ではなく、この本はZennで表示することを前提としているためZennを優先します。rustdocはコードブロックに注釈が無い場合にはrustだと認識しますが、これを避けるため必ず本文のコードブロックには全て注釈を書いてください。

注意事項
---------
依存しているcrateは全て[Cargo.toml](./Cargo.toml)に記述されています。
