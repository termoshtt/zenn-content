---
title: 乱数を生成する(rand crate)
---

Rustで乱数を生成するには[rand crate](https://github.com/rust-random/rand)を使用します。この記事ではこの使い方について簡単にまとめます。

疑似乱数生成器(pseudo random number generator, PRNG)
--------
Rustでの使い方を見る前に、疑似乱数についての基本的な用語を確認しておきます。

乱数列の様に見えるが、実際には決定論的な方法で生成されている列のことを疑似乱数と呼びます。一方で物理的な乱数（熱雑音や宇宙線によるもの）を真の乱数と呼んで区別します。

### 周期
決定論的、つまり1つ前の状態から次の状態が完全に決定するPRNGでは、状態が有限個の時必ず有限回で同じ状態に戻ってきてしまいます。例えば状態がN個のビットで表現される乱数生成器は状態が$2^N$通りしかありえません。全ての状態取りうるかどうかは生成器のアルゴリズム毎に異なり、同じ状態に戻ってくるまでの回数を周期と呼びます。

PRNGの周期を越えて乱数を生成した場合、それは乱数の様に見えて実際には周期的な信号になってしまっているので、多くの乱数を生成するモンテカルロシミュレーション等の用途では長い周期を持つPRNGが必要となります。

### 暗号論的擬似乱数生成器
乱数を鍵生成等のセキュリティが必要な場面で使う場合には暗号論的擬似乱数生成器(Cryptographically secure pseudo random number generator, CSPRNG)と呼ばれるクラスの疑似乱数生成器を使う必要があります。CSPRNGの詳細についてはここでは述べませんが、rand crateにはCSPRNGの要件を満すものも実装されており、他のPRNGと同じように使うことができます。

rand crate
-----------

```toml:Cargo.toml
[dependencies]
rand = "0.6"
```

この記事を書いている2018/12の段階では0.6が最新版です。`rand`は0.4から0.5に上る際大幅な仕様変更があり、[Upgrade to 0.5](https://rust-random.github.io/book/update-0.5.html)に移行のためのドキュメントがあります（英語）。この際に`rand_core`と呼ばれるcrateが分離され、PRNGを使うユーザーは`rand`、PRNGを独自に実装する場合は`rand_core`を使うような形になっています。

`rand`では乱数のビットを生成する乱数生成器(RNG)と乱数の分布をわけて使うインタフェースを採用しています：

```rust
use rand::Rng; // Rng::genを使うためにtraitをuseしておく

let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
let i: i32 = rng.gen();           // 整数値の乱数を生成する
let f: f32 = rng.gen();           // 浮動小数点数の乱数を生成する
```

`thread_rng`はその環境で一番速いセキュアな擬似乱数生成器を選択するようになっています。`gen`は次のような定義になっています：

```rust:ignore
trait Rng: ... {
    fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>; 
    ...
}
```

ここで`&mut self`を受け取っているのは生成機の状態が更新されるためです。疑似乱数生成器は今の状態から次の状態を計算してそこに遷移していきます。なので`Rng::gen`を呼び出す順序は結果に影響します。

ここで型`T`は引数から推定されます。そのため`let i: i32 = rng.gen()`のように型強制構文を使っています。

`Standard: Distribution<T>`という制約はいくつかのプリミティブ型に対して以下の様に定義されています：

- 整数型(`i32`, `usize`等)：可能なすべての値で均等に
- `char`: Unicodeスカラー値から一様に
- `bool`: true/falseがそれぞれ0.5
- 浮動小数点数(`f32`,`f64`)：`[0, 1)`の一様分布

https://rust-random.github.io/rand/rand/distributions/struct.Standard.html

これ以外の分布を使用する場合は次の様に[`rand::distributions`](https://rust-random.github.io/rand/rand/distributions/index.html)にある分布を指定します：

```rust
use rand::distributions::{Bernoulli, Distribution};

let d = Bernoulli::new(0.3).unwrap();
let v = d.sample(&mut rand::thread_rng());
```
https://rust-random.github.io/rand/rand/distributions/struct.Bernoulli.html

`Distribution::sample`に対してRNGの`&mut`を渡す事で乱数を生成します。この部分のインタフェースが0.5で大きく変ったので注意してください。

乱数列を固定する
---------------

上で述べた `rand::thread_rng()` はシステムから得た乱数で安全に初期化を行おうとしますが、用途によっては乱数列を再現したい場合もあります。その場合 `thread_rng` はアルゴリズムが一致することが保証されないので、特定のアルゴリズムを指定して、シード値を使って初期化します：

```rust
use rand::{Rng, SeedableRng};

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    println!("Fixed Random number in f32: {}", rng.gen::<f32>());
}
```

[`SeedableRng::seed_from_u64`](https://rust-random.github.io/rand/rand/trait.SeedableRng.html)は`u64`の整数値をシードとして生成器を初期化する関数です。これはセキュアな用途には使ってはいけないことに注意してください。

参考リンク
----------

より詳しい使い方については以下のドキュメントを読んでください：

- [The Rust Rand Book](https://rust-random.github.io/book/intro.html)
- [Crate rand](https://rust-random.github.io/rand/rand/index.html)
