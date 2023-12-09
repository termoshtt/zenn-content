---
title: "inventoryの使い方"
emoji: "🧪"
type: "tech"
topics: ["rust"]
published: true
---

この記事では inventory crateの使い方と大雑把な仕組み、そして応用方法について議論します。

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

各`Flag`の初期化自体は`const fn`で行われるので特に不思議なことはありません。不思議なのは「`submit!`されたすべての`Flag`を収集できる」ということです。このリストはどこから来るのでしょうか？いつ`submit!`されたのでしょう？

これは実行時の`main`関数が始まる前に収集されます。実はC++のグローバル変数の初期化と同じリンカの機能を使ってこれが実現されています。

## ELFの `.init_array` セクション

TBW

# proc-macroと組み合わせる

さてこれはなぜ必要なのでしょうか？同じcrate内でしか使わないのであれば、`static`変数を使えば良いのではないでしょうか？

