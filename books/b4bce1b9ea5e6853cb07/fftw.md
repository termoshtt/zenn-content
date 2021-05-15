---
title: 高速Fourier変換 (fftw crate)
---

https://github.com/rust-math/fftw

- fftw crate: Rust向けのAPI
- fftw-sys crate: FFTWへのFFI (ほぼ自動生成)
- fftw-src crate: FFTWをコンパイルしてリンクする為のcrate, intel-mkl-srcに置き換え可能

のように３つのcrateに分かれています。

```toml
[dependencies]
fftw = "0.6.2"
```

とするとデフォルトで `fftw-src` がつかわれFFTWをビルドしてリンクします。Intel MKLを使うには次の様に `features` を指定します：

```toml
[dependencies]
fftw = { version = "0.6.2", features = ["intel-mkl"] }
```

他にも `features = ["system"]` とするとシステムに既に存在するFFTWのバイナリを探してリンクしようとします。

fftw crate
-----------

元のFFTWのインタフェース(C API)にしたがって、一旦Planと呼ばれる構造体を生成します。これにはFFTを効率良く計算するために前もって計算したデータを持つための構造体で、これを保持することにより実行を高速に出来ます。特に同じサイズの配列を複数回FFTする際に有効です。またFFTWはSIMD演算を使って高速化するためにメモリのアライメントに対して制約を課します。これをRust側で使えるように [AlignedVec](https://docs.rs/fftw/0.6.2/fftw/array/struct.AlignedVec.html) が用意してあります：

- Complex to Complex

```rust
use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use std::f64::consts::PI;

let n = 128;
let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
let mut a = AlignedVec::new(n);
let mut b = AlignedVec::new(n);
let k0 = 2.0 * PI / n as f64;
for i in 0..n {
    a[i] = c64::new((k0 * i as f64).cos(), 0.0);
}
plan.c2c(&mut a, &mut b).unwrap();
```

- Complex to Real

```rust
use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use std::f64::consts::PI;

let n = 128;
let mut c2r: C2RPlan64 = C2RPlan::aligned(&[n], Flag::MEASURE).unwrap();
let mut a = AlignedVec::new(n / 2 + 1);
let mut b = AlignedVec::new(n);
for i in 0..(n / 2 + 1) {
    a[i] = c64::new(1.0, 0.0);
}
c2r.c2r(&mut a, &mut b).unwrap();
```
