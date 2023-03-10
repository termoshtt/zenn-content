---
title: "rustdoc中にp5.jsで図を書く"
emoji: "📈"
type: "tech"
topics: ["rust", "p5js", "javascript"]
published: true
---

数値計算のコードなど、機能の説明に複雑な図を必要とする場合があります。これを[p5.js]によって記述する為の簡単なマクロを生成しました。

https://github.com/termoshtt/p5doc

使い方
-------

```rust
#[cfg_attr(doc, p5doc::p5doc)]
/// Some function!
///
/// Before
///
/// ```p5doc:200x100
/// background(220);
/// ellipse(50,50,80,80);
/// ```
///
/// After
///
pub fn some() {}
```

このように記述すると`p5doc`の部分が[p5.js]でによって描画されたHTMLのCanvas要素に置き換わります:
![image](https://user-images.githubusercontent.com/1238153/223720335-bdf1e9a3-8a7b-43a4-ac2d-f188c90cd944.png)

[p5.js]: https://p5js.org/

動作原理
---------
Rustのドキュメントジェネレータである`rustdoc`はMarkdown形式で記述された内容からHTMLのドキュメントを生成しますが、この際HTML片はそのまま受理されて出力のHTMLに埋め込まれます。これは`<script>`を埋め込んでもいいので、例えば：

```rust
/// Head line
///
/// <script>alert("Hey!")</script>
pub fn test() {}
```

と記述するとこの`test()`のページを開くと`alert`が発生します。これは`rustdoc`を実行したときではなく、ドキュメントをブラウザが開いたときにこの`<script>`を見て実行している事に注意してください。つまり任意のJavaScriptライブラリをドキュメントを書くために使えます。これを応用して、図を生成するのに必要なJavaScriptのスクリプトをドキュメント内に埋め込んでおき、ユーザーがブラウザでそのドキュメントを開いたときにブラウザに図を描画させることが出来ます。

そこで図を描画してくれるJavaScriptのライブラリが必要になります。今回は[p5.js]を選択しました。
