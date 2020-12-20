---
title: 乱数を生成する(rand crate)
---

Rustで乱数を生成するには[rand crate](https://github.com/rust-random/rand)を使用します。この記事ではこの使い方について簡単についてまとめます。

疑似乱数生成器(pseudo random number generator, PRNG)
--------
Rustでの使い方を見る前に、疑似乱数についての基本的な用語を確認しておきます。

乱数列の様に見えるが、実際には決定論的な方法で生成されている列のことを疑似乱数と呼びます。一方で物理的な乱数（熱雑音や宇宙線によるもの）を真の乱数と呼んで区別します。

### 周期
決定論的、つまり一つ前の状態から次の状態が完全に決定するPRNGでは、状態が有限個の時必ず有限回で同じ状態に戻ってきてしまいます。例えば状態がN個のビットで表現される乱数生成器は状態が$2^N$通りしかありえない。同じ状態に戻ってくるまでの回数を周期と呼びます。
PRNGの周期を越えて乱数を生成した場合、それは乱数の様に見えて実際には周期的な信号になってしまっているので、乱数を用いたモンテカルロシミュレーション等の用途では長い周期を持つPRNGが必要となります。

### 暗号論的擬似乱数生成器
乱数を鍵生成等のセキュリティが必要な場面で使う場合には暗号論的擬似乱数生成器(Cryptographically secure pseudo random number generator, CSPRNG)と呼ばれるクラスの疑似乱数生成器を使う必要があります。CSPRNGの詳細についてはここでは述べませんが、rand crateにはCSPRNGの要件を満すものも実装されており、他のPRNGと同じように使うことができます。

rand crate
-----------

```toml:Cargo.toml
[dependencies]
rand = "0.6"
```

この記事を書いている2018/12の段階では0.6が最新版です。randは0.4から0.5に上る際に大幅な仕様変更があり、[Upgrade to 0.5](https://rust-random.github.io/book/update-0.5.html)に移行のためのドキュメントがあります（英語）。この際に`rand_core`と呼ばれるcrateが分離され、PRNGを使うユーザーはrand、PRNGを独自に実装する場合はrand_coreを使うような形になっています。

randでは乱数のビットを生成する乱数生成器(rng)と乱数の分布をわけて使うインターフェイスを採用しています

```rust
use rand::Rng;
let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
let i: i32 = rng.gen();           // genはRng traitに定義されている
```

`thread_rng`はその環境で一番速いセキュアな擬似乱数生成器を選択するようになっています。`gen`は

```rust
trait Rng: ... {
    fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>; 
    ...
}
```

のような定義になっており、戻り値に併せて乱数を生成する事ができます。`Standard`はいくつかのプリミティブ型に対して定義されていて以下のように定義されています。

- 整数型(i32, usize等)：可能なすべての値で均等に
- char: ユニコードスカラー値から一様に
- bool: true/falseがそれぞれ0.5
- 浮動小数(f32,f64)：`[0, 1)`の一様分布

https://rust-random.github.io/rand/rand/distributions/struct.Standard.html

これ以外の分布を使用する場合は

```rust
use rand::distributions::{Bernoulli, Distribution};
let d = Bernoulli::new(0.3);
let v = d.sample(&mut rand::thread_rng());
```
https://rust-random.github.io/rand/rand/distributions/struct.Bernoulli.html

のように`Distribution::sample`に対してRNGの`&mut`を渡す事で乱数を生成します。この部分のインターフェイスが0.5で大きく変ったので注意してください。

参考リンク
----------

これで基本的な使い方について説明したので、ドキュメントを読めると思います：

- [The Rust Rand Book](https://rust-random.github.io/book/intro.html)
- [Crate rand](https://rust-random.github.io/rand/rand/index.html)
