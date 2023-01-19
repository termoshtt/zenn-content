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

`[lib]`だけはパッケージ(crate)に対して1つしか定義できず、他のものは上で述べたように複数定義できます。これらは[プロジェクトのディレクトリ構成](https://doc.rust-lang.org/cargo/guide/project-layout.html)に応じて[自動的に補完](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery)されます。例えば`src/lib.rs`は`[lib]`のルートとみなされ、`src/bin/xxx.rs`というファイルがあれば対応する`[[bin]]`が補完されます。なのでこの記事で述べる説明は通常は必要なく、追加で特殊な設定を行いたいときに追加します。

これらのセクションでは共通のフィールドが使われます。以下はそれぞれのフィールドとターゲットの組み合わせに付いて、

- デフォルト値がある場合はデフォルト値
- ディレクトリ構成から自動的に補完される値の場合は `*`
- 指定出来ない、指定しても効果が無い場合は `-`

を示した表です：

| field               | `[lib]`      | `[[bin]]` | `[[example]]` | `[[test]]` | `[[bench]]` |
|:--------------------|:------------:|:---------:|:-------------:|:----------:|:-----------:|
| `name`              | crate name   | *         | *             | *          | *           |
| `path`              | *            | *         | *             | *          | *           |
| `test`              | true         | true      | false         | true       | false       |
| `bench`             | true         | true      | false         | false      | true        |
| `doc`               | true         | true      | false         | false      | false       |
| `doctest`           | true         | -         | -             | -          | -           |
| `proc-macro`        | false        | -         | -             | -          | -           |
| `harness`           | true         | true      | true          | true       | true        |
| `crate-type`        | `["lib"]`    | -         | `["bin"]`     | -          | -           |
| `required-features` | -            | `[]`      | `[]`          | `[]`       | `[]`        |

加えて`plugin`, `edition`がありますが、これらはほぼ使うことは無いので省略します。

`name`, `path` fields
----------------------
基本的に自動的に補完されます。特殊なディレクトリ構成をする場合、例えば`src/bin/`以外に実行ファイルとなるコードを置きたい時等に使います。

`crate-type` field
-------------------
crateをビルドした出力を指定します。次の値のうち生成するものをリストで指定します。

- `bin`
- `lib`
- `rlib`
- `dylib`
- `cdylib`
- `staticlib`
- `proc-macro`

`proc-macro` field
-------------------
手続きマクロ(proc-macro)を提供するcrateで使います。

```toml
[lib]
proc-macro = true
# 他のフィールドは自動的に補完される値を使う
```

この場合`crate-type`は強制的に`["proc-macro"]`になります。

`doc` field
------------
`cargo-doc`コマンドでドキュメントを生成するかどうかを変更できます。

`[lib]`と`[[bin]]`の場合にはデフォルトで`true`になり他では`false`になりますが、特殊ルールとして`[[bin]]`の場合にターゲット名がcrate名と一致する場合はデフォルト値が`false`になります。

`test`, `bench` fields
-----------------------
それぞれ`cargo-test`及び`cargo-bench`コマンドで実行するかどうかを変更できます。`test = true`の場合、`cargo-test`を実行するとそのターゲットに含まれる`#[test]`で修飾された関数を探してそれを実行します。同じように`bench = true`だと`#[bench]`で修飾された関数を探してそのベンチマークをとりますが、この機能はnightlyの為stable toolchainでは使えません。この`#[test]`や`#[bench]`を順番に実行して結果を表示するランタイムの事を`harness`と呼んでいます。

独自のテストやベンチマークフレームワークを使う場合、実行ファイルに追加で引数を与えるケースがあります。例えば次に述べる`criterion`ではベンチマーク結果に名前をつけて保存する場合にその名前(例えば`main`)を次の様に指定します：

```shell
cargo bench -- --save-baseline main
```

中間の`--`は`cargo-bench`に与える引数と、`cargo-bench`が起動したプログラムに対する引数を区別する仕切りです。この時`[lib]`の`bench` fieldはデフォルトで`true`なので、`cargo bench`によってたとえ`#[bench]`で修飾された関数が1つも無くても`[lib]`を対象としてベンチマークを実行されます。すると上の引数はサポートされていないので実行に失敗します。これを防ぐには次の様にします：

```toml:Cargo.toml
[lib]
bench = false
```

他のfieldは空のままにしておけばデフォルトの値が使われます。

`harness` field
----------------
Rust toolchainに含まれる`libtest`によるテスト・ベンチマークのドライバを使用するかどうか変更できます。`harness = false`にすると`#[test]`や`#[bench]`を収集せずに、`path`に指定されたファイルの`main`関数を実行します。この機能により独自のテストやベンチマークスイートを言語ランタイムでなくサードパーティで実装できます。例えば代表的なベンチマークスイートである[criterion](https://github.com/bheisler/criterion.rs)はこの機能を使って実現されています。
