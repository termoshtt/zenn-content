---
title: SIMDを使う
---

この記事ではRustでSIMDのコードを書く方法について解説します。大まかに言って、RustでSIMDを使う方法は二通りあります：

- LLVMの最適化に任せる
- アーキテクチャ毎に定義されるintrinsicsを使う

LLVMに最適化させる： `target_feature` attribute
-------------------------------------------------

RustはMIRと呼ばれる中間表現を経由してLLVM IRにコンパイルされますが、[LLVMによる自動ベクトル化機能](https://llvm.org/docs/Vectorizers.html)により、単純なfor文等をSIMDを用いて計算できます。しかしこの最適化を実行するにはSIMDの拡張命令を明示的に教える必要がありました。

```shell
RUSTFLAGS='-C target-feature=+avx' cargo run --release
```

CPUに搭載されているSIMD演算器はCPUのバージョン毎で異なります。コンパイルされたバイナリが実際に動作するCPUにおいてサポートされていない命令を含む場合、実行されたプロセスはIllegal Instruction errorで終了します。上の`RUSTFLAGS`ではAVX命令を有効にするため、AVX命令実装されていないとても古いCPUでは動作しません。これは上の様に`cargo build`全体で有効にすることも出来ますが、`#[target_feature]`属性を使うと関数毎に適用できます：

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

([RFC 2325][RFC2325]より)このように関数の属性としてSIMDの有効・無効を指定できます。これにより関数`foo`はAVX命令を用いて最適化することが許可されるため、コンパイラに`target-feature=+avx`を渡した場合と同様のアセンブラが出力されることを期待できます。また同時に`cfg!`マクロの引数としても使えます：

```rust
if cfg!(target_feature = "avx") {
    println!("this program was compiled with AVX support");
}
```

([RFC 2325][RFC2325]より)加えて、実行時に検出することも出来ます：

```rust
if is_x86_feature_detected!("sse4.1") {
    println!("this cpu has sse4.1 features enabled!");
}
```

初回の`is_target_feature_detected!`呼び出し時にCPUを見て判定し、以降の呼び出し時にはコストがかからないようになります。これにより1つのバイナリに複数のCPU向けの最適化を同居させ、実行時に切り替えることが可能になります。

[RFC2325]: https://rust-lang.github.io/rfcs/2325-stable-simd.html

`core::arch` modules
--------------------

Nightly RustはLLVMのintrinsicsを`#![feature(platform_intrinsic)]`経由で呼び出すことができ、CPUのSIMD命令はLLVMのintrinsicsとして登録されているため、特定のSIMD命令を実行するためにこの機能を用いていました。多くのプラットフォーム上でこの機能はNightlyが必要ですが、[x86][x86]と[x86_64][x86_64]、及び[wasm32][wasm32]の一部についてはStable Rustで使うことが出来ます。ただしSIMD命令はAVX2までで、AVX512命令はNightlyのままです。

[x86]: https://doc.rust-lang.org/core/arch/x86/index.html
[x86_64]: https://doc.rust-lang.org/core/arch/x86_64/index.html
[wasm32]: https://doc.rust-lang.org/core/arch/wasm32/index.html
[core::arch]: https://doc.rust-lang.org/core/arch

intrinsicsの呼び出しは全てunsafeになります。例えばSSE4.1の命令を使ったコードは次のようになります（[core::arch][core::arch]より）：

```rust
// translated from
// <https://github.com/Matherunner/bin2hex-sse/blob/master/base16_sse4.cpp>
#[target_feature(enable = "sse4.1")]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn hex_encode_sse41(mut src: &[u8], dst: &mut [u8]) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let ascii_zero = _mm_set1_epi8(b'0' as i8);
    let nines = _mm_set1_epi8(9);
    let ascii_a = _mm_set1_epi8((b'a' - 9 - 1) as i8);
    let and4bits = _mm_set1_epi8(0xf);

    let mut i = 0_isize;
    while src.len() >= 16 {
        let invec = _mm_loadu_si128(src.as_ptr() as *const _);

        let masked1 = _mm_and_si128(invec, and4bits);
        let masked2 = _mm_and_si128(_mm_srli_epi64(invec, 4), and4bits);

        // return 0xff corresponding to the elements > 9, or 0x00 otherwise
        let cmpmask1 = _mm_cmpgt_epi8(masked1, nines);
        let cmpmask2 = _mm_cmpgt_epi8(masked2, nines);

        // add '0' or the offset depending on the masks
        let masked1 = _mm_add_epi8(
            masked1,
            _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask1),
        );
        let masked2 = _mm_add_epi8(
            masked2,
            _mm_blendv_epi8(ascii_zero, ascii_a, cmpmask2),
        );

        // interleave masked1 and masked2 bytes
        let res1 = _mm_unpacklo_epi8(masked2, masked1);
        let res2 = _mm_unpackhi_epi8(masked2, masked1);

        _mm_storeu_si128(dst.as_mut_ptr().offset(i * 2) as *mut _, res1);
        _mm_storeu_si128(
            dst.as_mut_ptr().offset(i * 2 + 16) as *mut _,
            res2,
        );
        src = &src[16..];
        i += 16;
    }

    let i = i as usize;
    hex_encode_fallback(src, &mut dst[i * 2..]);
}

fn hex_encode_fallback(src: &[u8], dst: &mut [u8]) {
    fn hex(byte: u8) -> u8 {
        static TABLE: &[u8] = b"0123456789abcdef";
        TABLE[byte as usize]
    }

    for (byte, slots) in src.iter().zip(dst.chunks_mut(2)) {
        slots[0] = hex((*byte >> 4) & 0xf);
        slots[1] = hex(*byte & 0xf);
    }
}
```

stdsimd
--------

過去に一時的に導入されていた`core::simd`は現在(2021/5)[rust-lang/stdsimd](https://github.com/rust-lang/stdsimd)として開発されています。これはPortable SIMDを標榜するプロジェクトで、アーキテクチャに依存しない[f64x4](https://rust-lang.github.io/stdsimd/core_simd/type.f64x4.html)の様な汎用な整数・浮動小数点数型やそれへの演算を定義しています。

参考リンク
-----------
- [Rust Internal: SIMD now available in libstd on nightly!](https://internals.rust-lang.org/t/simd-now-available-in-libstd-on-nightly/6903)
- [RFC 2325: Stable SIMD](https://github.com/rust-lang/rfcs/blob/master/text/2325-stable-simd.md)
- [Rustの関数でSIMDをつかう → もっとはやい](https://qiita.com/tatsuya6502/items/7ffc623fc60be0220409)
- [rust で SIMD -- x86intrinsic を移植した話](http://mayah.jp/article/2016/x86intrin/)
