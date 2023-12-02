---
title: "serdeを使ったRustとPythonでの相互変換"
emoji: "⛎"
type: "tech"
topics: ["python", "rust", "serde", "pyo3"]
published: true
publication_name: "jij_inc"
---

https://github.com/Jij-Inc/serde-pyobject

の宣伝・解説記事です。

# RustからPythonを操作する

この記事では同じプロセス内にあるPythonのオブジェクトをRustから操作する方法を解説します。別プロセスのPythonを操作する、例えばサブプロセスとしてPythonを起動して、標準入力を介してPythonコードを送信して結果を標準出力やファイルシステムを介して取得する場合に比べて、同じプロセス内での相互運用には以下のメリットがあります。

- メモリのコピーが不要
- プロセス間通信のオーバーヘッドがない
- プロセス間通信のためのシリアライズ/デシリアライズが不要

同じプロセス内のPythonを操作するには[Python C API](https://docs.python.org/3/c-api/index.html)か、それを適切にラップしたライブラリを使います。この記事では[PyO3](https://github.com/pyo3/pyo3)を使います。PyO3はPythonからRustのコードを使うためのライブラリという印象が強いかもしれませんが、ごくごく単純なケースを除いては一旦Rust側に処理を移した後に引数で渡されたPythonオブジェクトやPythonインタプリタのグローバルな挙動を調整するためにRust側からPythonを操作する必要があります。PyO3を使ってRust側からPythonを操作する方法については公式のユーザーガイドがあるのでこちらも参照してください。

https://pyo3.rs/v0.20.0/python_from_rust

このようなライブラリを使うことにため開発コストが上昇することがこの方法のデメリットと言えるでしょう。

## Pythonの辞書を作る

まずはモチベーションを説明するためにPythonの辞書をRust側から作ってみましょう。なおRust側で辞書を作ってそれをPython側であたかも辞書のように使えるすることもできますが、それは別の解説記事に譲ります。Python上で辞書を作るには次のようにします。

```python
a = {"a": 1, "b": "test"}
```

PyO3には[`eval`](https://docs.rs/pyo3/latest/pyo3/marker/struct.Python.html#method.eval)が用意されているので、Pythonコードの結果をそのままRustで扱うことができます。

```rust
use pyo3::{Python, types::PyDict};

Python::with_gil(|py| {
    let a: &PyDict = py.eval(
        "{'a': 1, 'b': 'test'}",
        None,  // globals
        None   // locals
    )
    .unwrap()    // evalは成功する
    .downcast()  // evalの結果はPyDictにdowncast
    .unwrap();   // downcastは成功する

    // 値が取得できることをテスト
    assert_eq!(
        a.get_item("a")
         .unwrap()  // get_itemは成功する
         .unwrap()  // item "a"は存在する
         .extract::<u32>()  // "a"の値をu32として取り出す
         .unwrap(),         // u32に変換できる
        1
    );
});
```

`unwrap`がたくさんあるので少し詳しくコメントをつけておきました。`eval`や`get_item`のようなPython C API呼び出し自体がそもそも失敗する可能性があることに注意してください。`eval`は実行した結果がどんなデータ型になるか実行してみないと分からないので、辞書型に相当する[`PyDict`](https://docs.rs/pyo3/latest/pyo3/types/struct.PyDict.html)にダウンキャストしてあげます。これは実行時には `eval` したあと出来上がった `PyObject` の持つ型タグをみて、それが `PyDict` に一致するなら成功、異なるならエラーを返します。

では辞書の中身が増えるとどうなるでしょうか？上の例ではPythonコードに直接辞書の中身を書いていますが、Pythonコードを生成しないといけないのでしょうか？その必要はなく、Rust側で直接`PyDict`を作ることができます。

```rust
use pyo3::{Python, types::PyDict};

Python::with_gil(|py| {
    let a = PyDict::empty(py);  // mutでなくていい
    a.set_item("a", 1).unwrap();
    a.set_item("b", "test").unwrap();
    a.set_item("c", 3.14).unwrap();
});
```

Rust側から操作するとそれぞれの操作が成功するかどうか静的にある程度わかるので `unwrap` が少なくなります。ここで `mut a`になってないことにRustユーザーは違和感があるでしょう。これは要は[`RefCell<T>`による内部可変性](https://doc.rust-jp.rs/book-ja/ch15-05-interior-mutability.html)と似たような話で、ランタイムが借用を管理するので参照型レベルでは緩くなっています。Pythonの辞書は異なるデータ型を入れることができるので[`set_item`](https://docs.rs/pyo3/latest/pyo3/types/struct.PyDict.html#method.set_item)は`ToPyObject`を実装している任意の型を受け取ることができます。

なお `serde-pyobject` では[maplit](https://docs.rs/maplit/latest/maplit/)-likeな `pydict!` マクロを使って辞書を作ることができます。

```rust
use pyo3::{Python, types::PyDict};
use serde_pyobject::pydict;

Python::with_gil(|py| {
    let dict: &PyDict = pydict! {
        py,
        "foo" => 42,
        "bar" => "baz"
    }
    .unwrap();
})
```

これは上のような`set_item`を使ったコードに展開されます。

# serde-pyobject

さてここまでで十分に感じるかもしれませんが、Rustユーザーの多く(要出典)は[serde](https://serde.rs/)を日常的に使い慣れているので、`#[derive(Serialize)]`のついたRustの構造体を自動的に`PyDict`に変換してほしいと思うはずです。つまり次の`to_pyobject`が動いてほしいということです。

```rust
use serde::Serialize;
use pyo3::{Python, types::{PyAny, PyDict}};
use serde_pyobject::{to_pyobject, pydict};

#[derive(Serialize)]
struct A {
    a: u32,
    b: String,
}

Python::with_gil(|py| {
    let a = A { a: 1, b: "test".to_string() };
    let obj: &PyAny = to_pyobject(py, &a).unwrap();
    assert!(obj.eq(pydict! { py, "a" => 1, "b" => "test" }.unwrap()).unwrap());
});
```

動きます＼＼\\٩( 'ω' )و //／／

逆に`PyAny`からRustの構造体を復元することもできます。

```rust
use serde::Deserialize;
use pyo3::{Python, types::{PyAny, PyDict}};
use serde_pyobject::{from_pyobject, pydict};

#[derive(Debug, PartialEq, Deserialize)]
struct A {
    a: u32,
    b: String,
}

Python::with_gil(|py| {
    let a: &PyDict = pydict! { py,
      "a" => 1,
      "b" => "test"
    }
    .unwrap();
    let a: A = from_pyobject(a).unwrap();
    assert_eq!(a, A { a: 1, b: "test".to_string() });
});
```

これは[serde data model](https://serde.rs/data-model.html)に従って`PyAny`を[data format](https://serde.rs/data-format.html)であると見なすことで実現しています。対応表は次のとおりです：

| serde data model | PyO3 type | Rust | Python |
|------------------|-----------|------------|---------------|
| `i8`, `i16`, `i32`, `i64`, `isize`, <br> `u8`, `u16`, `u32`, `u64`, `usize` | `PyLong` | `123` | `123` |
| `f32`, `f64` | `PyFloat` | `1.0` | `1.0` |
| `bool` | `PyBool` | `true` | `true` |
| `char`, `string` | `PyString` | `'a'`, `"test"` | `"a"`, `"test"` |
| option | `PyAny` | `None`, `Some(1)` | `None`, `1` |
| unit | `PyTuple` | `()` | `()` |
| unit struct | `PyTuple` | `struct Unit` | `()` |
| unit variant | `PyDict` | `E::A` in `enum E { A, B }` | `"A"` |
| newtype struct | `PyDict` | `A(32)` of `struct A(u8)` | `32` |
| newtype variant | `PyDict` | `E::N(41)` of `enum E { N(u8) }` | `{ "N": 41 }` | 
| seq | `PyList` | `vec![1, 2, 3]` | `[1, 2, 3]` |
| tuple | `PyTuple` | `(1, "test")` | `(1, "test")` |
| tuple struct | `PyDict` | `T(1, "a")` of `struct T(u32, String)` | `(1, "a")` |
| tuple variant | `PyDict` | `E::S(1, 2)` of `enum E { S(u8, u8) }` | `{ "S": (1, 2) }` |
| map | `PyDict` | `hashmap!{ "a".to_string() => 1, "b".to_string() => 2 }` | `{ "a": 1, "b": 2 }` |
| struct | `PyDict` | `A { a: 1, b: "test" }` of `struct A { a: u32, b: String }` | `{ "a": 1, "b": "test"}` |
| struct variant | `PyDict` | `E::S { r: 1, g: 2, b: 3 }` of `enum E { S { r: u8, g: u8, b: u8 } }` | `{ "S": { "r": 1, "g": 2, "b": 3 } }` |

具体例がそれぞれのドキュメントにあるのでそちらも参照してください。

https://docs.rs/serde-pyobject/latest/serde_pyobject/fn.from_pyobject.html
https://docs.rs/serde-pyobject/latest/serde_pyobject/fn.to_pyobject.html

serde data modelについては私の過去の記事でも解説しているので参考になるかもしれません。

https://zenn.dev/termoshtt/articles/serde-typeinfo
https://www.ricos.co.jp/tech/serde-deserializer/


# 最後に

＼Rustエンジニア募集中！／
株式会社Jijでは、数学や物理学のバックグラウンドを活かし、量子計算と数理最適化のフロンティアで活躍するRustエンジニアを募集しています！
詳細は下記のリンクからご覧ください。 **皆様のご応募をお待ちしております！**
https://open.talentio.com/r/1/c/j-ij.com/pages/51062

JijのXのフォローもよろしくお願いします！

https://twitter.com/Jij_Inc_JP/status/1722874215060349290
