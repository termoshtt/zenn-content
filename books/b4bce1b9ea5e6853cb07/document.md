---
title: ドキュメントを書く
---

# 何故ドキュメントを書くのか？何を書くのか？
何を書くべきかを考える時はまず読む側の事を考えましょう。あなたが書いているコードは誰が使いますか？まずはあなたが使うはずです。ではあなたはコードを書いていて、ある標準ライブラリの使い方がすぐに思い出せなかったのでドキュメントを見に行くとしましょう。さてドキュメントを端から端まで読みますか？読まないはずです。とりあえず動けばいいので、サンプルコードをコピペして自分のコードに合うように直そうとして、やっぱり直し方が分からないとなったときに初めて文章を読み始めるでしょう。

あなたが書いたコードについてもこれが出来るようにドキュメントを書くべきです。よって大事なのはそのドキュメントがブラウザですぐ見れるようになっていることと、動作するサンプルコードがそこで見れることです。

# ドキュメントを生成する

Cargoで管理されているRustのプロジェクトでは

```shell
cargo doc
```

とすると `target/doc` 以下にドキュメントが生成されます。生成されたHTMLをブラウザで開くには`cargo doc --open`のように`--open`オプションをつけます。この時依存しているライブラリのドキュメントも一緒に生成するので、必要ない場合は `cargo doc --no-deps` とします。この時はcrateのメタデータに登録されているURLが参照されます。多くの場合はRustの公式チームの一つである[Docs.rs team](https://www.rust-lang.org/governance/teams/dev-tools#docs-rs)によって管理されている <https://docs.rs/> にホストされています。このサイトはRustの公式パッケージレジストリ <https://crates.io/> にパッケージがアップロードされると自動的にそのドキュメントを生成してホストしてくれます。

# サンプルコードを書く
まずはサンプルコードを書いていきましょう。例えば二つの `usize` 引数を取って和を返すコードに対してサンプルコードを書いてみましょう：

```rust
/// ```
/// use rust_math_book_test::document::add;
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

Rustにはいくつかコメントの種類がありますが、主に使うのは次の3つです

- `//` から始まる行コメント
- `///` から始まるドキュメントコメント
- `//!` から始まるモジュールドキュメントコメント

いずれも本体の実装に影響しないのは同じですが、後者二つはドキュメントに含まれます。ドキュメントコメントはMarkdown形式で記述します。\`\`\`で囲われている部分がサンプルコードになります。この時`cargo doc`でHTMLファイルを生成すると次のようになります：

![](https://storage.googleapis.com/zenn-user-upload/7a93168fe844-20230928.png)

:::message
残念ながら一般的にMarkdownと呼ばれる書式に共通の定義はなく処理系に応じて様々な独自の拡張があります。例えば[GitHub flavored Markdown](https://github.github.com/gfm/)やこの文章を記述している[ZennのMarkdown記法](https://zenn.dev/zenn/articles/markdown-guide)などがあります。Rustのドキュメントを処理する `rustdoc` は[CommonMark](https://commonmark.org/)に基づいています。
:::

ではサンプルコードを見ていきましょう。最初に `use` 文がありますね、これはなぜ必要なのでしょう？このドキュメントはcrateのユーザーのためのものなので、ユーザーがこの関数を使うときには `use` でインポートする必要があります。またcrateのユーザーに見えない `pub` の付いていない内部用の関数や構造体については使うことができないことに注意してください。

このコードは `cargo test` でテストの一つとして実行され、なので [Documentation test](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)と呼ばれます。ドキュメントテストだけを実行するには `--doc` オプションを追加します。

```shell
cargo test --doc
```

さらに特定の関数名を指定することもできます。

```text
$ cargo test --doc document::add
   Finished test [unoptimized + debuginfo] target(s) in 0.27s
   Doc-tests rust-math-book-test

running 1 test
test src\lib.rs - document::add (line 21) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 44 filtered out; finished in 0.32s
```

実はこの本の本文中のコードをテストするためにもドキュメントテストを使っています。

:::message alert
自動でテストされていないサンプルコードはすぐに動かなくなります。自分だけしか使わないから、等と言い訳をする前に自動的にテストが実行される環境を整えましょう。
:::

ドキュメントテストはさらに `cargo test` での挙動を調整できます

- コンパイルして実行する (default)
- 実行して失敗することを確認する (`should_panic`)
  - 正しくない例 (実行時に失敗するタイプ) を示すために使う
- コンパイルに失敗する事を確認する (`compile_fail`)
  - 正しくない例 (コンパイル時に失敗するタイプ) を示すために使う
- コンパイルだけする (`no_run`)
  - 別途重いテスト用のデータ用意しないといけない場合などに使う
- コンパイルもしない (`ignore`)
  - 疑似コードを示したい時に使う

これらを指定するには\`\`\`の後ろに`should_panic`や`compile_fail`などを書きます：

```rust
/// ```should_panic
/// use rust_math_book_test::document::add;
/// assert_eq!(add(1, 2), 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```

