---
title: "格子の最短ベクトル問題とLenstraのアルゴリズム"
emoji: "♻"
type: "tech"
topics: ["math", "algorithm"]
published: true
---

この記事では[代数的・幾何的アプローチによる離散最適化入門](https://www.kyoritsu-pub.co.jp/book/b10031338.html)の第２章に基づいて、最短ベクトル問題(shortest vector problem, SVP)とLenstra-Lenstra-Lovász (LLL)アルゴリズムを議論し、それを整数計画法に応用したLenstraのアルゴリズムについて解説します。

# 格子の最短ベクトル問題

# LLLアルゴリズム

# 整数計画法への応用

# 参考文献

最短ベクトル問題及びLLLアルゴリズムは暗号の分野で重要なので、その文脈での資料が多く存在します。

## LLLアルゴリズムの解説

- [Lenstra–Lenstra–Lovász lattice basis reduction algorithm -- Wikipedia](https://en.wikipedia.org/wiki/Lenstra%E2%80%93Lenstra%E2%80%93Lov%C3%A1sz_lattice_basis_reduction_algorithm)
  - LLLアルゴリズムはMathematicaやSageMathなどの数式処理システムに実装されている
- [An Introduction to Lenstra-Lenstra-Lovasz Lattice Basis Reduction Algorithm (Xinyue Deng, MIT)](https://math.mit.edu/~apost/courses/18.204-2016/18.204_Xinyue_Deng_final_paper.pdf)
- [LLLを理解するぞ](https://mitsu1119.github.io/blog/p/lll%E3%82%92%E7%90%86%E8%A7%A3%E3%81%99%E3%82%8B%E3%81%9E/)

## 数理最適化への応用

https://www.kyoritsu-pub.co.jp/book/b10031338.html

## 暗号への応用

- [格子理論を用いた暗号解読の最近の研究動向（國廣 昇）](https://www.jstage.jst.go.jp/article/essfr/5/1/5_1_42/_pdf/-char/ja)
- [格子暗号 （廣政 良, 三菱電機, 耐量子計算機暗号と量子情報の数理）](https://joint.imi.kyushu-u.ac.jp/wp-content/uploads/2022/08/220801_03hiromasa.pdf)
- [CTFにおけるLLLの使い方を現役エンジニアが解説](https://qiita.com/kusano_k/items/5509bff6e426e5043591)