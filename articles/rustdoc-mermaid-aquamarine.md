---
title: "rustdoc中でmermaidを使って図を書く"
emoji: "🦀"
type: "tech"
topics: ["Rust", "Mermaid"]
published: true
---

[mermaid][mermaid]はテキストベースでフローチャート図などを書くためのJavaScriptライブラリです。
![mermaid-header-image](https://mermaid-js.github.io/mermaid/img/header.png)
([mermaid公式ページ][mermaid]より)これは例えば[GitLabでは標準でMarkdown処理系に組み込まれており](https://docs.gitlab.com/ee/user/markdown.html#mermaid)、次の様にMarkdown中に記述するとその位置に図を出してくれます。
~~~
```mermaid
graph TD;
  A-->B;
  A-->C;
  B-->D;
  C-->D;
```
~~~
![gitlab-mermaid-image](https://raw.githubusercontent.com/termoshtt/zenn-content/main/articles/gitlab-mermaid-graph-screenshot.png)

この記事ではmermaidをrustdoc中で使う方法についてまとめます。

[mermaid]: https://mermaid-js.github.io/mermaid/

Aquamarine crate
-----------------

結論から先に書くと、[aquamarine][aquamarine]というcrateが存在し、これを使って次の様にAttributeを書くと使えます：

~~~rust
#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
/// graph LR
///     s([Source]) --> a[[aquamarine]]
///     r[[rustdoc]] --> f([Docs w/ Mermaid!])
///     subgraph rustc[Rust Compiler]
///     a -. inject mermaid.js .-> r
///     end
/// ```
pub fn example() {}
~~~
![aquamarine-image](https://raw.githubusercontent.com/mersinvald/aquamarine/master/resources/light.png)
([aquamarineのREADME][aquamarine]より)

ここで`#[cfg_attr(cond, attr)]`は[条件付きコンパイルの文法][cfg_attr]で、条件`cond`の時だけ`#[attr]`と同じ様に振る舞います。`rustdoc`でコンパイルするとき、条件`doc`が設定されるため：

~~~rust
#[aquamarine::aquamarine]
/// ```mermaid
/// graph LR
///     s([Source]) --> a[[aquamarine]]
///     r[[rustdoc]] --> f([Docs w/ Mermaid!])
///     subgraph rustc[Rust Compiler]
///     a -. inject mermaid.js .-> r
///     end
/// ```
pub fn example() {}
~~~

つまりドキュメントのビルド時だけこのようにAttributeが展開されるわけです。
この`aquamarine::aquamarine`はproc-macroで、ドキュメント句(`///`から始まる部分、`#[doc = "..."]`としてAttribute関数のAttributeとしてproc-macro側に渡される)を解析してその中の`mermaid`部分だけを抜き出してmermaid APIが読める形に変換します。mermaid APIはページロード時に`<div class="mermaid">`なHTML要素を置き換えるJavaScriptなので、これでrustdocの生成したHTMLをブラウザで開いたらMarkdown内のmermaidにしたがって図が描画されます。

[aquamarine]: https://github.com/mersinvald/aquamarine
[cfg_attr]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
