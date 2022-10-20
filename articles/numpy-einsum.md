---
title: "numpy.einsumの仕様詳細"
emoji: "📦"
type: "tech"
topics: ["numpy", "python", "einsum"]
published: true
---

rust-ndarray用のeinsumマクロを作るためにまずNumPyの実装を調べたので、その内容をまとめていく。

アインシュタインの縮約記法
------------------------
`einsum`の実装を見る前にアインシュタインの縮約記法について確認しておこう。

### テンソル
まずテンソルを次の様に定義する。ある有限個の要素からなる添字集合 $I$ に対して実数値 $\mathbb{R}$ を返す関数

$$
f: I \to \mathbb{R}
$$

を$\mathbb{R}$上の1階のテンソルと定義する。この定義は $\mathbb{R}$ 上の有限次元ベクトル空間において基底を固定した場合に各基底毎に成分一つ決める操作と対応するが、その事は忘れることにする。これは全ての $i \in I$ に対する実数の組 $\{f(i) \vert i \in I\}$ を一つ指定することによって一意に定まるが、特に $I$ は有限集合なので有限個の実数の組を一つ定めるごとに一つ定まる。1階のテンソル全体の集合を

$$
\mathbb{R}^I = \{ f \vert f: I \to \mathbb{R} \}
$$

と書くことにする。これを使ってまた別の有限の添字集合 $J$ に対して2階のテンソルを

$$
\mathbb{R}^{I \times J} = \{ f \vert f: J \to \mathbb{R}^I \}
$$

と定義する。これもやはり有限個の実数値 $\{ f(j)(i) \vert i \in I, j \in J\}$ によって一意に定まることに注意する。再帰的に $n+1$ 階のテンソルを $n$ 階のテンソルを使って

$$
\mathbb{R}^{I_1 \times \dots \times I_{n+1}} = \{ f \vert f: I_{n+1} \to \mathbb{R}^{I_1 \times \dots \times I_n} \}
$$

と定義する。この場合でも各添字の組 $(i_1, \ldots, i_n) \in I_1 \times \cdots \times I_n$ に対する関数の値 $f(i_n) \cdots (i_1)$ によって関数が一意に定まるのは同じである。なのでこのような実数の組とテンソルを同一視することにする。

添字集合を問わずに$n$階テンソル全てを集めたものを $T_n\mathbb{R}$ と書き、特に0階のテンソルを $T_0 \mathbb{R} = \mathbb{R}$ とする。任意階のテンソルの全てを集めた集合を $T\mathbb{R} = \cup_{n \geq 0} T_n\mathbb{R}$ と書くことにする。

### 転置
2階以上のテンソルに対して、$n$番目の添字集合と$m$番目の添字集合の順番を入れ替える操作を $T_{nm}: T_l\mathbb{R} \to T_l\mathbb{R}$ と書こう。ただし $l \geq \max(n, m)$ は整数とする。例えば $\mathbb{R}^{I \times J}$ に対して、

$$
T_{12}: \mathbb{R}^{I \times J} \to \mathbb{R}^{J \times I}
$$

は通常の行列に対する転置に一致する。より高階のテンソルに対しても同様に

$$
T_{23}: \mathbb{R}^{I \times J \times K \times L} \to \mathbb{R}^{I \times K \times J \times L}
$$

の様に一部だけを入れ替える処理を考えることができ、これも転置と呼ぶことにする。

### 積
二つのテンソル $a \in \mathbb{R}^{I_1 \times \cdots \times I_n}$ と $b \in \mathbb{R}^{J_1 \times \cdots \times J_m}$ があるとき、各添字の組 $i_1 \in I_1, \ldots, i_n \in I_n; j_1 \in J_1, \ldots, j_m \in J_m$ に対して積 $c = a \otimes b \in \mathbb{R}^{I_1 \times \cdots \times I_n \times J_1 \times \cdots \times J_m}$ を次で定める

$$
c(j_m)\cdots(j_1)(i_n)\cdots(i_1) = a(i_n)\cdots(i_1) b(j_m)\cdots(j_1)
$$

この2項演算 $\otimes: T\mathbb{R} \times T\mathbb{R} \to T\mathbb{R}$ は添字の順序があるので可換ではないが、添字を順番に並べるので結合的になる

$$
(a \otimes b) \otimes c = a \otimes (b \otimes c) \in \mathbb{R}^{I_1 \times \cdots \times I_n \times J_1 \times \cdots \times J_m \times K_1 \times \cdots \times K_l}
$$

### 縮約
テンソルの添字には例えば $a \in \mathbb{R}^{I \times I}$ のように同じ添字集合が複数回現れる事がある。これについて和をとる事で $T\mathbb{R}$ 上の写像が作れる

$$
\mathbb{R}^{I \times I} \ni a \xmapsto{S_I} \sum_{i \in I} a(i)(i) \in \mathbb{R}
$$

別の添字が含まれる場合、例えば $b \in \mathbb{R}^{I \times I \times J}$ の場合にはその添字についてはそのまま残す

$$
\mathbb{R}^{I \times I \times J} \ni b \xmapsto{S_I} \left\{ \sum_{i \in I} b(j)(i)(i) \middle\vert j \in J \right\} \in \mathbb{R}^J
$$

この写像は特に $T\mathbb{R}$ において添字集合 $I$ を含むテンソルに対して定義され、含まれる全ての $I$ について和をとることにする。

### アインシュタインの縮約記法
これらの演算を組み合わせる事でよく知られている演算を実現できる。例えば二つの1階のテンソル $a, b \in \mathbb{R}^I$ に対して

$$
S_I (a \otimes b) = \sum_{i \in I} a(i) b(i) \in \mathbb{R}
$$

はベクトルの内積であり、二つの2階のテンソル $a \in \mathbb{R}^{I \times J}, b \in \mathbb{R}^{J \times K}$ に対して

$$
S_J (a \otimes b) = \left\{ \sum_{j \in J} a(j)(i) b(k)(j) \middle\vert i \in I, k \in K \right\} \in \mathbb{R}^{I \times K}
$$

は行列積である。
