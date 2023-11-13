---
title: "PyO3拡張にPythonの型ヒントを付ける"
emoji: "🔗"
type: "tech"
topics: ["rust", "pyo3", "python"]
published: true
publication_name: "jij_inc"
---

# Pythonの型ヒントとstub file

Python 3.5 ([PEP 484](https://peps.python.org/pep-0484/)) から型ヒントが導入され、Pythonのコードに型を付けることが出来るようになりました。これはPythonのコードを読むときに型を理解しやすくなるだけでなく、IDEなどのツールが型を利用して補完やリファクタリングを行うことが出来るようになります。また、型ヒントを付けることで静的解析を行うことが出来るようになり、実行時のエラーを減らすことが出来ます。

```python
def greeting(name: str) -> str:
    return 'Hello ' + name
```

Pythonの型ヒントの歴史と仕組みは別の記事に譲るとして、この記事ではRustのPyO3拡張にPythonの型ヒントを付ける方法を紹介します。

## Stub file (`*.pyi`)
Pythonの型ヒント上述したようなPythonコード内にインラインに埋め込む形式に加えて、[PEP 561](https://www.python.org/dev/peps/pep-0561/)で定義されるstub fileと呼ばれる `*.pyi` の拡張子を持つ別ファイルに記述でき、これにより元のPythonコードを改変することなく型ヒントを追加できます。これは次の2つのシナリオにおいて有用です：

- 既存のPythonコードを改変できない・したくない場合
  - 例えば、開発元が開発を停止しており型ヒントをつけてくれる見込みがない場合、元のパッケージとは別にパッケージを作って元のパッケージの型ヒントだけを追加で提供できる
- ライブラリがPythonコードでなくバイナリで提供されている場合
  - この記事で解説するように、RustやC++で書かれたライブラリをPythonから使う場合
  - 現状(3.12) Python C APIでパッケージを作る際に型ヒントを付与する機能は存在していない

### Links
- [Type Stub Files -- pyright document](https://microsoft.github.io/pyright/#/type-stubs)

# 手動で stub file を書く
この記事では [`PyO3/maturin`](https://github.com/PyO3/maturin) を用いてRustを使ったパッケージを生成する想定でstub fileを設定します。別の方法としては [`setuptools-rust`](https://github.com/PyO3/setuptools-rust)がありますがこれについては触れません。

## maturinの使い方
まず準備として `maturin` の使い方を一通り説明します。なおこの記事では[pip](https://github.com/pypa/pip)を使います。[poetry](https://github.com/python-poetry/poetry)や[rye](https://github.com/mitsuhiko/rye)等を使う場合は適宜読み替えてください。

### Install `maturin`

```bash
pip install maturin
```

### Create project by `maturin new`

```bash
maturin new -b pyo3 my_first_stub_file
```

これで次のようなディレクトリ構成でプロジェクトが作成されます。

```
my_first_stub_file/
├── Cargo.toml
├── pyproject.toml
└── src
    └── lib.rs
```

いくつか今回の目的に関係ない部分を除くとそれぞれのファイルの中身は次のようになっています。

```toml:Cargo.toml
[package]
name = "my_first_stub_file"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_first_stub_file"  # これが共有ライブラリの名前になる
crate-type = ["cdylib"]

[dependencies]
pyo3 = "0.19.0"
```

```toml:pyproject.toml
[build-system]
requires = ["maturin>=1.1,<2.0"]
build-backend = "maturin"

[project]
name = "my_first_stub_file"
requires-python = ">=3.7"

[tool.maturin]
features = ["pyo3/extension-module"]   # maturin buildする時にcargoに渡すフラグ
```

```rust:src/lib.rs
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn my_first_stub_file(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

### Build

まず仮想環境を用意しておきましょう。

```bash
cd my_first_stub_file/
python -m venv .venv
source .venv/bin/activate
```

次にビルドします。この時仮想環境の外でビルドすると失敗するので注意してください。

```bash
maturin develop
```

これで`Cargo.toml`と`pyproject.toml`の設定に従って`target/wheels`以下にwheelを生成し、仮想環境にインストールします。あるいは `pip` は `pyproject.toml` があるディレクトリをパッケージとみなすので

```bash
pip install .
```

とすると `build-backend = "maturin"` に従って `maturin build` が実行されます。

これでPythonからRustの関数を呼び出すことができるようになります。

```bash
python << EOF
import my_first_stub_file
print(my_first_stub_file.sum_as_string(1, 2))
EOF
```

### メタデータの確認

Rustで実装された `sum_as_string` がPython側でどう認識されているか確認してみましょう

```bash
pip install ipython
ipython
```

```
In [1]: import my_first_stub_file

In [2]: my_first_stub_file.sum_as_string?
Signature: my_first_stub_file.sum_as_string(a, b)
Docstring: Formats the sum of two numbers as string.
Type:      builtin_function_or_method
```

この `Docstring` はRustコードのドキュメントコメント (`///` の部分) のものです。このようにPyO3はRustのドキュメントコメントをPythonのdocstringとして扱います。しかしRustで実装した時にはあった引数 `a` と `b` が `usize` で戻り値が `String` であるという情報は失われています。これを補うのがstub fileです。

### Links
- [Maturin User Guide](https://www.maturin.rs/)
- [PyO3: これまで/これから](https://qiita.com/kngwyu/items/5e5fe2e2fbf19ce3fe38)
- [PythonとRustの融合⁠⁠：PyO3/maturinを使ったPythonバインディングの作成入門](https://gihyo.jp/article/2023/07/monthly-python-2307)

## stub file を書く

Pythonの型ヒントは実行時には捨てられてしまうので、この型ヒントを使うには追加で型検査機が必要になります。ここでは [pyright](https://github.com/microsoft/pyright)を使いましょう。これはVisual Studio CodeのPython拡張機能にも組み込まれている型検査機です。

```bash
pip install pyright
```

### インライン型ヒントの場合
さてまずpyrightの使い方を見るため、インラインに型ヒントを書いた場合を見てみましょう

```python:test_inline.py
def sum_as_string1(a, b):
    return str(a + b)


def sum_as_string2(a: int, b: int) -> str:
    return str(a + b)


sum_as_string1(1, "2")
sum_as_string2(1, "2")
```

型ヒントは2つの整数を取ると言っているのに、文字列を渡しています。これをpyrightで検査すると次のようなエラーが出ます。

```
$ pyright test_inline.py 
/zenn-content/my_first_stub_file/test_inline.py
  /zenn-content/my_first_stub_file/test_inline.py:10:19 - error: Argument of type "Literal['2']" cannot be assigned to parameter "b" of type "int" in function "sum_as_string2"
    "Literal['2']" is incompatible with "int" (reportGeneralTypeIssues)
1 error, 0 warnings, 0 informations 
```

型ヒントの付いている`sum_as_string2`でだけエラーになっていることがわかります。

### stub fileの場合

まずRust実装を呼び出すコードを書いてみましょう。

```python:test_stub.py
import my_first_stub_file

my_first_stub_file.sum_as_string(1, "2")
```

これは整数を受け取るべきところを文字列を渡しているので実行時に失敗します。

```
$ python test_stub.py 
Traceback (most recent call last):
  File "/zenn-content/my_first_stub_file/test_stub.py", line 3, in <module>
    my_first_stub_file.sum_as_string(1, "2")
TypeError: argument 'b': 'str' object cannot be interpreted as an integer
```

pyrightを実行するとどうなるでしょうか？

```
$ pyright test_stub.py 
/zenn-content/my_first_stub_file/test_stub.py
  /zenn-content/my_first_stub_file/test_stub.py:3:20 - error: "sum_as_string" is not a known member of module "my_first_stub_file" (reportGeneralTypeIssues)
1 error, 0 warnings, 0 informations 
```

そもそも `sum_as_string` なんて関数は `my_first_stub_file` のメンバーとして知られていないと言ってますね。それもそのはず、この関数はRustで実装された共有ライブラリをPythonがロードすると、その初期化ルーチン内で定義されてpyrightは共有ライブラリは読み込まないので知る由もありません。これを教えてくれるのがstub fileです。次ようなファイルを作ります

```python:my_first_stub_file.pyi
def sum_as_string(a: int, b: int) -> str: ...
```

最終的なディレクトリ構成は次のようになります。

```
my_first_stub_file/
├── test_stub.py
├── test_inline.py
├── Cargo.toml
├── pyproject.toml
├── my_first_stub_file.pyi   # NEW!
└── src
    └── lib.rs
```

これを保存して `maturin develop` で再度ビルドすると `pyright` がstub fileを読み込めるようになります。

```
$ pyright test_stub.py
/zenn-content/my_first_stub_file/test_stub.py
  /zenn-content/my_first_stub_file/test_stub.py:3:37 - error: Argument of type "Literal['2']" cannot be assigned to parameter "b" of type "int" in function "sum_as_string"
    "Literal['2']" is incompatible with "int" (reportGeneralTypeIssues)
1 error, 0 warnings, 0 informations 
```

インラインの時と同様に整数を渡すべきところに文字列を渡しているというエラーになりました！

### Links
- [Adding Python type information -- maturin user guide](https://www.maturin.rs/project_layout.html?highlight=stub#adding-python-type-information)

# Rust/PyO3実装から自動でstub fileを生成する
のはまた次回(´・ω・｀)

# 最後に

＼Rustエンジニア募集中！　／
株式会社Jijでは、数学や物理学のバックグラウンドを活かし、量子計算と数理最適化のフロンティアで活躍するRustエンジニアを募集しています！
詳細は下記のリンクからご覧ください。 **皆様のご応募をお待ちしております！**
https://open.talentio.com/r/1/c/j-ij.com/pages/51062

JijのXのフォローもよろしくお願いします！

https://twitter.com/Jij_Inc_JP/status/1722874215060349290
