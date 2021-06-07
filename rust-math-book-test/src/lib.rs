//! [Rustで数値計算][rust-math-book]のサンプルコードをテストするためのcrateです
//!
//! [rust-math-book]: https://zenn.dev/termoshtt/books/b4bce1b9ea5e6853cb07
//!
//! How it works
//! -------------
//! Rustのアトリビュート
//!
//! ```
//! #[doc = "homhom"]
//! mod sub_section {}
//! ```
//!
//! はドキュメントに関する属性を設定するもので、
//!
//! ```
//! /// homhom
//! mod sub_section {}
//! ```
//!
//! と同様に処理されるのですが、Rust 1.54.0よりこの中で`include_str!`が使えるようになるため
//! これを使ってZenn bookのMarkdownファイルをそのままドキュメントとして挿入します。
//! するとMarkdown中のRustコードはdocstring中ではdoctestとして認識されるため
//! `cargo test --doc`でテストされる事になります。
//! この事を利用して本文中のサンプルコードをテストします。
//!
//! How to add new section
//! -----------------------
//! Book本文中の各章毎にモジュールを作り、それのdocstringとしてテストを行います
//!
//! 注意
//! -----
//! 依存しているライブラリは **この** crateの`[dependencies]`に追加されています
//!

#[doc = include_str!("../../books/b4bce1b9ea5e6853cb07/ndarray_linalg.md")]
pub mod ndarray_linalg {}
