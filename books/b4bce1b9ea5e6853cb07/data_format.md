---
title: ディスクにデータを保存する
---

数値計算ではたくさんのデータを扱いますが、それらは一時的・永続的にディスクに保存します。ディスクにデータが保存できると個々のプロセスは各々の役割に集中出来ます。例えば

- 設定ファイルを読み込んで時間発展を計算して、時系列データをディスクに保存する
- 時系列データをディスクから読み込んで統計量を計算する
- 時系列データを可視化する
- 時系列データと設定ファイルを読み込んで続きの時系列を計算する

等の操作を別々のプロセス・プログラムで行うことが出来ます。例えば時間発展は性能が必要なのでRustで行って可視化はPythonやParaViewのような専用のソフトウェアを使うこともあるでしょう。この時データをどうやって保存するのかが重要になります。

ディスク上にデータを保存するには、データをシリアライズ(直列化)する必要があります。用途に応じて様々なデータフォーマットが提案され使用されています。それらの性質によって分類して議論する事も可能ですが、まずはよく普及している具体例を見ていきましょう。

JSON
-----
まずは単純なJSONから見ていきましょう。JSONは単純なテキストで構造化された記述できるフォーマットです。例えば
```json
{
  "input": "data.json",
  "step": 2,
  "field": [0.0, 0.0, 0.0, 0.1]
}
```
のように文字列や整数、浮動小数点数とそのリスト及びマップを表現出来ます。JSONと言う名前はJavaScript Object Notationの略称ですが、現在ではJavaScriptに限らず非常に広い範囲で使われています。

RustからJSONを使うには[serde_json](https://docs.rs/serde_json/latest/serde_json/index.html) crateが便利です。

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
  input: String,
  step: usize,
  field: Vec<f64>,
}

// JSON文字列からDataを作る
let data: Data = serde_json::from_str(r#"
{
  "input": "data.json",
  "step": 2,
  "field": [0.0, 0.0, 0.0, 0.1]
}
"#).unwrap();
assert_eq!(data, Data {
  input: "data.json".to_string(),
  step: 2,
  field: vec![0.0, 0.0, 0.0, 0.1]
});

// DataからJSON文字列にする
let data_str = serde_json::to_string(&data).unwrap();
assert_eq!(data_str, r#"{"input":"data.json","step":2,"field":[0.0,0.0,0.0,0.1]}"#);
```

このように構造体を定義して、`#[derive(serde::Serialize)]`を付けるとシリアライズコードを、`#[derive(Deserialize)]`を付けるとデシリアライズ(シリアライズしたものから復元する)コードを生成してくれるので、`serde_json::from_str`で文字列から`Data`型にデシリアライズすることができます。

serdeというのはユーザー定義の構造体に対してJSONに限らず様々なシリアライズ・デシリアライズ実装をするためのフレームワークです。上の例では`serde::Serialize`が`Data`型に対して実装されていて、これはserde_jsonが提供しているのは`serde_json::from_str`と`serde_json::to_string`だけであることに注意してください。serdeではこのように外部のcrateによってデータフォーマットを追加することができます。JSONの他にも[YAML](https://github.com/dtolnay/serde-yaml)や[TOML](https://docs.rs/toml/latest/toml/)といった設定ファイルによく使われるフォーマットやバイナリ形式のJSONとも言える[MessagePack](https://github.com/3Hren/msgpack-rust)や[BSON](https://github.com/mongodb/bson-rust)、あるいは[PythonのPickleへの変換出来るcrate](https://github.com/birkenfeld/serde-pickle)が開発されています。詳しくは[serdeのドキュメント](https://docs.rs/serde/latest/serde/index.html#data-formats)を見てください。

Protocol Buffers
----------------
次に代表的なスキーマ付きのデータフォーマットであるProtocol Buffersを見てみましょう。JSONと大きく異なる点として、Protocol Buffersではまず保存するデータを`.proto`の拡張子のついたファイルに記述していきます。

```proto
syntax = "proto3";

package rust_math_book.items;

message Data {
  string input = 1;
  int64 step = 2;
  repeated double field = 3;
}
```

これを`src/items.proto`として保存し、`build.rs`内でコードを生成します。

```rust:build.rs
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}
```

これでコンパイル時の環境変数`OUT_DIR`で指定される位置にRustのコードが生成されるので、`include!`マクロにより読み込みます。

```rust
mod rust_math_book {
    include!(concat!(env!("OUT_DIR"), "/rust_math_book.items.rs"));
}
```

この時のファイル名`rust_math_book.items.rs`は`items.proto`の`package rust_math_book.items`を反映しています。

このようなデータ形式を記述するものをインターフェース定義言語(Interface Description Language; IDL)と呼びます。IDLから各言語向けにそのデータ構造を扱うためのコードが自動生成されます。Protocol Buffersでは公式の`protoc`コマンドによってRustのコードを生成することも出来ますが、今回は[prost](https://docs.rs/prost/latest/prost/) crateを使って生成しましょう。

serdeではRustの構造体からデータのシリアライザ・デシリアライザが導出されていたのである意味Rustの構造体の定義が`.proto`ファイルと同じ役割を果たしていたと言えます。Protocol Buffersのようにスキーマが独立して存在することにより特定の言語に依存しないデータ形式を定義する事ができます。例えばPyTorch等のライブラリ間でNeural Networkのモデルを交換するためのOpen Neural Network Exchange (ONNX)でもProtocol Buffersが採用されています。
https://github.com/onnx/onnx

### バイナリデータの互換性
数値計算では研究が進むにつれて保存するデータが増えたり減ったりすることがよくあります。すると保存されたデータ形式毎の互換性の問題が発生します。例えばあるバージョンにおいては存在していた`"time"`という浮動小数点数の値が別のバージョンでは無くなっていて代わりに整数のステップ数`"step"`になっているかもしれません。

Tar
----
少し趣向を変えてアーカイブ方式であるTarについて見てみましょう。シリアライズ方式としてTarを見るとこれは個別にシリアライズされたデータを一つのファイルとしてまとめる役割があります。例えば時系列データを保存することを考えると、各時間ステップに応じてデータがあるのでそれらを個別にシリアライズした後全体をまとめるものが必要です。

