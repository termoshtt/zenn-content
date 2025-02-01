---
title: "ブール代数"
---

単にSATソルバーと言った場合、命題論理の充足可能性問題を解くプログラムの事を指しますが、論理学を扱うのは今回の目的とは少し異なるので、この本では命題論理の数理構造に相当するブール代数だけを使って議論します。なので本文中で出てくる「命題」や「論理式」は論理学の用語ではなく、ブール代数を用いて表現された数学的な概念を指します。

ブール代数
----------

まずブール代数を定義しましょう。
集合 $L$ とその上の２項演算 $\land$ と $\lor$ が任意の $x, y, z \in L$ に対して次の条件を満たす時、$\langle L, \land, \lor \rangle$ の組を分配束と呼びます。

- 交換則： $x \land y = y \land x$, $x \lor y = y \lor x$
- 結合則：$(x \land y) \land z = x \land (y \land z)$, $(x \lor y) \lor z = x \lor (y \lor z)$
- 吸収則：$(x \land y) \lor x = x$, $(x \lor y) \land x = x$
- 分配則：$(x \lor y) \land z = (x \land z) \lor (y \land z)$, $(x \land y) \lor z = (x \lor z) \land (y \lor z)$

さらに $L$ に特殊な元 $0, 1$ が存在し、単項演算 $\lnot$ が任意の $x \in L$ に対して次の条件を満たす時、$\langle L, \land, \lor, 0, 1, \lnot \rangle$ の組をブール代数（ブール束）と呼びます。

- 補元則： $x \land \lnot x = 1$, $x \land \lnot x = 0$

特に台集合が $L = \{0, 1\}$ であるブール代数を2元ブール代数と呼び、この本では $\mathbb{B}$ で表します。

ブール環
--------

ブール代数 $L$ は可換環としても見なすことができます。任意の $x, y \in L$に対して

- 乗法: $x \cdot y = x \land y$
- 加法: $x + y = (x \land \lnot y) \lor (\lnot x \land y)$

として定義すると、$\langle L, +, \cdot \rangle$ は可換環となる事が知られています。この環をブール環と呼びます。逆にブール環 $\langle R, +, \cdot \rangle$ に対して、

- $x \land y = x \cdot y$,
- $x \lor y = x + y + xy$
- $\lnot x = 1 + x$

として $\langle R, \land, \lor, 0, 1, \lnot \rangle$ を定義すると、これはブール代数となる事が知られています。
この対応によってブール代数とブール環を同一視し、特に $\mathbb{B}$ も可換環として扱います。

充足可能性
----------
さてブール代数は可換環と見なせるのでその上の多項式を考えましょう。$\mathbb{B}$ 上の多変数多項式全体を $\mathbb{B}[x_1, x_2, \ldots, x_n]$ と書きます。多項式 $f \in \mathbb{B}[x_1, x_2, \ldots, x_n]$ を一つとると、この多項式を使って方程式 $f(x_1, x_2, \ldots, x_n) = 1$ を考える事ができます。この方程式の解が $\mathbb{B}^n$ に存在するとき、$f$ は充足可能であると言います。SATソルバーは与えられた多項式に対して充足可能性を判定するプログラムです。

多項式の代入として表現できる関数であって、定義域が $\mathbb{B}^n$ であるものを考えましょう。

$$
P = \{ f: \mathbb{B}^n \to \mathbb{B}: (x_1, \ldots, x_n) \mapsto f(x_1, \ldots, x_n)  \mid f \in \mathbb{B}[x_1, \ldots, x_n] \}
$$

これは関数環として和と積を入れるとブール環になります。