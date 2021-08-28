---
title: "rustdoc中で数式を書く"
emoji: "🦀"
type: "tech"
topics: ["Rust", "LaTeX", "KaTeX"]
published: true
---

rustdocは主にブラウザで閲覧しますが、ブラウザ上で数式をレンダリングする方法として[MathJax][mathjax]と[KaTeX][katex]が良く知られています。今回KaTeXを使ってrustdoc中の数式をレンダリングするcrate、[katexit][katexit]を作ったので紹介します

[mathjax]: https://www.mathjax.org/
[katex]: https://katex.org/
[katexit]: https://docs.rs/katexit/0.1.0/katexit/

Usage
-----

`katexit::katexit`というproc-macroが提供されるので：

```toml:Cargo.toml
[dependency]
katexit = "0.1.0"
```

```rust
#[cfg_attr(doc, katexit::katexit)]
/// We can write $\LaTeX$ expressions
///
/// Display style
/// -------------
///
/// $$
/// c = \\pm\\sqrt{a^2 + b^2}
/// $$
pub fn my_func() {}
```

とすれば使えます。`$`で囲まれた範囲がKaTeXによって認識され数式としてレンダリングされます。`$$`でディスプレイモードも使えます。KaTeXで使える数式要素については[公式のドキュメントを参照][katex-doc]してください。

[katex-doc]: https://katex.org/docs/supported.html

How it works
-------------

これは以前紹介した[Mermaid][mermaid]をrustdoc中で使えるようにする[aquamarine][aquamarine]のアイデアをそのまま使っています。rustdocのMarkdown to HTMLコンバータはMarkdown中のHTML片をそのままHTMLに埋め込むので、これを使って[KaTeX auto-render][autorender]を挿入します。ブラウザは`$`がそのまま残っているrustdocの生成したHTMLと一緒にKaTeX auto-renderを読み込み、KaTeX auto-renderがそのページ中の`$`で囲まれた部分を置換します。

[mermaid]: https://mermaid-js.github.io/mermaid/
[aquamarine]: https://github.com/mersinvald/aquamarine
[autorender]: https://katex.org/docs/autorender.html
