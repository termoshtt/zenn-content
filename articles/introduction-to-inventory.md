---
title: "inventory crateを使って複数のproc-macroの結果を統合する"
emoji: "🧪"
type: "tech"
topics: ["rust", "pyo3"]
published: false
publication_name: "jij_inc"
---

この記事は[Jij Inc. Advent Calendar 2023](https://qiita.com/advent-calendar/2023/jij_inc_2023)の12日目の記事です。

この記事ではinventory crateの使い方と大雑把な仕組み、そして応用方法について議論します。

https://github.com/dtolnay/inventory

# `collect!` / `submit!` / `iter`

READMEにあるようにある大規模なCLIパッケージを開発しているとしましょう。このパッケージは様々なサブコマンドを持ち、それぞれに対して独自の処理を行います。このパッケージを開発しているときに、サブコマンドを追加するたびに、それを実行するための関数を`main.rs`に追加していくことになります。それぞれのサブコマンドを実装すると自動的に収集してくれる機能があれば、それを使ってサブコマンドを追加するだけで済み、追加し忘れることもありません。

inventoryではまず収集に使う型を用意して、それを`collect!`マクロで収集できるようにします。例えば次の `Flag` という型を収集するとしましょう：

```rust
pub struct Flag {
    short: char,
    name: &'static str,
}

impl Flag {
    pub const fn new(short: char, name: &'static str) -> Self {
        Flag { short, name }
    }
}

inventory::collect!(Flag);
```

この定義を参照して各サブコマンドを実装するときに、`submit!`マクロを使って登録します：

```rust
inventory::submit! {
    Flag::new('v', "verbose")
}
```

このコードはcrate内のたくさんの場所に散らばることになります。最後これらを使って`main`関数を実装する際には`iter`を使います：

```rust
for flag in inventory::iter::<Flag> {
    println!("-{}, --{}", flag.short, flag.name);
}
```

# 動作原理

各`Flag`の初期化自体は`const fn`で行われるので特に不思議なことはありません。不思議なのは「`submit!`されたすべての`Flag`を収集できる」ということです。このリストはどこから来るのでしょうか？　いつ`submit!`されたのでしょう？

これは実行時の`main`関数が始まる前に収集されます。実はC++のグローバル変数の初期化と同じリンカの機能を使ってこれが実現されています。

## ELFの `.init_array` セクション

リンカとローダの話になるのでOS毎にいくつか差異があります。ここではLinuxの場合を考えましょう。Linuxでは実行ファイルのフォーマットはELFが使われますが、ELFには `.init_array` というセクションがあります。このセクションには実行ファイルの起動時に実行される関数のポインタが並んでいます。これらの関数は実行ファイルの起動時に実行されます。Rustでこの場所に関数を置くには `#[link_section = ".init_array"]` を使います。

https://doc.rust-lang.org/reference/abi.html#the-link_section-attribute

```rust
static mut COUNTER: u32 = 0;

unsafe extern "C" fn __ctor() {
    unsafe { COUNTER += 1 }
}

#[link_section = ".init_array"]
static __CTOR: unsafe extern "C" fn() = __ctor;

fn main() {
    println!("COUNTER: {}", unsafe { COUNTER });
}
```

Linuxでこれを実行すると

```text
COUNTER: 1
```

となります。これは`main`関数が実行される前に`__ctor`が実行されたことを意味します。なお `__ctor` 関数中では `print!` や `println!` は使えないので注意してください。

他のOSにおいても同じような機能があります。例えばWindowsでは `#[link_section = ".CRT$XCU"]`、macOSでは `#[link_section = "__DATA,__mod_init_func"]` となります。ただし対応する機能がWebAssemblyにはないので、WebAssemblyでは使えません。

# proc-macroと組み合わせる

おおまかな動作機構を理解したところで応用編です。inventoryを使うことでproc-macroの弱点である全体の情報を収集できないという点を補うことができます。例えばPyO3の`multi-pymethods` featureを見てみましょう。

https://pyo3.rs/v0.20.0/features.html#multiple-pymethods

これは1つの `#[pyclass]` に対して複数の `#[pymethods]` を定義できるようにする機能です。

```rust
#[pyclass]
pub struct Foo {}

// 一つ目
#[pymethods]
impl Foo {
    #[new]
    fn new() -> Self {
        todo!()
    }
}

// 二つ目
#[pymethods]
impl Foo {
    #[getter]
    fn get(&self) -> PyResult<u32> {
        todo!()
    }
}
```

PyO3は共有ライブラリのロード時にPython C APIを使ってクラスを作る機能を提供しますが、これはクラスを作る際にそのメンバー関数を全て列挙しておく必要があります。つまり `#[pyclass]` proc-macroで生成されるコード中で `inventory::collect!`し、`#[pymethods]` proc-macroで生成されるコード中で `inventory::submit!` を行えば、Python C APIの呼び出しを行うコード中で `inventory::iter` を使ってメンバー関数を列挙できるようになります。

このように複数に分割できるようになることで開発者はコードを分割しやすくなります。例えばRustで実装した機能をPythonからも見えるようにするための操作は多くが単調な繰り返しになるのでcustom-derive `#[derive(MyTrait)]`を使ってPython用の関数を生成したくなります。この際custom-derive内で新たに `#[pymethods]` を使ったコードを生成できるようになります。

# 最後に

＼Rustエンジニア募集中！／
株式会社Jijでは、数学や物理学のバックグラウンドを活かし、量子計算と数理最適化のフロンティアで活躍するRustエンジニアを募集しています！
詳細は下記のリンクからご覧ください。 **皆様のご応募をお待ちしております！**
https://open.talentio.com/r/1/c/j-ij.com/pages/51062

JijのXのフォローもよろしくお願いします！

https://twitter.com/Jij_Inc_JP/status/1722874215060349290
