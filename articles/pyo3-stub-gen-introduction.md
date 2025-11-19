---
title: "Rustの型情報からPythonの型ヒントを生成する"
emoji: "📌"
type: "tech"
topics: ["pyo3", "rust", "python"]
published: false
publication_name: "jij_inc"
---

この記事は [Jij Advent Calendar 2025](https://qiita.com/advent-calendar/2025/jij_inc_2025) の記念すべき第１日目の記事です！

私が入社直後くらいに書いた2023年の記事 [PyO3拡張にPythonの型ヒントを付ける](https://zenn.dev/jij_inc/articles/pyo3-mannually-type-stub-file) の続き兼OSSの紹介記事です。前回の記事では手動で stub file (`*.pyi`) を作成しましたが、今回はRust側の情報を使って半自動生成するツールである `pyo3-stub-gen` を紹介します。

https://github.com/Jij-Inc/pyo3-stub-gen

これはJijのプロダクトである [JijModeling](https://pypi.org/project/jijmodeling/) や [OMMX](https://github.com/Jij-Inc/ommx) 等で使われています。

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

# 最後に
