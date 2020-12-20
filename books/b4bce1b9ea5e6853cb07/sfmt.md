---
title: SIMD-oriented Fast Mersenne Twister (sfmt crate)
---

ここでは [std::arch](https://doc.rust-lang.org/core/arch/x86_64/index.html) を使ったメルセンヌツイスタ実装である[sfmt crate](https://github.com/rust-math/rust-sfmt)を例に、`rand_core`の使い方を見ていきましょう。

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
