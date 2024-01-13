---
title: "Rust/PyO3拡張のwasm wheelを作る"
emoji: "🪄"
type: "tech"
topics: ["webassembly", "pyodide", "rust", "pyo3"]
published: true
---

[前回](https://zenn.dev/termoshtt/articles/pyodide-wasm-wheel)はC拡張をwasmにビルドしてPyodideで使う方法をまとめました。今回はRustで書いたPyO3拡張をwasmにビルドしてPyodideで使う方法をまとめます。今回のコードも前回と同じく以下のリポジトリにあります。

https://github.com/termoshtt/pyodide-wasm-wheel-example

# 環境構築
今回直接使うことになるツールは通常のRust/PyO3プロジェクトと同様 `maturin` です

https://github.com/PyO3/maturin

これが `pyodide-build` とほぼ同等の作業、つまりPyO3のプロジェクトをビルドしてpyodide用のwheelを作ってくれます。`pyodide-build`は裏でEmscriptenのCコンパイラ`emcc`を呼び出していましたが、 `maturin` はRustコンパイラの`wasm32-unknown-emscripten`ターゲットを使います。なので `rustup` でこのターゲットをインストールしておきます。このターゲットはRustのnightlyビルドでのみ利用可能です（stableにもインストールできますが、実行するとNightlyの機能を使っているので失敗します）。

```shell
rustup toolchain install nightly
rustup target add --toolchain nightly wasm32-unknown-emscripten
```

RustコンパイラはさらにリンカとしてEmscriptenのコンパイラ `emcc` を使います。なので前回と同様emsdkをインストールします。

```shell
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 3.1.45
./emsdk activate 3.1.45
source emsdk_env.sh
```

emsdkのバージョンは前回とあわせておきました。別バージョンだとどうなるのかは検証していません。

# Rust/PyO3拡張のビルド

まずは `maturin` をインストールします。`pip`で用意するのが簡単でしょう

```shell
pip install maturin
```

PyO3プロジェクトを作るには `maturin new` を使います。

```shell
maturin new -b pyo3 rust-extension
```

これで `rust-extension` というディレクトリが作られます。このディレクトリで `maturin build` とすると通常のwheelがビルドされますが、今回はwasm用にビルドします。まずは`nightly`コンパイラを使うので `rust-toolchain` を書き換えます。

```shell
echo "nightly" > rust-extension/rust-toolchain
```

toolchainファイルについては[rustupのドキュメント](https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file)を見てください。これでwasm wheelがビルドできます。

```shell
cd rust-extension
maturin build --release -o dist --target wasm32-unknown-emscripten -i python3.11
```

この時インタプリタを `-i python3.11` のように指定する必要があることに注意してください。これはおそらく普段はPythonのランタイムがあるのでそこから自動的に取得していますが、今回はpyodideのランタイムはこの段階で存在していないので明示的に指定する必要があるのでしょう。`-o dist`はwheelの出力先です。ここにwheelが出来上がります。

```text
dist
└── rust_extension-0.1.0-cp311-cp311-emscripten_3_1_45_wasm32.whl
```

# Node.jsでテストする
出来上がったwheelをGitHub Pages等にアップロードすることで前回みたようにJupyterLiteから使うことができますが、今回は別の方法でテストしましょう。PyodideはブラウザだけでなくNode.jsでも使うことができるので、これを使ってみましょう。

https://pyodide.org/en/stable/usage/index.html#node-js
https://www.npmjs.com/package/pyodide

まず適当に `npm` 環境を用意しておき、 `pyodide` をインストールします。

```shell
npm install pyodide
```

これで `node_modules` に `pyodide` がインストールされて次のように使うことができます

```javascript:test.js
const { loadPyodide } = require("pyodide");

async function hello_python() {
  let pyodide = await loadPyodide();
  return pyodide.runPythonAsync("1+1");
}

hello_python().then((result) => {
  console.log("Python says that 1+1 =", result);
});
```

```shell
node ./test.js  # => Python says that 1+1 = 2
```

このJavaScriptのAPIのリファレンスは以下にあります

https://pyodide.org/en/stable/usage/api/js-api.html

これはローカルのファイルシステムにあるwheelも読み込むことができます。

https://github.com/termoshtt/pyodide-wasm-wheel-example/blob/7bf00889fe2b5286c41267af21ff92805707d81f/test_rust.js

これはCIでテストするのに便利です。
