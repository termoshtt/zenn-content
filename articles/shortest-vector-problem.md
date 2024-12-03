---
title: "格子の最短ベクトル問題とLenstraのアルゴリズム"
emoji: "🪟"
type: "tech"
topics: ["数理最適化", "math"]
published: true
---

この記事は[数理最適化 Advent Calendar 2024](https://qiita.com/advent-calendar/2024/mathematical-optimization)3日目の記事です。

--------


この記事では[代数的・幾何的アプローチによる離散最適化入門](https://www.kyoritsu-pub.co.jp/book/b10031338.html)の第２章に基づいて、最短ベクトル問題(shortest vector problem, SVP)とLenstra-Lenstra-Lovász (LLL)アルゴリズム、及びそれを整数計画法に応用したLenstraのアルゴリズムについて紹介します。
とはいえこの本はLenstraのアルゴリズムの部分はほとんど書いていないため、詳しくは [50 Years of Integer Programming 1958-2008 Chapter 14 Integer Programming and Algorithmic Geometry of Numbers](https://link.springer.com/chapter/10.1007/978-3-540-68279-0_14)を読む必要があると思われます。

LLLアルゴリズムは元々多項式を既約多項式の積に因数分解するために考案されたらしいですが、現在では格子暗号の分野で広く普及しているためその文脈での解説が多いです。なのでこの記事ではLLLアルゴリズムについては深入りせずに整数計画法への応用に焦点を当てます。

:::message
この記事は筆者がこれからこれらの文献を真面目に読むのでそのための予備調査の結果をまとめたものです。個々の定理の証明を理解することを目的とせず、最終的に何が示されるのか、その仮定でどのような補題が使われているのかの概略をまとめることを目的としています。
この記事を書いている段階で筆者はこれらの証明を理解できていないため誤りが含まれる可能性が高いです。間違いは指摘していただけると助かります。
:::

# Lenstraのアルゴリズム

まずLenstraのアルゴリズムについての紹介から始めましょう。これは整数線形充足問題(ILP)、つまり $m \times n$の整数値行列 $A \in \mathbb{Z}^{m \times n}$ と整数値ベクトル $b \in \mathbb{Z}^m$ が与えられたとき、$Ax \leq b$ を満たす整数値ベクトル $x \in \mathbb{Z}^n$ を一つ求めるか、その解が存在しないことを判定する問題です。

Lenstraのアルゴリズムでは次元 $n$ を固定したとき、他の入力に対して多項式時間で解を求めることができるとされています。このアルゴリズムの鍵となるのがKhinchineの平坦性定理です。

## Khinchineの平坦性定理

まず準備として方向付きの厚みを定義しましょう。$K \subset \mathbb{R}^n$ が空で無い凸閉集合とします。$d \in \mathbb{R}^n$ に対する $K$ の厚みを次のように定義します。

$$
w_d(K) = \max \{ \langle d, x \rangle \mid x \in K \} - \min \{ \langle d, x \rangle \mid x \in K \}
$$

ここで $\langle \cdot, \cdot \rangle$ は通常の内積です。$\max$ か $\min$ が存在しないときは $w_d(K) = \infty$ とします。特に方向 $d$ が整数値の場合の厚さの最小値

$$
w(K) = \min_{d \in \mathbb{Z}^n \setminus \{0\}} w_d(K)
$$

が重要となります。この $\min$ を達成する $d$ の事を $K$ の flat direction と呼びます。

:::message
（Khinchineの平坦性定理）

$K \subset \mathbb{R}^n$ が空で無い凸閉集合とする。この時 $K$ は整数点を持つか、$w(K) \leq \Omega(n)$ が成り立つ。ただし $\Omega(n)$ は次元だけに依存する定数。
:::

この定理により凸集合の幅という幾何学的な量が整数点の存在に関連していることが分かります。

## Lenstraのアルゴリズム

ILPを線形緩和した実行可能領域 $P = \{ x \in \mathbb{R}^n \mid Ax \leq b \}$ が有界でかつ空で無い場合を考えましょう。これは凸閉なので flat direction $d$ を求められたとすると $w_d(P) \leq \Omega(n)$ であれば平坦性定理より整数点を持つことが分かりILPの解が存在することが分かります。

$d$をどうやって計算するかは後回しにして、$w_d(P) > \Omega(n)$の場合を考えましょう。$d \in \mathbb{Z}^n$なのでもし整数点 $x \in P \cap \mathbb{Z}^n$ が存在するならば $\langle d, x \rangle$ は整数値を取ります。今 $P$ は有界なので $\delta \in \mathbb{Z}$ で $\min_{x \in P} \langle d, x \rangle \leq \delta \leq \max_{x \in P} \langle d, x \rangle$ を満たすものは有限個しかありません。つまり $n$ 次元のILPを有限個の $\langle d, x \rangle = \delta$ による定まる超平面上の $n-1$ 次元のILPに帰着できるということです。
連続の場合と違って超平面上に制限したときにILPになるかは自明では無いですが、これはエルミート標準形に変形することで確認できます。

この $d$ は $P$ が一番薄い方向なのでそれだけ分岐の個数が少なくなるという性質があるはずですが、ここまでの議論ではこれが十分に少なくなるかは分かりません。これは線形計画法に対する楕円体法に似た方法で示すことが出来るとあります。

以上がLenstraのアルゴリズムの概略です。$d$ を求める問題は最短ベクトル問題として見なすことができるのでLLLアルゴリズムを使って解くことができるというのがオリジナルのLenstraの仕事で、後の研究で多くの改善がなされて現在ではLenstra型のアルゴリズムと呼ばれているようです。

# 参考文献

最短ベクトル問題及びLLLアルゴリズムは暗号の分野で重要なので、その文脈での資料が多く存在します。

## LLLアルゴリズムの解説

- [Lenstra–Lenstra–Lovász lattice basis reduction algorithm -- Wikipedia](https://en.wikipedia.org/wiki/Lenstra%E2%80%93Lenstra%E2%80%93Lov%C3%A1sz_lattice_basis_reduction_algorithm)
  - LLLアルゴリズムはMathematicaやSageMathなどの数式処理システムに実装されている
- [An Introduction to Lenstra-Lenstra-Lovasz Lattice Basis Reduction Algorithm (Xinyue Deng, MIT)](https://math.mit.edu/~apost/courses/18.204-2016/18.204_Xinyue_Deng_final_paper.pdf)
- [LLLを理解するぞ](https://mitsu1119.github.io/blog/p/lll%E3%82%92%E7%90%86%E8%A7%A3%E3%81%99%E3%82%8B%E3%81%9E/)

## 数理最適化への応用

https://www.kyoritsu-pub.co.jp/book/b10031338.html

https://link.springer.com/chapter/10.1007/978-3-540-68279-0_14

## 暗号への応用

- [格子理論を用いた暗号解読の最近の研究動向（國廣 昇）](https://www.jstage.jst.go.jp/article/essfr/5/1/5_1_42/_pdf/-char/ja)
- [格子暗号 （廣政 良, 三菱電機, 耐量子計算機暗号と量子情報の数理）](https://joint.imi.kyushu-u.ac.jp/wp-content/uploads/2022/08/220801_03hiromasa.pdf)
- [CTFにおけるLLLの使い方を現役エンジニアが解説](https://qiita.com/kusano_k/items/5509bff6e426e5043591)