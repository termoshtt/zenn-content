---
title: データ並列 (rayon crate)
---

[nikomatsakis/rayon](https://github.com/nikomatsakis/rayon)はデータ並列なコードをiterator形式で簡単に実装するためのライブラリです。C/C++/FortranにおいてOpenMPで並列化していたような部分の代替と考えられます。バックエンドの実装方式としては[Intel Clik](https://www.cilkplus.org/)と同様のwork stealingによります。イテレータを分割してスレッドプールを用いてそれぞれに対して処理を実行します。

簡単な使い方
------------
READMEに詳しく書いてあるので、簡単な紹介だけ：
Rustのコードではイテレータで処理を記述することが多いですが、Rayonは`Iterator`の代わりに`ParallelIterator`を導入します：

- `iter()` → `par_iter()`
- `iter_mut()` → `par_iter_mut()`
- `into_iter()` → `into_par_iter()`

に変更するだけで普通のIteratorと同じように使えるようになっています。

```rust
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(|&i| i * i)
         .sum()
}
```

のように記述するだけで並列に`map`を計算し合計値を計算します。
使える関数は[rayon::par_iter::ParallelIterator](https://docs.rs/rayon/0.5.0/rayon/par_iter/trait.ParallelIterator.html)にまとまっています。

初期化
-------
上記のようなコードは自動的にスレッドプールの初期化を実施します。
スレッドプールの数を変更したい場合は明示的に初期化する必要があります。

```rust
let cfg = rayon::Configuration::new();
rayon::initialize(cfg.set_num_threads(4)).unwrap();
```

詰まったところ
--------------
次のようなコードはコンパイルできません：

```rust
let a = vec![1.0; size];
a.par_iter()
    .map(|x| 2.0 * x)
    .collect();
```

このコードは`collect()`が存在していないためコンパイルできないです。本来`collect()`は[std::iter::FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)を通して定義されますが、`par_iter()`が[ParallelIterator](https://docs.rs/rayon/0.5.0/rayon/par_iter/trait.ParallelIterator.html)を返しているのでそのままでは定義されない(´・ω・｀)。
代わりにRayonには`collect_into()`が定義されているようだ。

```rust
let a = vec![1.0; size];
let result = vec![0.0; size];
a.par_iter()
    .map(|x| 2.0 * x)
    .collect_into(&mut result);
```

事前にvectorを用意しておく必要があるらしい。
[collect_into()](https://docs.rs/rayon/0.5.0/rayon/par_iter/trait.ExactParallelIterator.html#method.collect_into)は事前に長さがわかっているイテレータ(ExactParallelIterator)にのみ定義されているので注意です。

