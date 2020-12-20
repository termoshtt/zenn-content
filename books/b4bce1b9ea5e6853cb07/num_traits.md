---
title: 数値を扱うTrait (num-traits crate)
---

Rustで数値型を扱う際に有用なTraitについてまとめていきます。

[rust-num organization](https://github.com/rust-num) 以下に数値関係のcrateがまとまっています。`num` crateは全体をまとめて再エクスポートしたもので、実体としては [num-traits](https://github.com/rust-num/num-traits) や [num-complex](https://github.com/rust-num/num-complex) 等のそれぞれのcrateで開発されています。

### 前提知識１: `std::ops::Add`

Rustでは`+`等の演算子のオーバーロードもTraitとして実装されます

```rust
use std::ops::Add;
fn calc<T: Add>(a: T, b: T) -> T {
  a + b
}
```

このように`T`型に`Add`を要求しておくと`+`演算子で足し算が出来るようになります。

```rust
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

`Add` Trait はこのように足される `Rhs` と足された結果 `Output` をそれぞれ型パラメータと関連型として持っています。`Rhs`毎に`Add<Rhs>` Traitが定義され、このTraitに対して`Output`型が一つ決まります。

### 前提知識２: Traitの継承

Traitの定義時に他のTraitを継承できる。

```rust
trait A {
  fn fa(&self);
}
trait B: A {
  fn fb(&self);
}

fn f<T: B>(a: &T) {
  a.fa();  // Aの関数も使える
  a.fb();
} 
```

この準備の下で`num_traits` crateを見ていきます。

num_traits::Num
----------------

`num_traits::NumOps`演算子だけ定義されいてるTraitです

```rust
pub trait NumOps<Rhs = Self, Output = Self>:
   Add<Rhs, Output = Output>
 + Sub<Rhs, Output = Output>
 + Mul<Rhs, Output = Output>
 + Div<Rhs, Output = Output>
 + Rem<Rhs, Output = Output>
{}
```

これで足し算・掛け算他が入ったので、単位元を定義できます：

```rust
pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn set_zero(&mut self) { ... }
}
pub trait One: Sized + Mul<Self, Output = Self> {
    fn one() -> Self;
    fn is_one(&self) -> bool where Self: PartialEq { ... }
    fn set_one(&mut self) { ... }
}
```

これらに加えてさらに`PartialEq`を追加したものが`Num`です。

```rust
pub trait Num: PartialEq + Zero + One + NumOps {
  // 略
}
```

演算子には`+=`のように自己に代入するものもあり`num_traits::NumOpsAssign`が対応します。こちらは出力が`Self`で固定になるので`Output`はパラメータに存在しません。

```rust
pub trait NumAssignOps<Rhs = Self>:
   AddAssign<Rhs>
 + SubAssign<Rhs>
 + MulAssign<Rhs>
 + DivAssign<Rhs>
 + RemAssign<Rhs>
{}
```

これを追加したのが `NumAssign` です

```rust
pub trait NumAssign: Num + NumAssignOps {}
```

これらのTraitは全て `usize`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`等の標準の数値型に対して実装されています。

num_traits::NumCast
---------------------
プリミティブ型に変換するためのTraitが`ToPrimitive`です。

```rust
pub trait ToPrimitive {
    fn to_i64(&self) -> Option<i64>;
    fn to_u64(&self) -> Option<u64>;
    fn to_isize(&self) -> Option<isize> { ... }
    fn to_i8(&self) -> Option<i8> { ... }
    fn to_i16(&self) -> Option<i16> { ... }
    fn to_i32(&self) -> Option<i32> { ... }
    fn to_i128(&self) -> Option<i128> { ... }
    fn to_usize(&self) -> Option<usize> { ... }
    fn to_u8(&self) -> Option<u8> { ... }
    fn to_u16(&self) -> Option<u16> { ... }
    fn to_u32(&self) -> Option<u32> { ... }
    fn to_u128(&self) -> Option<u128> { ... }
    fn to_f32(&self) -> Option<f32> { ... }
    fn to_f64(&self) -> Option<f64> { ... }
}
```

`Option`になっているのは、値がその型で表現できない場合があるからです。逆向きが`FromPrimitive`です。

```rust
pub trait FromPrimitive: Sized {
    fn from_i64(n: i64) -> Option<Self>;
    fn from_u64(n: u64) -> Option<Self>;
    fn from_isize(n: isize) -> Option<Self> { ... }
    fn from_i8(n: i8) -> Option<Self> { ... }
    fn from_i16(n: i16) -> Option<Self> { ... }
    fn from_i32(n: i32) -> Option<Self> { ... }
    fn from_i128(n: i128) -> Option<Self> { ... }
    fn from_usize(n: usize) -> Option<Self> { ... }
    fn from_u8(n: u8) -> Option<Self> { ... }
    fn from_u16(n: u16) -> Option<Self> { ... }
    fn from_u32(n: u32) -> Option<Self> { ... }
    fn from_u128(n: u128) -> Option<Self> { ... }
    fn from_f32(n: f32) -> Option<Self> { ... }
    fn from_f64(n: f64) -> Option<Self> { ... }
}
```

これを使って統一的に扱えるようにしたのが`NumCast`です

```rust
pub trait NumCast: Sized + ToPrimitive {
    fn from<T: ToPrimitive>(n: T) -> Option<Self>;
}
```

num_traits::PrimInt
---------------------
整数型の性質を抽象化したTraitです。ビット演算が多く実装されています。

