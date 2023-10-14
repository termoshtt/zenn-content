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

# 数式や図を書く
さてドキュメントは読まずにまずはサンプルコードをコピペして動かすとは言いましたが、これでは動かし方が分かっても何を計算してくれるものなのか分からないことも多いでしょう。この時ヒントになるのはまず関数名と引数・戻り値の型です。例えば `add` という関数名は和を計算することを示唆しています。また引数と戻り値の型が `usize` であることから、この関数は整数の和を計算することが分かります。こうなれば何を計算しているかは明らかです。わざわざ英語や日本語ような自然言語で何かを説明する必要はないでしょう。自然言語による説明より型による説明の方が正確で簡潔になり、さらにコンパイラが型をチェックしてくれるので間違いが減ります。

それぞれの関数が名前と型から自明に処理が分かるように設計する技術というのは非常に重要ですが、それでも全てが型で表現できるわけではありません。そうなるといよいよ自然言語でドキュメントを書くことになります。ドキュメントを書く際に気を付けるべきことは、一度自然言語で書いたドキュメントは人間がそれを読んで意味を把握し現在のコードの状況を理解して更新していかなければならないという点です。しかもこの作業はおそらくあなたが面白い研究のアイディアを思いついて、それを実現するために修正している最中に起こります。どうしても説明しないといけないことを端的に説明する必要があり、論文を書くように長々と書くことはできません。

端的に必要なことを説明するにはどうすればいいでしょう？研究集会で自分の研究を短い時間で聴衆に伝えるときどうしていますか？数式や図を多用するはずです。なのでドキュメントでも数式や図が簡単に書けなくてはいけません。ここではその方法を説明します。

ドキュメントはブラウザで見ることになるので、ブラウザで動く技術をドキュメントを記述するために使うことができます。ドキュメントコメントはMarkdownで書くといいましたが、Markdownには[生のHTML片を埋め込む機能](https://spec.commonmark.org/0.30/#raw-html)があるので、HTMLをそのまま埋め込むことができるだけでなく `<script>` 句を埋め込むこともできるので、これにより多くのJavaScriptライブラリをそのまま使うことができます。つまりフロンドエンドの全ての技術をドキュメントを記述するために使うことができます！

## 数式を書く
数値計算の本なので、まずは数式を書く方法を説明しましょう。ブラウザで数式を表示するにはKaTeXの[Auto-render機能](https://katex.org/docs/autorender)を使います。

https://katex.org/

```rust
use ndarray::Array2;

/// Test of $\KaTeX$ document
///
/// $$
/// A = LU
/// $$
///
/// where $A \in R^{n \times n}$ is input matrix,
/// and lower triangular matrix $L \in R^{n \times n}$ and upper triangular matrix $U \in R^{n \times n}$ will be returned.
///
/// <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css" integrity="sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV" crossorigin="anonymous">
/// <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js" integrity="sha384-XjKyOOlGwcjNTAIQHIpgOno0Hl1YQqzUOEleOLALmuqehneUG+vnGctmUb0ZY0l8" crossorigin="anonymous"></script>
/// <script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js" integrity="sha384-+VBxd3r6XgURycqtZ117nYw44OOcIax56Z4dCRWbxyPt0Koah1uHoK0o4+/RRE05" crossorigin="anonymous"></script>
/// <script>
///     document.addEventListener("DOMContentLoaded", function() {
///         renderMathInElement(document.body, {
///           // customised options
///           // • auto-render specific keys, e.g.:
///           delimiters: [
///               {left: '$$', right: '$$', display: true},
///               {left: '$', right: '$', display: false},
///               {left: '\\(', right: '\\)', display: false},
///               {left: '\\[', right: '\\]', display: true}
///           ],
///           // • rendering keys, e.g.:
///           throwOnError : false
///         });
///     });
/// </script>
///
pub fn lu(a: Array2<f64>) -> (Array2<f64>, Array2<f64>) {
    todo!()
}
```

これで次のようなドキュメントが生成されます

![](https://storage.googleapis.com/zenn-user-upload/ed4e0c662345-20231014.png)

この `<script>` 句はちょっと邪魔ですね。これを手続きマクロで挿入してくれるcrateが[katexit](https://github.com/termoshtt/katexit)です。

```rust
use ndarray::Array2;

#[cfg_attr(doc, katexit::katexit)]
/// Test of $\KaTeX$ document
///
/// $$
/// A = LU
/// $$
///
/// where $A \in R^{n \times n}$ is input matrix,
/// and lower triangular matrix $L \in R^{n \times n}$ and upper triangular matrix $U \in R^{n \times n}$ will be returned.
pub fn lu_(a: Array2<f64>) -> (Array2<f64>, Array2<f64>) {
    todo!()
}
```

だいぶ簡潔に書けるようになりました。実は手続きマクロはドキュメント部分も操作することができます。