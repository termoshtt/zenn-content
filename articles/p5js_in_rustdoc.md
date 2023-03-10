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

そこで図を描画してくれるJavaScriptのライブラリが必要になります。今回は[p5.js]を選択しました。[p5.js]は電子アートとビジュアルデザインを標榜する[Processing](https://processing.org/)の開発元が作っているJavaScriptライブラリで、同じように図を書いたりアニメーションを作ったりできます。

[p5.js]をCDNから取得して簡単な図を書くドキュメントを生成するには例えば次のように書きます：

```rust
/// Some function!
///
/// Before
///
/// <script src="https://cdn.jsdelivr.net/npm/p5@1.6.0/lib/p5.js"></script>
/// <script>
/// function setup() {
///   var canvas = createCanvas(200, 200);
///   canvas.parent("doc-some");
/// }
/// function draw() {
///   background(220);
///   ellipse(50,50,80,80);
/// }
/// </script>
/// <div id="doc-some"></div>
///
/// After
///
pub fn some() {}
```

[p5.js]は二つの関数`setup()`と`draw()`を定義するとそれに従って図を書いてくれます。`setup()`の中の`createCanvas(200, 200)`でHTMLの`<canvas>`要素がページに作られ、`ellipse(...)`で楕円が追加されます。この時何もしていしないとページの一番下に図を作ってしまうので`<div>`要素を追加してそれの下に図を作るように指定することでその位置、この例だと`Before`と`After`の間に図を作ってくれます。

これは今回作った`p5doc`は全く関係なく動作します。なのでもし`p5doc`を使ってみて上手くいかなかったらこの方式に切り替えてください。`p5doc`の手続きマクロはドキュメント中の`p5doc`で始まるインラインコードを見つけてそれを上のような`<script>`を生成します。ユーザーは`draw()`の中身だけを書くだけでドキュメントに図が描画されます。

Roadmap
--------
- 現在は一つのドキュメントに一つの図しか挿入できません
  - [Issue](https://github.com/termoshtt/p5doc/issues/3)は立ててありますが、複数の図を書く場合[Globalモードでなくinstanceモードを使う必要がある](https://github.com/processing/p5.js/wiki/Global-and-instance-mode)ので何かしらの変換機構をつくるか、ユーザーに常にinstanceモードで書かせることにするか、設計の選択があるので一旦放置です。

