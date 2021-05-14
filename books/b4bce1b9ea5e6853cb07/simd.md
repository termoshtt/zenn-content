---
title: SIMD (std::arch)
---

2018/3/4のnightlyより[stdsimd](https://github.com/rust-lang-nursery/stdsimd)プロジェクトの成果として、標準ライブラリとしてSIMDを提供する機能(stable SIMD)が実装されました。この記事ではStable SIMDを用いてRustでSIMDのコードを書く方法について解説します。

- [Rust Internal: SIMD now available in libstd on nightly!](https://internals.rust-lang.org/t/simd-now-available-in-libstd-on-nightly/6903)
- [RFC 2325: Stable SIMD](https://github.com/rust-lang/rfcs/blob/master/text/2325-stable-simd.md)

(2018/8/9追記) `std::simd`が消えたことによって不要になった説明を削除・修正

前回までのあらすじ
--------
- [Rustの関数でSIMDをつかう → もっとはやい](https://qiita.com/tatsuya6502/items/7ffc623fc60be0220409)
- [rust で SIMD -- x86intrinsic を移植した話](http://mayah.jp/article/2016/x86intrin/)

上述の記事が詳しいですが簡単にまとめると

- RustでSIMDを使う方法は２つある
    - LLVMの最適化に任せる
    - intrinsicsを使う

今回のStable SIMDは両方の用法に対して大きな変更点になります

- `#[target_feature]`属性が追加され、関数単位でアーキテクチャを指定できるようになった
- ~~`std::{simd,arch}` moduleの導入により、直接intrinsicsを叩かなくて良くなった~~
  - `std::simd`は消えました(´・ω・｀)（下の追記参照）
- `std::arch`が導入されてターゲット毎のintrinsicsをstableで使えるようになった
  - 以前は`#![feature(platform_intrinsic)]`が必要でnightlyでしか使えなかった

`target_feature` attribute
---------------------------

RustはMIRと呼ばれる中間表現を経由してLLVM IRにコンパイルされますが、LLVMによる自動ベクトル化機能により、単純なfor文等をSIMDを用いて計算できます。しかしこの最適化を実行するにはSIMDの拡張命令を明示的に教える必要がありました。

```
RUSTFLAGS='-C target-feature=+avx' cargo run --release
```

これまではこの指定子 `target-feature` はコンパイル毎に指定されるため、複数のCPUで動くような実装をこのようにコンパイルすることは不可能でした。今回stable SIMDで追加された`#[target_feature]`属性はこの指定を関数単位で指定できるようにしたものです。

```rust
#[cfg(target_feature = "avx")]
fn foo() {
    // implementation that can use `avx`
}

#[cfg(not(target_feature = "avx"))]
fn foo() {
    // a fallback implementation
}
```

(RFC 2325より)このように関数の属性としてSIMDの有効・無効を指定できます。これにより関数`foo`はAVX命令を用いて最適化することが許可されるため、コンパイラに`target-feature=+avx`を渡した場合と同様のアセンブラが出力されることが期待できます。また同時に`cfg!`マクロの引数としても使えます：

```rust
if cfg!(target_feature = "avx") {
    println!("this program was compiled with AVX support");
}
```

(RFC 2325より)
加えて、実行時に検出することも出来ます：

```rust
if is_target_feature_detected!("sse4.1") {
    println!("this cpu has sse4.1 features enabled!");
}
```

初回の`is_target_feature_detected!`呼び出し時にCPUを見て判定し、以降の呼び出し時にはコストがかからないようになります。これにより1つのバイナリに複数のCPU向けの最適化を同居させ、実行時に切り替えることが可能になります。

- 判定にちょっとバグがあるっぽいです 
  - https://github.com/rust-lang-nursery/stdsimd/issues/348

`std::arch` modules
--------------------
RustはLLVMのintrinsicsを`#![feature(platform_intrinsic)]`経由で呼び出すことができるため、特定のSIMD命令を実行するためにこの機能を用いていました[^ptx]。[simd](https://rust-lang-nursery.github.io/simd/doc/simd/index.html) crateや上述の記事にある[x86intrin](https://crates.io/crates/x86intrin) crateはこれらをラップしたRust関数を提供していました。これはnightlyの機能なのでstableでは使えませんでした。

[^ptx]: [RustでCUDAカーネルを書く](https://qiita.com/termoshtt/items/b98d5c46ab9c1ab1f7b6)

今回のStable SIMDでこの点が大きく整理されました。まず2つのモジュールが`std`に追加されました：

- ~~`simd`: ポータブルなSIMD計算のための型定義 `i32x4`等~~
  - [packed_simd](https://github.com/rust-lang/rfcs/pull/2366)としてやり直すようです 
- `arch`: プラットフォーム固有な関数のラッパー `_mm_setr_epi32`等、および固有の型`__m128i`等

~~これらは `#![feature(stdsimd)]` で有効化されます。これは `#![feature(platform_intrinsics)]`とは別個の機能で、恐らく比較的すぐに安定化されると期待されます[^rust2018]。~~
Rust 1.27で`#![feature(stdsimd)]`は安定化されました。

[^rust2018]: SIMDの安定化は[Rust 2018 Roadmap](https://github.com/rust-lang/rfcs/blob/master/text/2314-roadmap-2018.md)でもCustom Allocator/Macros2.0と共に言及されています。

[Tracking issue for stable SIMD in Rust #48556](https://github.com/rust-lang/rust/issues/48556)

なお、AVX512は記事執筆段階でまだ実装中のため入っていません。

この構成を見ると、例えば`std::arch::x86_64::_mm_xor_si128`のような関数は引数として`std::simd::i32x4`のような汎用なSIMD型を取れるのかな、と期待されますが残念ながらプラットフォーム固有の`std::arch::x86_64::__m128i`を取ります。これについてはだいぶ議論があったようですが、非常に多数あるSIMD命令全てをこのように「型付きの」関数にするにはコストがかかり過ぎるという事で、引き続き`stdsimd`プロジェクト側で開発が続けられstable SIMDからは外れることになったようです。`__m128i`から`i32x4`に変換するには "either via transmutes or via explicit functions"と言っていますが、現状明示的な変換関数が見当たらないので`transmute`を使う形になりそうです。

最後に
------
ちょうどモンテカルロシミュレーション用の高速な疑似乱数生成アルゴリズムであるSIMD-oriented Fast Mersenne Twister (SFMT)をRust/stdsimdで実装していたところだったので、勢いでstable SIMDに移植しました：

https://github.com/termoshtt/rust-sfmt

Nightlyでしか動作しませんが、XOR Shiftと同等の速度で周期 $2^{19937} -1$の疑似乱数が生成できます。

追記(2018/7/7)
--------------
2018/6/21リリースのRust 1.27よりstdsimdは安定化し、`#[feature(stdsimd)]`無しでコンパイルできるようになりました

- [Announcing Rust 1.27](https://blog.rust-lang.org/2018/06/21/Rust-1.27.html)

追記(2018/8/9)
--------------
1.27での`std::simd`の安定化が無かったことになりました

- [Update stdsimd to undo an accidental stabilization #52535](https://github.com/rust-lang/rust/pull/52535)

`std::simd`として安定化された部分は`packed_simd`として再度検討されるようです

- [RFC 2366: Portable packed SIMD vector types](https://github.com/rust-lang/rfcs/pull/2366)

`std::simd`互換のインタフェースを提供する目的で[packed_simd](https://github.com/rust-lang-nursery/packed_simd) crateが出来ており、例えば[rand](https://github.com/rust-lang-nursery/rand/pull/569)や[servo](https://github.com/servo/servo/pull/21272)もそちらを使っているようです。このRFCの安定化まではpacked_simdはnightlyでしか動きません。

ややこしいですが、stdsimd全体が無かったことになったわけでは無く、[std::arch](https://doc.rust-lang.org/beta/std/arch/)は安定化されているのでターゲット固定のSIMD機能はstableで使用できます。

余談ですが、Embedded-WGではRustでインラインアセンブラをstableで使うために`asm!`機能を安定化させるのでなく、`core::arch`以下に実装していく方針のようです

- [Stable assembly operations](https://github.com/rust-embedded/wg/issues/63)

