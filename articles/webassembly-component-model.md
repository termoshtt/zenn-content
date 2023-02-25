---
title: "WebAssemblyコンポーネントモデルを調べる"
emoji: "🔄"
type: "tech"
topics: ["webassembly", "wit"]
published: true
---

Rustの相互運用ABI(`interoperable_abi`)の提案
https://github.com/rust-lang/rust/pull/105586
について調べるはずが、いつのまにかWebAssembly Interface Types (WIT)
https://hacks.mozilla.org/2019/08/webassembly-interface-types/
について調べていたのでとりあえずまとめておきます。

2023/2/11現在Interface Typesの提案はWebAssembly component modelに統合されています。
https://github.com/WebAssembly/component-model

WebAssemblyコンポーネントを使うとき
------------------------------------
このプロジェクトで目標としている[ユースケース](https://github.com/WebAssembly/component-model/blob/4a5b49377f11d59eec5f19868cf04d054070fefc/design/high-level/UseCases.md)にはサーバーレス等のいくつかの場合があがっていますが、今回はCPythonからwasmのコンポーネントを読み込んでその中にある実装を利用するケースを考えましょう。これはCPythonにとっては共有ライブラリを読み込んでその中の関数を呼び出しているのと似ています。これを実現するためにコンポーネントモデルはwasmのコア言語の上に

- [インターフェース定義言語(IDL)](https://github.com/WebAssembly/component-model/blob/4a5b49377f11d59eec5f19868cf04d054070fefc/design/mvp/WIT.md)
- [構造体のバイナリフォーマット](https://github.com/WebAssembly/component-model/blob/4a5b49377f11d59eec5f19868cf04d054070fefc/design/mvp/Binary.md)
- [呼び出し規則(Canonical ABI)](https://github.com/WebAssembly/component-model/blob/4a5b49377f11d59eec5f19868cf04d054070fefc/design/mvp/CanonicalABI.md)

を定めることになります。WebAssembly Interface TypeはこのIDLとしてコンポーネントモデルに統合されました。
これは共有ライブラリの呼び出し、つまりC ABIの場合と比較してみるといいでしょう。この場合IDLとは概ねCのヘッダの事でどの構造体をどの関数にあげるべきかが記述され、バイナリフォーマットとはCの構造体をメモリ上でどのように並べるかを規定し、呼び出し規則とは関数を呼び出す際に引数をどうやって指定してどのアドレスに処理を移すのかを記述するものでした。
wasmコンポーネントの場合にもIDLでデータ構造を記述し、バイナリフォーマットでそれをwasmの線形メモリ上にどう保持するかを定め、Canonical ABIによって関数呼び出しをwasmコア言語上にどうやって翻訳するかを定めます。

共有ライブラリではインターフェースの記述は伝統的にC言語という"IDL"を通じて行われてきました。多くの言語においてC言語というIDLによるインターフェース定義からその言語向けのAPIを生成する仕組みが存在しています。
wasmコンポーネントではIDLとしてWITが採用され、これはテキストによる`*.wit`ファイル形式
```
interface host {
  log: func(msg: string)
}
```
と対応する[wasmテキスト形式](https://developer.mozilla.org/ja/docs/WebAssembly/Understanding_the_text_format)
```
(component
  (import "host" (instance $host
    (export "log" (func (param "msg" string)))
  ))
  ;; ...
)
```
がそれぞれ定義されます。これは構造体や関数定義、あるいは別のWITでの定義を参照する`use`が使えます：
```
interface wasi-fs {
  use pkg.types.{errno}

  record stat {
    ino: u64,
    size: u64,
    // ...
  }

  stat-file: func(path: string) -> result<stat, errno>
}
```
なんとなく[Protocol Buffers](https://github.com/protocolbuffers/protobuf)に似てますね。

WebAssemblyコンポーネントを作るとき
------------------------------------
ではこのWIT定義は誰が作るのでしょうか？Rustでwasmモジュールを作るんだとするとRustの関数定義からそのままWITを生成してくれるとうれしいですね。これを自動でやってくれるのが`wit-bindgen`のようです
https://github.com/bytecodealliance/wit-bindgen
このリポジトリにはRustで作ったWIT定義をC++/Java/TinyGoから使う例も含まれているようですね。

Links
-----
https://gihyo.jp/article/2023/02/tfen008-rust-web-assembly
https://radu-matei.com/blog/intro-wasm-components/
