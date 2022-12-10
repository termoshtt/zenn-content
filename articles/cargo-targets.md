---
title: "cargoにおけるターゲット"
emoji: "🎯"
type: "tech"
topics: ["rust", "cargo"]
published: true
---

Rustのビルド兼パッケージ管理ツールであるcargoの設定ファイル(マニフェストと呼ばれる)`Cargo.toml`はcrateと呼ばれるパッケージを定義していて、この中には`[lib]`や`[[bin]]`という項がありこれらはターゲットと呼ばれます。

https://doc.rust-lang.org/cargo/reference/cargo-targets.html

`[[bin]]`のように`[]`が2重になっているのはTOMLの仕様で、

```toml:Cargo.toml
[[bin]]
name = "a"

[[bin]]
name = "b"
```

と書くとJSONで言えば

```json
{
  "bin": [{ "name": "a" }, { "name": "b" }]
}
```

の様にリストとみなされます。ターゲットには次の種類があります：

- `[lib]`
- `[[bin]]`
- `[[example]]`
- `[[test]]`
- `[[bench]]`

`[lib]`だけはパッケージ(crate)に対して一つしか定義できず、他のものは上で述べたように複数定義できます。これらは[プロジェクトのディレクトリ構成](https://doc.rust-lang.org/cargo/guide/project-layout.html)に応じて[自動的に補完](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery)されます。例えば`src/lib.rs`は`[lib]`のルートとみなされ、`src/bin/xxx.rs`というファイルがあれば対応する`[[bin]]`が補完されます。なのでこの記事で述べる説明は通常は必要なく、追加で特殊な設定を行いたいときに追加します。

これらのセクションでは共通のフィールドが使われます。以下はそれぞれのフィールドとターゲットの組み合わせに付いて、

- デフォルト値がある場合はデフォルト値
- ディレクトリ構成から自動的に補完される値の場合は `*`
- 指定出来ない、指定しても効果が無い場合は `-`

を示した表です：

| field             | `[lib]`      | `[[bin]]` | `[[example]]` | `[[test]]` | `[[bench]]` |
|:------------------|:------------:|:---------:|:-------------:|:----------:|:-----------:|
| name              | crate name   | *         | *             | *          | *           |
| path              | *            | *         | *             | *          | *           |
| test              | true         | true      | false         | true       | false       |
| doctest           | true         | -         | -             | -          | -           |
| bench             | true         | true      | false         | false      | true        |
| proc-macro        | false        | -         | -             | -          | -           |
| harness           | true         | true      | true          | true       | true        |
| crate-type        | `["lib"]`    | -         | `["bin"]`     | -          | -           |
| required-features | -            | `[]`      | `[]`          | `[]`       | `[]`        |