```rust
pub trait PrimInt:
   Sized + Copy + Num + NumCast + Bounded + PartialOrd + Ord + Eq + Not<Output = Self>
 + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> 
 + Shl<usize, Output = Self> + Shr<usize, Output = Self>
 + CheckedAdd<Output = Self> + CheckedSub<Output = Self> + CheckedMul<Output = Self> + CheckedDiv<Output = Self>
 + Saturating
{
    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
    fn leading_zeros(self) -> u32;
    fn trailing_zeros(self) -> u32;
    fn rotate_left(self, n: u32) -> Self;
    fn rotate_right(self, n: u32) -> Self;
    fn signed_shl(self, n: u32) -> Self;
    fn signed_shr(self, n: u32) -> Self;
    fn unsigned_shl(self, n: u32) -> Self;
    fn unsigned_shr(self, n: u32) -> Self;
    fn swap_bytes(self) -> Self;
    fn from_be(x: Self) -> Self;
    fn from_le(x: Self) -> Self;
    fn to_be(self) -> Self;
    fn to_le(self) -> Self;
    fn pow(self, exp: u32) -> Self;
}
```

[num_traits::Float][Float]
-----------------

[Float]: https://docs.rs/num-traits/0.2.14/num_traits/float/trait.Float.html

続いて浮動小数点に特化したTraitです。標準的な数学関数が定義されています。

```rust
pub trait Float: Num + Copy + NumCast + PartialOrd + Neg<Output = Self> {
    fn nan() -> Self;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn neg_zero() -> Self;
    fn min_value() -> Self;
    fn min_positive_value() -> Self;
    fn max_value() -> Self;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_normal(self) -> bool;
    fn classify(self) -> FpCategory;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn recip(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn abs_sub(self, other: Self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
    fn integer_decode(self) -> (u64, i16, i8);
    fn epsilon() -> Self { ... }
    fn to_degrees(self) -> Self { ... }
    fn to_radians(self) -> Self { ... }
}
```

[num_complex::Complex][Complex]
------------

複素数は構造体 [num_complex::Complex<T>][Complex] で定義されており、関連関数として数学関数が定義されています。0.3.0 でいくつか破壊的な変更が入っているので注意です。

[Complex]: https://docs.rs/num-complex/0.3.1/num_complex/struct.Complex.html

```toml
[dependencies]
num_complex = "0.3.1"
```

```rust
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct Complex<T> {
    /// Real portion of the complex number
    pub re: T,
    /// Imaginary portion of the complex number
    pub im: T,
}
pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;
```

これは `repr(C)` で定義されいているので C99 の `_Complex` 型、C++の `std::complex<T>` と同じメモリ配置になります。

これを複素数と実数型を合わせて同じように使えるようにした Trait が `cauchy::Scalar` です。

```rust
pub trait Scalar:
    NumAssign + FromPrimitive + NumCast + Neg<Output = Self>
  + Copy + Clone
  + Display + Debug + LowerExp + UpperExp
  + Sum + Product + Serialize + for<'de> Deserialize<'de> + 'static
{
    type Real: Scalar<Real = Self::Real, Complex = Self::Complex> + NumOps<Self::Real, Self::Real> + Float;
    type Complex: Scalar<Real = Self::Real, Complex = Self::Complex> + NumOps<Self::Real, Self::Complex> + NumOps<Self::Complex, Self::Complex>;
    fn real<T: ToPrimitive>(re: T) -> Self::Real;
    fn complex<T: ToPrimitive>(re: T, im: T) -> Self::Complex;
    fn from_real(re: Self::Real) -> Self;
    fn add_real(self, re: Self::Real) -> Self;
    fn sub_real(self, re: Self::Real) -> Self;
    fn mul_real(self, re: Self::Real) -> Self;
    fn div_real(self, re: Self::Real) -> Self;
    fn add_complex(self, im: Self::Complex) -> Self::Complex;
    fn sub_complex(self, im: Self::Complex) -> Self::Complex;
    fn mul_complex(self, im: Self::Complex) -> Self::Complex;
    fn div_complex(self, im: Self::Complex) -> Self::Complex;
    fn pow(&self, n: Self) -> Self;
    fn powi(&self, n: i32) -> Self;
    fn powf(&self, n: Self::Real) -> Self;
    fn powc(&self, n: Self::Complex) -> Self::Complex;
    fn re(&self) -> Self::Real;
    fn im(&self) -> Self::Real;
    fn as_c(&self) -> Self::Complex;
    fn conj(&self) -> Self;
    fn abs(&self) -> Self::Real;
    fn square(&self) -> Self::Real;
    fn sqrt(&self) -> Self;
    fn exp(&self) -> Self;
    fn ln(&self) -> Self;
    fn sin(&self) -> Self;
    fn cos(&self) -> Self;
    fn tan(&self) -> Self;
    fn asin(&self) -> Self;
    fn acos(&self) -> Self;
    fn atan(&self) -> Self;
    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn tanh(&self) -> Self;
    fn asinh(&self) -> Self;
    fn acosh(&self) -> Self;
    fn atanh(&self) -> Self;
    fn rand(rng: &mut impl Rng) -> Self;
}
```

Links
-------
- num-traits
    - https://github.com/rust-num/num-traits
    - https://docs.rs/num-traits/0.2.8/num_traits
- num-complex
    - https://github.com/rust-num/num-complex
    - https://docs.rs/num-complex/0.2.3/num_complex
- cauchy
    - https://github.com/rust-math/cauchy
    - https://docs.rs/cauchy/0.2.2/cauchy
