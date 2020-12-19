---
title: 乱数を生成する(rand crate)
---

この節は [Rust Advent Calendar 2018 12/9の記事](https://qiita.com/termoshtt/items/6e2ff724e6da86963aa9) の転載です

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

rand_core crate
----------------
ここでは[std::archによるSIMD](https://qiita.com/termoshtt/items/a1d3af42bc01c88273c8)を使ったメルセンヌツイスタ実装である[sfmt crate](https://github.com/termoshtt/rust-sfmt)を例に、`rand_core`の使い方を見ていきましょう。

sfmt crateは[オリジナルのSFMT(SIMD-oriented Fast Mersenne Twister)](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/SFMT/)をRustで書き直したものです。ただし実装してあるメルセンヌツイスタは内部状態が19937bit、周期が$2^{19937}-1$のものだけです。これは多くのモンテカルロシミュレーションで十分な周期ですが、CSPRNGでは無いのでセキュリティの必要な場面では使えません。
Double precision SIMD-oriented Fast Mersenne Twister (dSFMT)と呼ばれる直接f64を生成するアルゴリズムをSFMTとあわせた方法がモンテカルロシミュレーションの界隈では有名ですが、rand crateでは既にこのアルゴリズムでビット列からf64を生成しているのでsfmt crateには特に実装していません。

```rust
/// State of SFMT
///
/// This struct implements random number generation through `rand::Rng`.
#[derive(Clone)]
pub struct SFMT {
    /// the 128-bit internal state array
    state: [i32x4; sfmt::SFMT_N],
    /// index counter to the 32-bit internal state array
    idx: usize,
}
```

まず`sfmt::SFMT`として乱数の内部状態を表現する構造体を定義し、これに対してrand_coreの提供するtraitを実装していきます。

```rust
impl SeedableRng for SFMT {
    type Seed = [u8; 4];

    fn from_seed(seed: [u8; 4]) -> Self {
        let mut sfmt = SFMT {
            state:  [zero(); sfmt::SFMT_N],
            idx: 0,
        };
        let seed = unsafe { *(seed.as_ptr() as *const u32) };
        sfmt::sfmt_init_gen_rand(&mut sfmt, seed);
        sfmt
    }
}
```

[SeedableRng](https://rust-random.github.io/rand/rand_core/trait.SeedableRng.html)その名の通り乱数のSeedから初期化するためのcrateです。SFMTの初期化はオリジナルのCにあわせて`[u8; 4]`から初期化していますが、`SeedableRng::Seed`を長くする事もできます。

```rust
impl RngCore for SFMT {
    fn next_u32(&mut self) -> u32 {
        if self.idx >= sfmt::SFMT_N32 {
            self.gen_all();
        }
        self.pop32()
    }

    fn next_u64(&mut self) -> u64 {
        if self.idx >= sfmt::SFMT_N32 - 1 {
            // drop last u32 if idx == N32-1
            self.gen_all();
        }
        self.pop64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)   // next_u32を使って埋める
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
```

`rand_core::RngCore`が`rand::Rng`の機能を提供するためのtraitです。ここでは`SFMT::gen_all()`で生成した乱数を順次`u32`や`u64`として切り出しています。他の型への変換はtraitのデフォルト実装をそのまま採用しています。

この二つのtraitを実装することでrandを通して使う事ができます

```rust
use rand::{Rng, FromEntropy};
let mut rng = sfmt::SFMT::from_entropy();
let r = rng.gen::<u32>();
```

最後に
------
これで基本的な使い方について説明したので、ドキュメントを読めると思います：

- [The Rust Rand Book](https://rust-random.github.io/book/intro.html)
- [Crate rand](https://rust-random.github.io/rand/rand/index.html)

皆さんも計画的なAdC活動を！

