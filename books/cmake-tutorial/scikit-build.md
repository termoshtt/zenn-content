---
title: scikit-build
---

[scikit-build](https://github.com/scikit-build/scikit-build)というPythonの拡張モジュールをビルドするための補助ツールがあります。

> Improved build system generator for CPython C, C++, Cython and Fortran extensions 
> http://scikit-build.org

これを使ってcmakeでビルドされたプロジェクトをPythonの拡張モジュールとして配布する方法についてまとめます。まとめたものは以下にあります：

https://gitlab.com/termoshtt/skbuild-example

全体のディレクトリ構成は以下の通りです：

```
├── CMakeLists.txt
├── hello
│   ├── CMakeLists.txt
│   ├── _hello.cpp      # これを拡張モジュールにコンパイル
│   ├── __init__.py
│   └── __main__.py     # python -m hello で呼ばれたときに実行されるファイル
├── LICENSE
├── pyproject.toml
├── README.md
└── setup.py    
```

setup.py
--------
まずは通常のPythonのパッケージと同様にビルド用のスクリプトして`setup.py`を用意しますが、`setuptools`ではなく、`skbuild`の`setup`を使います：

```py:setup.py
from skbuild import setup

setup(
    name="hello",
    version="1.2.3",
    description="a minimal example package",
    author="Toshiki Teramura <toshiki.teramura@gmail.com>",
    license="MIT",
    packages=["hello"],
)
```

pyproject.toml
--------------
依存関係は `requirements.txt` ではなく公式のドキュメントに従って `pyproject.toml` に記述します

```toml:pyproject.toml
[build-system]
requires = ["setuptools", "wheel", "scikit-build", "cmake", "ninja"]
```

最近は[pyproject.toml](https://www.python.org/dev/peps/pep-0518)の制定によって`setup.py`を書かなくても`pip install`できるようになってきているらしいですが、今回はまだ`setup.py`の拡張として提供されている機能を使用します。また[Poetry](https://github.com/sdispater/poetry)にはscikit-buildがどうも対応してなさそう（？）なので今回は使いません。

CMakeLists.txt
--------------
元のコード(scikit-build/tests/samples/hello-cpp)がCMakeLists.txtを分割していたので分けていますが、特に意味はありません。順番に見ていきましょう。

```cmake:CMakeLists.txt
cmake_minimum_required(VERSION 3.5.0)
project(hello)
find_package(PythonExtensions REQUIRED)
add_subdirectory(hello)
```

```cmake:hello/CMakeLists.txt
add_library(_hello MODULE _hello.cpp)
python_extension_module(_hello)
install(TARGETS _hello LIBRARY DESTINATION hello)
```

ポイントとしては `PythonExtensions` を探してきている部分と、`python_extension_module(_hello)`ですね。中身は調べられていないのですが、Pythonの拡張モジュールとして公開する際には必要のようです。

Build
------
setup関数が`setuptools`から`skbuild`に切り替わっていますが、基本的な使い方は同じで、

```
python setup.py bdist_wheel
```

とすればwheelを作成してくれます。wheelは[PEP 427](https://www.python.org/dev/peps/pep-0427)で定義された配布形式で拡張子は `*.whl` となります。whlファイルは実体としてはZIPアーカイブで、コンパイルされた共有ライブラリや他の静的なファイルを含めることができます。
上のコマンドで、`dist/hello-1.2.3-cp37-cp37m-linux_x86_64.whl`のようなファイルが出来上がっているはずです。これはそのままインストールが可能です

```
pip install dist/hello-1.2.3-cp37-cp37m-linux_x86_64.whl --user
```

（`--user`はお好みで）これでGlobalにインストールされたので **別のディレクトリに行って**

```console
$ python -m hello
Hello, World! :)  
```

となると成功です。別のディレクトリに行かないと現在のディレクトリ以下にある `hello` モジュールを読み込みますが、ここにはcmakeで生成された拡張モジュールがありません…(´・ω・｀)

Links
------
- [scikit-build document](https://scikit-build.readthedocs.io/en/latest/)
  - 正直これを読んでも分からん...(´・ω・｀)
- [tests/samples](https://github.com/scikit-build/scikit-build/tree/master/tests/samples)
    - 上のリポジトリはここのhello-cppを拝借した
