---
title: "Rustの型情報からPythonの型ヒントを生成する"
emoji: "📌"
type: "tech"
topics: ["pyo3", "rust", "python"]
published: false
publication_name: "jij_inc"
---

[前回の記事](https://zenn.dev/jij_inc/articles/pyo3-mannually-type-stub-file) では手動で stub file (`*.pyi`) を作成しましたが、今回はRust側の情報を使って半自動生成するツールを紹介します。

https://github.com/Jij-Inc/pyo3-stub-gen

# 使い方

この記事の目的は使い方の網羅的な説明ではなくて設計思想の解説ですが、使い方がわからないと設計思想も理解しづらいと思うので先に簡単なケースに対して使い方を説明します。詳しい使い方は [README](https://github.com/Jij-Inc/pyo3-stub-gen/blob/main/README.md) や [Examples](https://github.com/Jij-Inc/pyo3-stub-gen/tree/main/examples) を参照してください。

ここで説明するプロジェクト構成は以下のようになります。`maturin`の[Pure Rust layout](https://www.maturin.rs/project_layout.html#pure-rust-project)になっていることに注意してください。

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

## 1. PyO3 拡張本体に型ヒント生成のためのマクロを追加する

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

## 2. stub 生成用実行ターゲットを追加する

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

# 設計思想

このプロジェクトは正直ちょっといくつか無茶な点があって

## 動作原理

## 開発体制

このプロジェクトは現在ほぼClaude Codeがコードを書いています。
