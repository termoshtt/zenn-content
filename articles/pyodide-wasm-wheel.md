---
title: "pyodide-buildでwasm-based wheelを作りJupyterLiteで使う"
emoji: "🪄"
type: "tech"
topics: ["python", "webassembly", "jupyterlite", "pyodide", "emscripten"]
published: true
---

[Pyodide](https://pyodide.org/en/stable/)はCPythonを[Emscripten](https://emscripten.org/)を用いてWebAssemblyにコンパイルしたもので、Pythonの（一部を除く）標準ライブラリを含むPythonの実行環境をブラウザや他のWebAssembly実行環境上で動かすことが出来ます。[JupyterLite](https://jupyterlite.readthedocs.io/en/stable/)はさらにブラウザの[Web Worker](https://developer.mozilla.org/ja/docs/Web/API/Web_Workers_API/Using_web_workers)としてPyodideを動かしこれをJupyter kernelとすることで、ブラウザのみでJupyterを動かすためのプロジェクトです。

しかしPythonの利点は多くのビルド済みの高速なネイティブ実装が適切にパッケージングされPyPIに存在していて、それを簡単にインストールできることに由来しています。Pyodideでも同様にC言語で書いた拡張機能をパッケージしてインストールできるようにする必要があります。今回は`pyodide-build`を使ってPyodideのためのwasm-based wheelを作成し、これをJupyterLiteで使う方法を紹介します。

なお残念ながら現状(2023/1/7)ではPyPIはwasmのwheelには対応していないため、今回はwheelをGitHub Pagesにアップロードし、HTTPSで取得することにします。

今回使ったコードは以下にあります。
https://github.com/termoshtt/pyodide-wasm-wheel-example

# 環境構築

本当はRust拡張のwheelを作りたいのですが、今回はまずドキュメントに沿ってC拡張をビルドしたwheelを作ることを目標にします。今回使用するのは次のバージョンです：

- [`pyodide-build` 0.24.1](https://pypi.org/project/pyodide-build/0.24.1/)
- [emsdk 3.1.45](https://github.com/emscripten-core/emsdk/releases/tag/3.1.45)

PyodideはEmscriptenの特定のバージョンを必要とします。なので先に `pyodide-build` をインストールしてからそれが必要とするEmscriptenのバージョンをインストールします。

```shell
python -m venv .venv
source .venv/bin/activate
pip install pyodide-build==0.24.1
```

続いてEmscriptenをインストールします。Emscriptenは`emsdk`というツールを使ってインストールします。

```shell
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
PYODIDE_EMSCRIPTEN_VERSION=$(pyodide config get emscripten_version)
./emsdk install ${PYODIDE_EMSCRIPTEN_VERSION}
./emsdk activate ${PYODIDE_EMSCRIPTEN_VERSION}
source emsdk_env.sh
```

`emsdk`は別の方法でインストールしてあることも多いでしょうから適宜読み替えてください。`pyodide`を実行すると `.pyodide-xbuildenv` というディレクトリを作るので`.gitignore`に追加しておきましょう。どうもここにin-treeでビルドする時と同じになるように必要なものを全部ダウンロードしているみたいですね。

# C拡張のビルド

テストとして次のようなC拡張 `lib.c` を用意しましょう

https://github.com/termoshtt/pyodide-wasm-wheel-example/blob/62739cb5089e25ca3e193200bccc6ad979fbb174/c-extension/lib.c

`f` は引数なしで整数値 `3` を返すだけの関数です。これを `setuptools` でビルドします。

https://github.com/termoshtt/pyodide-wasm-wheel-example/blob/62739cb5089e25ca3e193200bccc6ad979fbb174/c-extension/pyproject.toml

https://github.com/termoshtt/pyodide-wasm-wheel-example/blob/62739cb5089e25ca3e193200bccc6ad979fbb174/c-extension/setup.py

これらにはwasm用にビルドするという情報は一切ありません。実際これはそのまま `x86_64` 向けのwheelとしてビルドできます。

これをwasm用にビルドするには `pyodide-build` を使います。

```shell
pyodide build
```

これで `dist` ディレクトリにwheelが出来上がります。上の設定では次のようになります

```text
dist/
└── pyodide_wasm_wheel_example-0.0.0-cp311-cp311-emscripten_3_1_45_wasm32.whl
```

# JupyterLite
さて出来上がったwheelをJupyterLiteで読み込みましょう。JupyterLiteはインストールする必要はなく、次のURLにアクセスするとあなたのブラウザ内で起動します。

https://jupyterlite.readthedocs.io/en/stable/_static/lab/index.html

上で作ったwheelをGitHub PagesにUploadしてあるので、それを読み込んでみましょう：

```python
import micropip
await micropip.install("https://termoshtt.github.io/pyodide-wasm-wheel-example/pyodide_wasm_wheel_example-0.0.0-cp311-cp311-emscripten_3_1_45_wasm32.whl")
```

これでwheelがインストールされました。`micropip.list`でインストールされたパッケージを確認できます。

```text
Name                       | Version | Source
-------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------
packaging                  | 23.1    | pyodide
micropip                   | 0.5.0   | pyodide
piplite                    | 0.2.0   | https://jupyterlite.readthedocs.io/en/stable/_static/extensions/@jupyterlite/pyodide-kernel-extension/static/pypi/piplite-0.2.0-py3-none-any.whl
ipykernel                  | 6.9.2   | pypi
traitlets                  | 5.14.1  | pypi
comm                       | 0.2.1   | pypi
pyodide-kernel             | 0.2.0   | pypi
matplotlib-inline          | 0.1.6   | pypi
asttokens                  | 2.4.1   | pypi
pure-eval                  | 0.2.2   | pypi
ptyprocess                 | 0.7.0   | pypi
pexpect                    | 4.9.0   | pypi
executing                  | 2.0.1   | pypi
stack-data                 | 0.6.3   | pypi
wcwidth                    | 0.2.13  | pypi
prompt-toolkit             | 3.0.43  | pypi
ipython                    | 8.19.0  | pypi
decorator                  | 5.1.1   | pyodide
parso                      | 0.8.3   | pyodide
jedi                       | 0.19.0  | pyodide
six                        | 1.16.0  | pyodide
Pygments                   | 2.16.1  | pyodide
pyodide-wasm-wheel-example | 0.0.0   | https://termoshtt.github.io/pyodide-wasm-wheel-example/pyodide_wasm_wheel_example-0.0.0-cp311-cp311-emscripten_3_1_45_wasm32.whl
sqlite3                    | 1.0.0   | pyodide
```

次にwheelに含まれるモジュールを読み込んでみましょう。

```python
import pyodide_wasm_wheel_example
pyodide_wasm_wheel_example.f()
```

これで `3` が返ってくれば成功です！

# 参考文献

https://pyodide.org/en/stable/development/building-and-testing-packages.html
Pyodideの公式ドキュメントです。out-of-treeで、つまりPyodideのソースコードの外で拡張機能をビルドする方法が紹介されています。今回の記事はこの内容をなぞったものです。

https://zenn.dev/ymd_h/articles/7275cc8dca30e1
この記事では独自に `pyodide-build` に相当すると思われるC拡張ビルドするためのツールを作成しています。
