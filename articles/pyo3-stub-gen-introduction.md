---
title: "Rustの型情報からPythonの型ヒントを生成する"
emoji: "📌"
type: "tech"
topics: ["pyo3", "rust", "python"]
published: true
publication_name: "jij_inc"
---

:::message
この記事は [Jij Advent Calendar 2025](https://qiita.com/advent-calendar/2025/jij_inc_2025) の記念すべき第１日目の記事です 🎉
:::

私が入社直後くらいに書いた2023年の記事 [PyO3拡張にPythonの型ヒントを付ける](https://zenn.dev/jij_inc/articles/pyo3-mannually-type-stub-file) の続き兼OSSの紹介記事です。前回の記事では手動でstub file (`*.pyi`) を作成しましたが、今回はRust側の情報を使って半自動生成するツールである `pyo3-stub-gen` を紹介します。

https://github.com/Jij-Inc/pyo3-stub-gen

これはJijのプロダクトである [JijModeling](https://pypi.org/project/jijmodeling/) や [OMMX](https://github.com/Jij-Inc/ommx) 等で使われています。実はこのプロジェクトは私が入社直後からJijModelingのリポジトリで開発していた機能をOSSとして分離・発展させたものです。

# 設計思想

このプロジェクトは **Rustの型システムとPythonの型システムというのは根本的に別物なので、Rust側の型情報からPythonの完全な型ヒントを作成するというのが不可能** であるという点からスタートします。なのでこのプロジェクトでは

- Rust型システムからの自動的なPython型ヒント生成
- 手動での型ヒント定義

という2つのアプローチを組み合わせた半自動生成を目指します。

:::message
この記事の目的は使い方の網羅的な説明ではなくて設計思想の解説ですが、使い方がわからないと設計思想も理解しづらいと思うので簡単なケースに対して使い方を説明します。詳しい使い方は [README](https://github.com/Jij-Inc/pyo3-stub-gen/blob/main/README.md) や [Examples](https://github.com/Jij-Inc/pyo3-stub-gen/tree/main/examples) を参照してください。
:::

## 自動的なPython型ヒント生成

こちらが基本のモードになるので、まず全体の設定から始めましょう。ここで説明するプロジェクト構成は以下のようになります。`maturin`の[Pure Rust layout](https://www.maturin.rs/project_layout.html#pure-rust-project)になっていることに注意してください。

```
.
├── Cargo.toml           # Rustの設定を記述
├── pyproject.toml       # maturinの設定や他のPythonの設定を記述
├── pure.pyi             # 生成される stub ファイル
└── src
    ├── bin
    │   └── stub_gen.rs  # stub 生成コマンド
    └── lib.rs           # PyO3 拡張本体
```

### 1. PyO3 拡張本体に型ヒント生成のためのマクロを追加する

まず次のようなPyO3拡張があるとします。

```rust
use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn your_module_name(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

これに次のように `pyo3_stub_gen` のマクロ、 `#[gen_stub_pyfunction]` を追加します。

```rust:src/lib.rs
use pyo3::prelude::*;
use pyo3_stub_gen::{derive::gen_stub_pyfunction, define_stub_info_gatherer};

#[gen_stub_pyfunction]  // proc-macroを追加
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn your_module_name(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

// stub 情報収集用の関数を定義
define_stub_info_gatherer!(stub_info /* 関数名 */);
```

最後の `define_stub_info_gatherer!` マクロはstub file生成に必要な情報を集約するための関数を定義します。`#[gen_stub_pyfunction]` をPython側に公開する関数全部につけることになるので、それらを集約するための関数が必要になるからです。

### 2. stub 生成用実行ターゲットを追加する

`pyo3-stub-gen` ではproc-macroとRustの型システムによって型ヒントの情報を生成するので、情報を生成するためにコンパイルが必須であり、その情報を出力するために一度実行する必要があります。`src/bin/stub_gen.rs` のような実行ファイルターゲットで `stub_info()` を呼び、`stub.generate()?;` を実行します。

```rust:src/bin/stub_gen.rs
use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = pure::stub_info()?; // define_stub_info_gatherer! で定義した関数
    stub.generate()?;
    Ok(())
}
```

PyO3のプロジェクトでは通常共有ライブラリを作るだけなので `crate-type = ["cdylib"]` となっていますが、この実行ファイルをビルドして実行するため `rlib` を含める必要があります。

```toml:Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]
```

以上の準備のもとで `cargo run --bin stub_gen` を実行すると `pure.pyi` のようなstubファイルが生成されます。これは `pyproject.toml` などの情報からパスとファイル名が自動的に`maturin` が正しく読める位置に生成されます。

この手続きは正直少し難しいので導入が難しくなりがちですが、いくつかの設計上・技術上の制約からこの形になっています。

## 手動での型ヒント補完

最初に述べたようにRustコードからの自動生成は便利ですが、完全な型ヒントが常に生成できるわけではありません。そこで `pyo3_stub_gen` ではユーザが手動で型ヒントを補完できる仕組みを提供しています。次のようにPythonの構文で手動で型ヒントを指定できます。

```rust
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyfunction(python = r#"
    import collections.abc
    import typing

    def fn_with_callback(callback: collections.abc.Callable[[str], typing.Any]) -> collections.abc.Callable[[str], typing.Any]:
        """Example using python parameter for complete override."""
"#)]
#[pyfunction]
pub fn fn_with_callback<'a>(callback: Bound<'a, PyAny>) -> PyResult<Bound<'a, PyAny>> {
    callback.call1(("Hello!",))?;
    Ok(callback)
}
```

この例ではコールバック関数を受け取ることを期待しているコードですが、実際には `PyAny` 型で受け取っているためRust側からは関数の引数や戻り値の型情報がわかりません。そこで `#[gen_stub_pyfunction]` マクロの `python` 引数にPythonの関数定義を文字列で与えることで、完全な型ヒントを手動で指定しています。

:::message
手動書き換えの場合はRust側の型を使う必要があるケースが多いと思うので、その場合はPythonコード中で `pyo3_stub_gen.RustType` という特別な型を使うと自動的にRust側の情報に置換されます。詳しくは [README](https://github.com/Jij-Inc/pyo3-stub-gen?tab=readme-ov-file#advanced-using-rusttype-marker) の該当箇所を参照してください。
:::

# 動作原理

:::message alert
詳しいことは覚えていないので [DeepWiki](https://deepwiki.com/Jij-Inc/pyo3-stub-gen) に聞いてください(´・ω・｀)
:::

大雑把な使い方を説明したところで動作原理を解説していきましょう。まず素朴な要望として、

```rust
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
```

のコードを見たらこれから

```python
def sum_as_string(a: int, b: int) -> str: ...
```

を作って欲しいわけです。これを実現するにはいくつかの考えるべきことがあります。

- ユーザーが書いたRustのコードを解析しないといけないですよね？
  - proc-macroを使えば良い
- Rustの型からPythonの型ヒントをどうやって取得するのか？ `usize` 等の組み込み型は対応表を持てば良いが `Vec<T>` などはどうすれば？　ユーザー定義型は？
  - Traitを1つ用意してそれ経由で型ヒントを取得するのが良さそう
- 関数毎に生成した型ヒント情報をどうやってモジュールごとのstub fileに集約するのか？
  - 以前の記事 [inventory crateを使って複数のproc-macroの結果を統合する](https://zenn.dev/jij_inc/articles/introduction-to-inventory) で解説したように `inventory` crateを使うのが良さそう

## proc-macroはRustの型が分からない

上の方針で行けそうですが、1つ確認しておく必要があることがあります。proc-macroというのは [`TokenStream`](https://doc.rust-lang.org/proc_macro/trait.TokenStream.html) を受け取って [`TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html) を返す関数なので、**proc-macroの段階ではRustの型システムの情報が全く分からない** という点です。つまり上の例で言えば `usize` や `PyResult<String>` は単なるトークン列としか認識できません。`usize` を `int` に変換するのは名前固定でマップするのも可能ですが、`PyResult<String>` はそもそも `PyResult` が何を表すのか、実は `PyO3` の `PyResult` ではないかもしれず、部分的なRustのコードではそれはわかりません。つまりproc-macroを使ってRustの型からPythonの型ヒントを直接生成するのは不可能です。

これはTraitを1つ噛ませることで簡単に解決できます。次のようなTraitを用意しましょう。

```rust
pub trait PyStubType {
    fn type_output() -> TypeInfo;
    fn type_input() -> TypeInfo;
}
```

`TypeInfo` はPythonの型ヒントを表す構造体です。PyO3はある程度よしなにPythonのクラスをRustの構造体に変換してくれるので、その入力時の変換ルールを反映した `type_input` と出力時の変換ルールを反映した `type_output` の2つの関数を用意しています。例えば `Vec::<i64>::type_output()` は `list[int]` になりますが、`Vec::<i64>::type_input()` はPyO3が `typing.Sequence[int]` なら `Vec<i64>` に変換してくれるのでこれを採用しています。

proc-macroでパースしているので型が入力で使われたのか出力で使われているのかは自明です。そこでproc-macroでは `usize` に対して

```rust
<usize as pyo3_stub_gen::PyStubType>::type_input
```

というトークン列を生成さえすれば、あとはコンパイラがこれを `TypeInfo` を出力する関数として解決してくれます。これなら型のトークンを一切解析する必要がなく、ユーザーがどんなふうにこの形を定義していようとコンパイラが正しく解決してくれます。これでproc-macroでRustの型に対してPythonの型ヒントを対応させる方法が定まります。あとはユーザーが使いたい型に対して `PyStubType` トレイトを実装すればユーザーが定義した型に対しても型ヒントを生成できるようになります。

## `const fn` で初期化したものしか `submit!` できない

もう1つ非自明な技術的な問題があって、 `inventory` crateの `submit!` マクロは `const fn` で初期化されたものしか登録できないという制約があります。そして現在 (2025/12) のStable Rustではtraitの関数を `const fn` にできません。つまり上で行ったように `<#type_tokenstream as PyStubType>::type_output()` のようにtraitの関数を呼び出して得られる `TypeInfo` を `inventory::submit!` マクロで登録できません。

これは回避が簡単で、関数ポインタを送信して `collect!` で集約した後に関数ポインタを評価するようにすれば良いです。最終的には次のようなコードがproc-macroで生成されます。

```rust
::pyo3_stub_gen::inventory::submit! {
    ::pyo3_stub_gen::type_info::PyFunctionInfo {
        name: "sum_as_string",
        parameters: &[
            ::pyo3_stub_gen::type_info::ParameterInfo {
                name: "a",
                kind: ::pyo3_stub_gen::type_info::ParameterKind::PositionalOrKeyword,
                type_info: <usize as ::pyo3_stub_gen::PyStubType>::type_input,
                default: ::pyo3_stub_gen::type_info::ParameterDefault::None,
            },
            ::pyo3_stub_gen::type_info::ParameterInfo {
                name: "b",
                kind: ::pyo3_stub_gen::type_info::ParameterKind::PositionalOrKeyword,
                type_info: <usize as ::pyo3_stub_gen::PyStubType>::type_input,
                default: ::pyo3_stub_gen::type_info::ParameterDefault::None,
            }
        ],
        r#return: <String as pyo3_stub_gen::PyStubType>::type_output,
        doc: "",
        module: None,
        is_async: false,
        deprecated: None,
        type_ignored: None,
        is_overload: false,
        file: file!(),
        line: line!(),
        column: column!(),
        index: 0usize,
    }
}
```

## 手動での型ヒントの場合

手動での型ヒント指定の場合は `#[gen_stub_pyfunction(python = "...")]` マクロを使いますが、この場合はproc-macro内でPythonコードをASTにパースし、`inventory::submit!` マクロで登録する `PyFunctionInfo` 構造体などを生成します。

# AIによる開発

`pyo3-stub-gen` は元々の機構は（2023当時はAIはまだまだ非力だったので）私が自力で実装していますが、最近の開発では [Claude Code](https://code.claude.com/docs/ja/overview) をかなり重用しています。最近のほぼ全てのコードとドキュメント、サンプルコードおよびテストは全てClaude Codeによって生成されています。ただしレビューについては私が行なっています。

現在開発中のJijModeling v2ではかなりヘビーに型ヒントを使いますが、それらの要望をClaude Codeによってまず（動作しない）サンプルコードとして作成させこの段階で詳細なレビューを行い、その後実際に動作するコードを生成させるというワークフローを採用しています。proc-macroのテストには [trybuild](https://docs.rs/trybuild/latest/trybuild/) および [insta](https://docs.rs/insta/latest/insta/) を使っていますが、Claude Codeはこれらの使い方もよく理解して適切にテストコードやスナップショットを生成してくれます。
 
`pyo3-stub-gen` はOSSとして公開されとりあえず動作するようになってからしばらく放置されていましたが、AIを使って省力にメンテナンス出来るようになり、AIにより存続できているプロジェクトといっても過言ではありません。

# 最後に
Jijでは各ポジションを積極的に採用しています！
現在の募集職種は、以下リンクよりご覧いただけます。
カジュアル面談からのスタートも大歓迎ですので、お気軽にご連絡ください。
https://open.talentio.com/r/1/c/j-ij.com/homes/1900
