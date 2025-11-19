---
title: "Rustの型情報からPythonの型ヒントを生成する"
emoji: "📌"
type: "tech"
topics: ["pyo3", "rust", "python"]
published: false
publication_name: "jij_inc"
---

:::message
この記事は [Jij Advent Calendar 2025](https://qiita.com/advent-calendar/2025/jij_inc_2025) の記念すべき第１日目の記事です 🎉
:::

私が入社直後くらいに書いた2023年の記事 [PyO3拡張にPythonの型ヒントを付ける](https://zenn.dev/jij_inc/articles/pyo3-mannually-type-stub-file) の続き兼OSSの紹介記事です。前回の記事では手動で stub file (`*.pyi`) を作成しましたが、今回はRust側の情報を使って半自動生成するツールである `pyo3-stub-gen` を紹介します。

https://github.com/Jij-Inc/pyo3-stub-gen

これはJijのプロダクトである [JijModeling](https://pypi.org/project/jijmodeling/) や [OMMX](https://github.com/Jij-Inc/ommx) 等で使われています。実はこのプロジェクトは私が入社直後からJijModelingのリポジトリで開発していた機能をOSSとして分離・発展させたものです。

# 設計思想

このプロジェクトは **Rustの型システムとPythonの型システムというのは根本的に別物なので、Rust側の型情報からPythonの完全な型ヒントを作成するというのが不可能** であるという点からスタートします。なのでこのプロジェクトでは

- Rust型システムからの自動的なPython型ヒント生成
- 手動での型ヒント定義

という二つのアプローチを組み合わせた半自動生成を目指します。

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

これに次のように `pyo3_stub_gen` のマクロを追加します。

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

### 2. stub 生成用実行ターゲットを追加する

`pyo3-stub-gen` は後述するようにRust側の機能を使って stub file 生成のための情報を生成・集約・出力するので実行ファイルが必要になります。`src/bin/stub_gen.rs` のような実行ファイルターゲットで `stub_info()` を呼び、`stub.generate()?;` を実行します。またこれを実行するため `[lib]` の `crate-type` には `cdylib` に加えて `rlib` を含める必要があります。

```rust:src/bin/stub_gen.rs
use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = pure::stub_info()?; // define_stub_info_gatherer! で定義した関数
    stub.generate()?;
    Ok(())
}
```

```toml:Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]
```

`cargo run --bin stub_gen` を実行すると `pure.pyi` のような stub ファイルが生成されます。`maturin build` するとこの stub が自動的に wheel に同梱されます。

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

`pyo3-stub-gen` crate は主に三つの部分から構成されています。

1. proc-macroによるRustコードから「型ヒント情報を登録するコード」の生成
2. `inventory` crateを使った型ヒント情報の登録と集約
3. 型ヒント情報を使った stub file の生成

## `PyStubType` Trait

まずPyO3で入出力になるRustの型に対して [`PyStubType`](https://docs.rs/pyo3-stub-gen/latest/pyo3_stub_gen/trait.PyStubType.html) トレイトを実装します。これによりRustの型を入力に使った時のPythonの型ヒント (`type_input`) と出力に使った時のPythonの型ヒント (`type_output`) を取得できるようになります。

```rust
pub trait PyStubType {
    fn type_output() -> TypeInfo;
    fn type_input() -> TypeInfo;
}
```

例えば `Vec::<i64>::type_output()` は `list[int]` になりますが、`Vec::<i64>::type_input()` は `typing.Sequence[int]` になります。これはPythonの型システムにおいては出力は具体的な型で良いですが、入力はより抽象的な型で受け取る方が柔軟性が高いからです。このとき `typing` モジュールをインポートする必要があるので、`TypeInfo` には必要なインポート情報も含まれていることに注意してください。`pyo3-stub-gen` crateでは標準ライブラリおよびPyO3、あるいは外部crateの多くの型に対して `PyStubType` トレイトの実装が提供されています。

このようにRustの型に対してPythonの型ヒントを対応させる方法が定まります。あとはユーザーが使いたい型に対して `PyStubType` トレイトを実装すればユーザーが定義した型に対しても型ヒントを生成できるようになります。

この方法の利点はRustの型システムに基づいて型ヒントを生成できることです。例えばTraitを使わない別の方法としてproc-macroの段階で `String` 型に対して `str` を直接返すという方法論はありえますが、proc-macroは例えば `use A as A_` のようなリネームすら追従できないので、この方針はすぐに破綻します。proc-macroはあくまで「型を表すトークン列」しかもらえないので、これを

```rust
<#type_token_stream as PyStubType>::type_output
```

という単なるトークン列に変換するという限定的な責任のみに制限できるのが大きな利点です。

## 型ヒントの登録

次に定義されている全部の型ヒント情報を集約してstub fileを生成するために、`inventory` crateを使って型ヒント情報を登録・集約する仕組みを見ていきましょう。

:::message
inventoryについては以前の記事、[inventory crateを使って複数のproc-macroの結果を統合する](https://zenn.dev/jij_inc/articles/introduction-to-inventory)を参照してください。
:::

`pyo3-stub-gen` ではPyO3のクラスに対して `#[gen_stub_pyclass]` マクロを使うことでそのクラスをstub file生成の対象として登録できます。例えば次のようなPyO3クラスがあるとします。

```rust
#[gen_stub_pyclass]
#[pyclass(module = "my_module", name = "MyClass")]
struct MyClass {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    description: Option<String>,
}
```

この `#[gen_stub_pyclass]` proc-macroは次の二つを生成します。

- 上述した `impl PyStubType for MyClass` の実装
- 次に示す `inventory::submit!` マクロによる型情報の登録

```rust
inventory::submit!{
    PyClassInfo {
        struct_id: std::any::TypeId::of::<MyClass>,
        module: Some("my_module"),
        pyclass_name: "MyClass",
        getters: &[
            MemberInfo {
                name: "name",
                r#type: <String as ::pyo3_stub_gen::PyStubType>::type_output,
                doc: "Name docstring",
                default: None,
                deprecated: None,
            },
            MemberInfo {
                name: "description",
                r#type: <Option<String> as ::pyo3_stub_gen::PyStubType>::type_output,
                doc: "Description docstring",
                default: None,
                deprecated: None,
            },
        ],
        doc: "Docstring used in Python",
        ...
    }
}
```

基本的にはクラスの名前やメンバーの名前や型、ドキュメント文字列を `PyClassInfo` 構造体に詰めて `inventory::submit!` マクロで登録しています。`PyClassInfo` は `inventory::collect!`　によってリンク時に集約され、実行時に取得できるようになります。

ここで少しテクニカルな点があって、`submit!` は `const` なものしか送信できません。しかし `PyStubType::type_output` はtraitの関数なので `const` にできません。そこで評価せずに関数ポインタとして送信し、`collect!` して集約する時に評価します。`TypeId::of::<MyClass>`なども同様です。`struct_id` はPyO3のclassとmethodsを対応づけるのに使われます。

## stub file の生成

inventoryで収集された情報を元にstub fileを実際に生成するのはほぼ自明なので省略します。

## 手動での型ヒントの場合

手動での型ヒント指定の場合は `#[gen_stub_pyfunction(python = "...")]` マクロを使いますが、この場合はproc-macro内でPythonコードをASTにパースし、`inventory::submit!` マクロで登録する `PyFunctionInfo` 構造体などを生成します。

# 最後に
（ここに採用情報を載せる）
