---
title: "PythonのBuild Backend"
emoji: "🏭"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["python", "pyo3", "maturin", "uv"]
published: false
---

PythonのパッケージシステムはC++やRustで実装されたパッケージをサポートする仕組みを備えていますが、少し複雑なため、問題が起きた際に何が失敗しているのかを把握しづらいケースが多いです。この記事では **現在の仕様に沿って** Pythonのビルドシステムの仕組みを解説します。

:::message
Pythonのパッケージシステムは改良が進むことで表面的な仕様が変化し、一方巷には古い情報が残っているので混乱しがちです。この記事ではその根本の仕組みを解説することで、古い情報に惑わされずに問題解決に取り組むための基礎知識を提供します。
:::

この記事は主に次のドキュメントを参考にしています:

https://packaging.python.org/ja/latest/overview/

## ビルド配布物（Wheel）とソースコード配布物（sdist）

それぞれのツールの役割を議論するにはまずそれらの入出力を確認するのが良いでしょう。Pythonのビルドシステムは主に次の二つの配布物を扱います:
- ソースコード配布物（sdist）: これはさらに二つの用途があります。
  - Pythonのみで記述されたパッケージはPythonスクリプトをそのまま配布するので、ビルドは必要ではありません。単にソースコードを適切なディレクトリ構成で `tar.gz` に固めたものです。
  - C++やRustで記述されたPythonパッケージをユーザーが自分の環境でビルドして使いたいケースがあります。その場合もソースコード配布物として配布します。これはそのままでは使えないので、取得したインストーラがビルドを行います。
- ビルド配布物（Wheel）
  - Pythonのパッケージシステムでは、ユーザー環境でソースコード配布物をビルドさせるのではなく、予めビルドした成果物を配布ることができます。これはユーザー環境にビルド環境を用意する必要がなく、またビルドには多くのリソースを消費するため、ビルド済みパッケージの配布が一般的です。ただしビルドの成果物はOSやPythonのバージョンに依存するので、よくある環境向けに複数のパッケージを用意する必要があります。

## Build FrontendとBackend

Pythonのパッケージシステムではビルドする必要があるソースコード配布物が配布される可能性があるので、パッケージ管理システムはビルドを実行できる必要があります。しかしこの世の全てをビルド出来るツールをパッケージ管理システムに組み込むわけにはいかないので、個々のパッケージ側が何を使ってビルドするのかを指定します。この時に指定されるビルドツールの事をBuild Backendと呼びます。

例えばRust製のPythonパッケージは典型的には次のように `pyproject.toml` に記述します:

```toml:pyproject.toml
[build-system]
requires = ["maturin >= 1.8.2"]  # ビルドバックエンドを起動するのに必要なパッケージを記述
build-backend = "maturin"        # 起動するPythonモジュールを指定
```

`requires` で指定される `maturin` は[Pythonパッケージとしての `maturin`](https://pypi.org/project/maturin/) であり、`build-backend` で指定された `maturin` はPythonのモジュールの事で

```shell
python -m ${build-backend で指定されたモジュール名}
```

と同じ挙動になるはずです。

逆に各パッケージのビルドを委任する側をBuild Frontendと呼び、これは典型的にはパッケージ管理システム、例えば `pip` や `uv` です。

ややこしいことに、Build Backendとして機能するツールにはフロントエンドとしての機能を持っているものもあります。例えば `maturin` はCLIツールとしてビルドだけで無くてインストールやパッケージのアップロードも行えます。なので「`maturin` はBuild Backendツール」ではなく「`maturin`はBuild Backendとしても機能する」と理解するのが適切です。

## Case Study

さて基礎事項を理解したところで、既存のツールについての分類をしていきましょう。

### build (`python -m build`)

https://build.pypa.io/en/stable/

これは `pip` や `uv` のようなパッケージ管理機能とは切り離された、単純にBuild Frontendとして動作するツールです。`pyproject.toml`の`build-backend` を起動します。

### setuptools (`setup.py`)

上の仕組みが出来上がる前から使われている伝統的なツールです。[`61.0.0` から上の仕組みをサポートし](https://setuptools.pypa.io/en/latest/userguide/pyproject_config.html)、Build Backendとして使うことが出来ます。次のように `pyproject.toml` に記述します:

```toml:pyproject.toml
[build-system]
requires = ["setuptools", "setuptools-scm"]
build-backend = "setuptools.build_meta"
```

旧来の `setup.py` による仕組みは廃止される予定ですが、現状の `pip` は `setup.py` を見つけたらBuild Backendとして `setuptools` が指定されたように動作します。

### scikit-build / scikit-build-core

[scikit-build](https://scikit-build.readthedocs.io/en/latest/)はcmakeを使ったビルドをsetuptoolsと連携するためのツールでしたが、前節の通りsetuptoolsのレガシー機構は廃止される予定なので、新たにBuild Backendとして機能する[scikit-build-core](https://scikit-build-core.readthedocs.io/en/latest/)が開発されています。

```toml:pyproject.toml
[build-system]
requires = ["scikit-build-core"]
build-backend = "scikit_build_core.build"

[project]
name = "scikit_build_simplest"
version = "0.0.1"
```