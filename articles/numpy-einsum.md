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

`einsum` はこの `subscripts` で与えられた文字列に応じて `operands` で与えられたテンソルに対して操作を行いその結果を返します。

この記事では `subscripts` の仕様をまとめていきます。

Implicit mode
--------------
`subscripts` にはimplicit modeとexplicit modeが存在し、implicit modeにおいては `subscripts` は `,` で区切られた引数の添字からなる。EBNF-likeに書くと次のようになる：

```text
index = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` | `m` | `n`
      | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` | `y` | `z`;
subscript = { index };
implicit_subscripts = subscript {`,` subscript }
```

例えば `ij,jk` のような文字列になります。これの評価規則は次の通りです：

- 同じ添字が2回以上現れたら、その添字については和をとる
- 残った添字がある場合はアルファベット順に並べた添字に基づいて `ndarray` を出力とし、添字が無ければスカラー値を出力とする
- 個々の `subscript` と `operands` の次元が違う場合はエラーにする

`ij,jk` に対してはまず `j` が2回現れるので和をとり、結果は `i` と `k` をアルファベット順に並べた `ik` の添字からなる `ndarray` を返します。添字はアルファベット順にソートされるので、例えば `np.einsum("ij", a)` はそのまま `a` が返されますが、`np.einsum("ji", a)` は転置をとります。

```python
>>> a = np.array([[1.0, 2.0], [3.0, 4.0]])
>>> np.einsum("ij", a)
array([[1., 2.], [3., 4.]])
>>> np.einsum("ji", a)
array([[1., 3.], [2., 4.]])
```

添字は0文字も許されます。例えば `,ij` はスカラー値と2階のテンソル値をオペランドにとります。

```python
>>> a = np.array([[1.0, 2.0], [3.0, 4.0]])
>>> np.einsum(",ij", 3, a)
array([[ 3.,  6.], [ 9., 12.]])
```

Explicit mode
--------------
Expicit modeにおいては `subscripts` はimplicit modeの場合の `,` で区切られた引数の添字に続いて `->` が置かれその後に出力の添字が置かれる。EBNF-likeに書くと次の様になる：

```text
index = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` | `m` | `n`
      | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` | `y` | `z`;
subscript = { index };
explicit_subscripts = subscript{`,` subscript } `->` subscript
```

例えば `ij,jk->ik` の様な文字列になる。この評価規則はimplicit modeの場合に比べて少し複雑になって

- 出力に現れた添字については和をとらない
- 出力に含まれない、引数に複数回出現する添字については和をとる
- 出力の添字に基づいて添字が空で無いときは `ndarray` を、空の時はスカラー値を返す
- 出力の添字に引数に現れていない添字があればエラーにする
- 個々の `subscript` と `operands` の次元が違う場合はエラーにする

これによって行列の対角成分をベクトルとして抜き出す操作を `ii->i` のように記述できるようになります。

```python
>>> a = np.array([[1.0, 2.0], [3.0, 4.0]])
>>> np.einsum("ii->i", a)
array([1., 4.])
```

また出力の添字を明示的に書くのでimplicit modeの時のアルファベット順にソートする規則はこの場合ありません。

省略記号 `...` とブロードキャスト
----------------------------------
添字の部分に省略記号 `...` を使うことが出来ます。省略記号は `,` で区切られた個々の添字で1度だけ使え、前後にアルファベットの添字を置けます。

```text
index = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` | `m` | `n`
      | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` | `y` | `z`;
ellipsis = `...`
subscript = { index } [ ellipsis ] { index };
subscripts = subscript {`,` subscript } [ `->` subscript ]
```

省略記号が含まれない場合は対応する `operand` の次元が添字と異なる場合はエラーになりましたが、省略記号が含まれる場合には省略された部分が `operand` から決定されます。例えば4階のテンソルに対して添字 `i...j` に対してが与えられた場合には最初と最後の次元についてはそれぞれ `i` と `j` の添字で表現され、省略部分は間の二つの次元に対応します。

まず一番単純なimplicit modeの添字に省略記号が含まれている場合かつ `operand` が一つの場合を考える。この時の評価規則は次のようになる：

- `operand` の `shape` からアルファベットの添字と省略記号に対応する次元を定める
- 省略記号に対応する次元を出力の最初に並べ、その後にアルファベット順に添字を並べる

例えば次のようになる:

```python
>>> a = np.random.random((2, 3, 4, 5))
>>> a.shape
(2, 3, 4, 5)
>>> np.einsum("i...", a).shape  # i=2, ...=(3, 4, 5)
(3, 4, 5, 2)
>>> np.einsum("...j", a).shape  # j=5, ...=(2, 3, 4)
(2, 3, 4, 5)
>>> np.einsum("i...j", a).shape  # i=2, j=5, ...=(3, 4)
(3, 4, 2, 5)
```

`operands` が複数ある場合、まず個々の添字に対して同じようにマッチを行い、続いて省略した部分についてブロードキャストを行う。例えば `np.einsum("..., ...", a, b)` はオペランドの形状によらず積のブロードキャスト `a * b` と同じ結果になる。部分的に省略する例として：

```python
>>> a = np.random.random((2, 3))
>>> b = np.random.random((2, 1))
>>> np.einsum("..., ...", a, b).shape  # [a] ...=(2, 3), [b] ...=(2, 1)
(2, 3)
>>> np.einsum("i..., ...", a, b).shape  # [a] i=2, ...=(3,), [b] ...=(2, 1)
(2, 3, 2)
>>> np.einsum("...i, ...", a, b).shape  # [a] i=3, ...=(2,), [b] ...=(2, 1)
(2, 2, 3)
>>> np.einsum("..., j...", a, b).shape  # [a] ...=(2, 3), [b] j=2, ...=(1,)
(2, 3, 2)
```

これらのケースではそれぞれ次の様にブロードキャストが行われる：

- `(2, 1)` を `(2, 3)` に昇格
- `(3,)` と `(2, 1)` を `(2, 3)` に昇格して `i=2` を最後に追加して `(2, 3, 2)`
- `(2,)` と `(2, 1)` を `(2, 2)` に昇格して `i=3` を最後に追加して `(2, 2, 3)`
- `(1,)` を `(2, 3)` に昇格して `j=2` を最後に追加して `(2, 3, 2)`

Explicit modeの場合出力の添字にも省略記号が使え、implicit modeのときに最初に省略された次元を省略記号の位置に置く。
