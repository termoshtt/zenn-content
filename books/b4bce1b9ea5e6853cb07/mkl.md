---
title: Intel MKLを使う (intel-mkl-src crate)
---

[Intel Math Kernel Library (MKL)](https://software.intel.com/en-us/mkl)は代表的な最適化済みの数学ライブラリで、[Intel Simplified Software License](https://software.intel.com/en-us/license/intel-simplified-software-license)に基づいて再配布が認められています。
これをRustからcrateとして使えるようにしたものが[intel-mkl-src](https://github.com/rust-math/intel-mkl-src)です

https://github.com/rust-math/intel-mkl-src

~~※現在はWindowsは対応していません~~ 0.4.0からWindowsも対応しました

使い方
------
このcrateは`*-src` crateで、MKLをダウンロードしてリンクするだけです。BLASやLAPACK, FFTの機能を使うにはそれぞれ[blas-sys](https://github.com/blas-lapack-rs/blas-sys), [lapack-sys](https://github.com/blas-lapack-rs/lapack-sys), [fftw-sys](https://github.com/rust-math/rust-fftw3/tree/master/fftw-sys)を使います。

```toml
[dependencies]
fftw-sys = { version = "0.4", features = ["intel-mkl"] }
```

このようにそれぞれのcrateでfeatureを使ってバックエンドが切り替えれるようになっているのでその機能を使います。あるいはより高次のライブラリである[ndarray-linalg](https://github.com/termoshtt/ndarray-linalg)を使います。

```toml
[dependencies]
ndarray-linalg = { version = "*", features = ["intel-mkl"] }
```

ndarray-linalg自体の使い方はこちら：

- [Rustで線形代数 (ndarray-linalgの使い方)](https://qiita.com/termoshtt/items/824684a37a2ec15dbfb9)

rust-math
---------
科学技術計算系のcrateのうち、ユーザーが居る気がするものを[rust-math](https://github.com/rust-math)に分けました。intel-mkl-srcのほかに二つ移行してあります。

- [rust-fftw3](https://github.com/rust-math/rust-fftw3): FFTW3 binding for Rust 
- [rust-sfmt](https://github.com/rust-math/rust-sfmt): Rust implementation of  SIMD-oriented Fast Mersenne Twister (SFMT) using stable SIMD

メンテが滞りがちなので、興味のある方は連絡ください。

