---
title: "numpy.einsumの仕様"
emoji: "🎓"
type: "tech"
topics: ["numpy", "python", "einsum"]
published: true
---

[rust-ndarray](https://github.com/rust-ndarray/ndarray)用の `einsum!` マクロを作るためにまず[NumPy][einsum]の実装を調べたので、その内容をまとめていく。

[einsum]: https://numpy.org/doc/stable/reference/generated/numpy.einsum.html

subscripts
-----------

[numpy.einsum][einsum]の引数はのうち、Optionalで無いのは次の二つ：

- `subscripts`: str
- `operands`: list of array_like

`einsum` はこの `subscripts` で与えられた文字列に応じて `operands` で与えられたテンソルに対して操作を行いその結果を返す。例えば二つの正方行列の積は次の様に書ける：

```python
import numpy as np
a = np.random.random((3, 3))
b = np.random.random((3, 3))
np.einsum("ij,jk", a, b)
```

この記事では `subscripts` の仕様をまとめるのが目的となる。

Implicit mode
--------------
`subscripts` にはimplicit modeとexplicit modeが存在し、implicit modeにおいては `subscripts` は `,` で区切られた引数の添字からなる。EBNF-likeに書くと次のようになる：

```text
index = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` | `m` | `n`
      | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` | `y` | `z`;
subscript = { index] };
implicit_subscripts = subscript {`,` [subscript] }
```

例えば `ij,jk` のような文字列になります。これの評価規則は次の通りです：

- 同じ添字が2回以上現れたら、その添字については和をとる
- 残った添字がある場合はアルファベット順に並べた添字に基づいて `ndarray` を出力とし、添字が無ければスカラー値を出力とする
- 個々の `subscript` と `operands` の次元が違う場合はエラーにする

`ij,jk` に対してはまず `j` が2回現れるので和をとり、結果は `i` と `k` をアルファベット順に並べた `ik` の添字からなる `ndarray` を返します。添字はアルファベット順にソートされるので、例えば `np.einsum("ij", a)` はそのまま `a` が返されますが、`np.einsum("ji", a)` は転置をとります。

Explicit mode
--------------
Expicit modeにおいては `subscripts` はimplicit modeの場合の `,` で区切られた引数の添字に続いて `->` が置かれその後に出力の添字が置かれる。EBNF-likeに書くと次の様になる：

```text
index = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` | `m` | `n`
      | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` | `y` | `z`;
subscript = { index] };
explicit_subscripts = subscript{`,` [subscript] } `->` [subscript]
```

例えば `ij,jk->ik` の様な文字列になる。この評価規則はimplicit modeの場合に比べて少し複雑になって

- 出力に現れた添字については和をとらない
- 出力に含まれない、引数に複数回出現する添字については和をとる
- 出力の添字に基づいて添字が空で無いときは `ndarray` を、空の時はスカラー値を返す

これによって行列の対角成分をベクトルとして抜き出す操作を `ii->i` のように記述できるようになります。また出力の添字を明示的に書くのでimplicit modeの時のアルファベット順にソートする規則はこの場合ありません。

省略記号 `...`
---------------
添字の部分に省略記号 `...` を使うことが出来ます。例えば `np.einsum('...ii->...i', a)`
