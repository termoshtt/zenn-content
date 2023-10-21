---
title: データ並列 (rayon crate)
---

[rayon](https://github.com/rayon-rs/rayon)はデータ並列のためのライブラリです。そのために次の二つが必要になり、Rayonはこれらを提供してくれます。

- スレッドプール実装
- 安全で便利にスレッドプールで並列処理を記述する事ができるAPI

# Parallel Iterator

例えば整数の配列の各要素の二乗の和を計算する関数を考えましょう。まずはシングルスレッドで書いてみます：

```rust
fn sum_of_squares(input: &[i32]) -> i32 {
    input.iter()
         .map(|&i| i * i)
         .sum()
}
```

これをRayonを使って並列化するには次のように `iter()` を `par_iter()` に置き換えます:

```rust
use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```

標準ライブラリの[Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)の代わりにRayonの[ParallelIterator trait](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html)を実装したイテレータが `par_iter()` で返されるので、`map()` や `sum()` などのメソッドはRayonのものになっています。これらのメソッドはRayonのスレッドプールを使って並列処理を行います。

# 安全な並列処理の為の制約

ここで重要になるのが `map` に与える関数の制約がRayonのものでは強くなっていることです。`Iterator`では

```rust:ignore
fn map<F, R>(self, f: F) -> Map<Self, F> where
    F: FnMut(Self::Item) -> R,
```

だったものがRayonでは次のように `FnMut` でなく `Fn` になりさらに `Sync` と `Send` が必要になります：

```rust:ignore
fn map<F, R>(self, f: F) -> Map<Self, F> where
    F: Fn(Self::Item) -> R + Sync + Send,
    R: Send,
```

これはこの `f` が別スレッドで実行されるかもしれないからです。これは`map`だけなく`for_each`のような他の関数でも同様です。この制約は意外と厳しく、例えば可変参照をキャプチャしたクロージャは使えなくなってしまいます。

```rust
let mut count = 0;
(0..10).for_each(|i| count += i);  // このラムダ式は &mut count をキャプチャしている
assert_eq!(count, 45);
```

なのでこれはそのままRayonに置き換えることができません：

```rust:compile_fail
use rayon::prelude::*;

let mut count = 0;
(0..10).into_par_iter().for_each(|i| count += i);  // これはコンパイルエラー
assert_eq!(count, 45);
```

これにはいくつか解決策があって、例えばこれは `sum()` で書き直せますが、もう少し一般的に同じ値に対する操作をに繰り返すような場合には `reduce` で置き換えられます：

```rust
use rayon::prelude::*;

let count = (0..10)
    .into_par_iter()
    .reduce(
        || 0, // countの初期値を返す。これは複数回呼ばれる
        |count: usize, i: usize| count + i
    );
assert_eq!(count, 45);
```

Reduceは`Vec` を連結したり`HashMap`を統合したりするのにも使えます。

```rust
use std::collections::HashMap;
use rayon::prelude::*;
use maplit::hashmap;

let out = (0..3)
    .into_par_iter()
    .map(|i| hashmap! {
        i.to_string() => i
    })
    .reduce(
        || HashMap::new(),
        |mut map1, map2| {
            map1.extend(map2);
            map1
        }
    );

assert_eq!(out, hashmap! {
    "0".to_string() => 0,
    "1".to_string() => 1,
    "2".to_string() => 2,
});
```

このMap-Reduceという形は並列処理においてよく使われる形です。Map部分でそれぞれのスレッドで独立に計算した結果をReduceで統合することで効率の良い並列処理ができます。