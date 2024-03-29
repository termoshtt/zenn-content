---
title: "多次元配列と写像の表示"
emoji: "📦"
type: "idea"
topics: ["math"]
published: true
---

多次元配列というのは単に複数の添字の組を引数にとる配列なのですが、どうしてこれが数値計算のソフトウェアにおいて重要になるのかをみるために数学的な導入を試みます。

配列と写像の表示
-----------------
まず多次元でなく1次元の配列を考えます。例えばRustには可変長の`Vec<T>`と固定長の`[T; n]`が存在しますが、ここでは後者を考えましょう。これはただ`n`個`T`型のデータが並んだだけのデータです。これの数学的な意味とはなんでしょうか？　素朴には$n$個の$T$の直積$$T^n = T \times \cdots \times T$$と考えることも出来ますが、ここでは少し違った見方として添字によるアクセスに注目します：

```rust
let a: [f64; 3] = [1.0, 2.0, 3.0];
println!("{}", a[2]); // 3.0
```

のように添字`2`を指定すると浮動小数点数`3.0`が取得できます。これは3つの要素からなる集合$\{ 0, 1, 2 \}$に対して浮動小数点数(しばらく実数と同一視しましょう)を返す写像に見えます。順序数を$[n] = \{ 0, 1, \ldots, n-1 \}$と書くことにすると、配列`a: [T; n]`は写像 $$a: [n] \to T$$と見ることが出来ます。逆に写像$a: [n] \to T$はそれぞれの整数についての値を取る事により配列を1つ定めるのでこの対応は1対1になります。この対応は集合$A$から集合$B$への写像全体を$\text{Hom}(A, B)$と書くとき、集合としての同型$$\text{Hom}([n], T) \simeq T^n$$が定まる事を意味します。

任意の有限集合$I$にはいつでも要素に番号をつけることで順序数へ、要素からその番号を取得する・逆に番号から要素を指定する全単射$I \simeq [\#I]$が作れるので($\#I$は元の個数)、有限集合を始域に持つ写像$f: I \to T$は常に配列で表示出来ます：$$\text{Hom}(I, T) \simeq \text{Hom}([\#I], T) \simeq T^{\#I}$$

このように写像として記述しておくと配列に対していくつか自然に操作が入ることが分かります。例えば終域$T$に足し算が入っているとすると、2つの写像$f, g: [n] \to T$に対して$$(f + g)(i) = f(i) + g(i)$$で写像自体に和$f+g: [n] \to T$を考えることが出来ますが、これはちょうど配列の各要素に対して和を取っていることに対応します。
```rust
let f: [f64; 3] = [1.0, 2.0, 3.0];
let g: [f64; 3] = [4.0, 5.0, 6.0];

let mut f_g = <[f64; 3]>::default();
for i in 0..3 {
  f_g[i] = f[i] + g[i];
}
```
この視点は特に多次元配列におけるいくつかの定義、特にブロードキャストを理解する助けになります。

多次元配列
----------
この議論は写像の始域$I$が有限な限り適用できますが、いつでも自然に一列に並べられるとは限りません。例えば画像データを考えましょう。これはカメラのピクセルの配置、つまり左から`i`番目で上から`j`番目のピクセルに対してRGBの色を1つ対応させたものと思うことが出来ます。つまりこれも1つの有限集合上の写像$$\text{picture}: [\text{height}] \times [\text{width}] \to \text{Color}$$と考えることが出来ます。ピクセルの個数は有限個なので適当に番号をつけることにより1次元の配列に収めることも出来ますが、せっかく意味ある2次元の構造を捨ててしまうのはもったいないです。そこでこの構造を残したまま写像を表示できる構造体を計算機の方に作ることにしましょう。

$n$個の有限集合の積から任意の型$T$への写像$$a: I_1 \times \cdots \times I_n \to T$$を配列と写像の対応との類推から多次元配列と呼びましょう。個々の有限集合$I_k$の元に番号をつける事でそれぞれを順序数に対応させることによりやはり配列と対応させることが出来ます$$\text{Hom}(I_1 \times \cdots \times I_n, T) \simeq \text{Hom}([\#I_1] \times \cdots \times [\#I_n], T) \simeq T^{(\#I_1 \times \cdots \times \#I_n)}$$これを多次元配列と呼ぶことにしましょう。

写像として作ることで1次元の配列で見たように終域に演算が入っている場合は自動的に配列に対しても要素毎の演算が入ります。つまり終域$T$に足し算が入っているとすると、2つの写像$f, g: I_1 \times I_2 \to T$に対して$$(f + g)(i, j) = f(i, j) + g(i, j)$$で和の写像$f+g: I_1 \times I_2 \to T$を考えることが出来ます。

このような複数の有限集合の直積の上の写像は、ちょうど画像の例が長方形区間(つまり$[0, \text{width}] \times [0, \text{height}]$の形に分解できる空間)の上の色空間をモデルしているように、元々モデルしようとしている空間が直積に分解できる時に自然に現れます。長方形の縦横ように上手く分解できない、例えばグラフのような構造の上の写像を扱う際には、それ用のデータ構造を用意することになります。その場合にもデータ構造を写像の表現とみなすことで、例えば終域の演算から自動的に要素毎の演算を用意したりする操作を考えることが出来ます。

