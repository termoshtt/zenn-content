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

このように構造体を定義して、`#[derive(serde::Serialize)]`を付けるとシリアライズコードを、`#[derive(Deserialize)]`を付けるとデシリアライズ(シリアライズしたものから復元する)コードを生成してくれるので、`serde_json::from_str`で文字列から`Data`型にデシリアライズできます。

serdeというのはユーザー定義の構造体に対してJSONに限らず様々なシリアライズ・デシリアライズ実装をするためのフレームワークです。上の例では`serde::Serialize`が`Data`型に対して実装されていて、これはserde_jsonが提供しているのは`serde_json::from_str`と`serde_json::to_string`だけであることに注意してください。serdeではこのように外部のcrateによってデータフォーマットを追加できます。JSONの他にも[YAML](https://github.com/dtolnay/serde-yaml)や[TOML](https://docs.rs/toml/latest/toml/)といった設定ファイルによく使われるフォーマットやバイナリ形式のJSONとも言える[MessagePack](https://github.com/3Hren/msgpack-rust)や[BSON](https://github.com/mongodb/bson-rust)、あるいは[PythonのPickleへの変換出来るcrate](https://github.com/birkenfeld/serde-pickle)が開発されています。詳しくは[serdeのドキュメント](https://docs.rs/serde/latest/serde/index.html#data-formats)を見てください。

Protocol Buffers
----------------
次に代表的なスキーマ付きのデータフォーマットであるProtocol Buffersを見てみましょう。JSONと大きく異なる点として、Protocol Buffersではまず保存するデータを`.proto`の拡張子のついたファイルに記述していきます。例えば上のJSONの場合の構造体は次のように記述します：

```protobuf:src/items.proto
// proto2からproto3で文法の非互換があるのでどちらで書くか指定する
syntax = "proto3";

// パッケージという単位で管理する
package rust_math_book.items;

// データ構造はmessageと呼ぶ
message Data {
  // 文字列型のメンバーを追加する。通し番号`1`を付けることで将来の定義変更に備える
  string input = 1;
  // 整数型はサイズを含めた名前になっている
  int64 step = 2;
  // `repeated`は0回以上の繰り返しを表す
  repeated double field = 3;
}
```

詳しい文法は[公式ページ](https://protobuf.dev/programming-guides/proto3/)を見てください。このようなデータ形式を記述するものをインタフェース定義言語(Interface Description Language; IDL)と呼びます。IDLから各言語向けにそのデータ構造を扱うためのコードが自動生成されます。今回は[prost](https://docs.rs/prost/latest/prost/) crateを使ってRustコードを生成します。これはProtocol Buffersのコンパイラ`protoc`をサブプロセスとして呼び出すので`protoc`コマンドが存在している必要があります。Ubuntuでは

```shell
sudo apt install protobuf-compiler
```

macOS/Homebrewでは

```shell
brew install protobuf
```

Windowsでは[Releaseページ](https://github.com/protocolbuffers/protobuf/releases)からインストーラをダウンロードします。これを`src/items.proto`として保存し、`build.rs`内でコードを生成します。

```rust:build.rs
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    Ok(())
}
```

これでコンパイル時の環境変数`OUT_DIR`で指定される位置にRustのコードが生成されるので、`include!`マクロにより読み込みます。

```rust
mod rust_math_book {
    // ファイル名`rust_math_book.items.rs`は`items.proto`の`package rust_math_book.items`を反映している
    include!(concat!(env!("OUT_DIR"), "/rust_math_book.items.rs"));
}

// include!したファイルの中で定義されている構造体
use rust_math_book::Data;
// prostが生成した構造体はMessage traitを実装しているのでこれで操作する
use prost::Message;

let data = Data {
  input: "data.bin".to_string(),
  step: 2,
  field: vec![0.0, 0.0, 0.0, 0.1]
};
// バイナリデータにシリアライズする
let encoded: Vec<u8> = data.encode_to_vec();
// バイナリデータから復元する
let data2: Data = Message::decode(encoded.as_slice()).unwrap();
assert_eq!(data2, data);
```

### データスキーマ
重要な視点は、アプリケーション(例えば具体的な数値計算プログラム)がどんなデータを保存する必要があるかという部分と、どうやってデータをファイルに保存するかは独立して考える事が出きるという点です。例えば数値計算の設定ファイルを設計するとき、ユーザーはそのファイルに複数の事項を記入してプログラム側でそれを解釈する必要がありますが、この時プログラムは次の二つをチェックする必要があります：

- どこからどこまでが何の情報かが分かる
- 計算に必要な情報を含んでいる

まず前者はほとんどのアプリケーションにおいて共通です。これが例えばJSONのような共通のデータフォーマットが普及した理由で、この段階では可能な限りどんなデータでも入れる事が出きるように設計されています。一方で後者を解決するためのものがスキーマです。典型的には入力の検査は必要な名前を持ったフィールドが例えば整数のような特定の型を持っているかどうかを調べる事になり、これはほとんど自動的に生成できます。

serdeではRustの構造体からデータのシリアライザ・デシリアライザが導出されていたのである意味Rustの構造体の定義が`.proto`ファイルと同じIDLの役割を果たしていたと言えます。Protocol Buffersのようにスキーマが独立して存在することにより特定の言語に依存しないデータ形式を定義できます。例えばPyTorch等のライブラリ間でNeural Networkのモデルを交換するためのOpen Neural Network Exchange (ONNX)でもProtocol Buffersが採用されています。
https://github.com/onnx/onnx
同じような事がJSONを使う場合でも[JSON Schema](https://json-schema.org/)を使うと可能です。
