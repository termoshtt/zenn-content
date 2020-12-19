---
title: 多倍長計算を行う (rug crate)
---

[Rug](https://docs.rs/rug/1.6.0/rug/index.html)という多倍長演算ライブラリがあったので[Rumpの例題](https://ja.wikipedia.org/wiki/%E7%B2%BE%E5%BA%A6%E4%BF%9D%E8%A8%BC%E4%BB%98%E3%81%8D%E6%95%B0%E5%80%A4%E8%A8%88%E7%AE%97#Rump%E3%81%AE%E4%BE%8B%E9%A1%8C)をやってみます

```rust
use rug::{ops::Pow, Float};

fn f200(val: f64) -> Float {
    Float::with_val(200, val)
}

fn f(a: Float, b: Float) -> Float {
    let a2 = a.clone().pow(2);
    let b2 = b.clone().pow(2);
    let b4 = b.clone().pow(4);
    let b6 = b.clone().pow(6);
    let b8 = b.clone().pow(8);
    (f200(333.75) - &a2) * &b6
        + &a2 * (f200(11.0) * &a2 * &b2 - f200(121.0) * &b4 - f200(2.0))
        + f200(5.5) * &b8
        + a / (f200(2.0) * b)
}

fn main() {
    println!("{:e}", f(f200(77617.0), f200(33096.0)));
}
```

`rug::Float`に`Copy`が無いのでちょっと面倒になりましたが概ね雑にこんな感じだと思います。

> Rug is a high-level interface to the following GNU libraries:
> - GMP for integers and rational numbers,
> - MPFR for floating-point numbers, and
> - MPC for complex numbers.

と言ってるので[GNU MPFR](https://www.mpfr.org/)のラッパーですね。整数は[num-bigint](https://github.com/rust-num/num-bigint)がRust実装ですが、浮動小数点数は見当たらないですね（たいして探して無いのであったら教えてください）。

