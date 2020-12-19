---
title: プロファイリングを行う (cargo-flamegraph)
---

[ferrous-systems/flamegraph](https://github.com/ferrous-systems/flamegraph)を使ってRustプログラムの性能評価を行っていきます。

![image.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/30426/ea8b3bee-4d06-f13d-fa0d-be7cdfad188e.png)
（[eom/example/bench](https://github.com/termoshtt/eom/blob/master/examples/bench.rs)のベンチマーク結果です）

[FlamgeGraph](https://github.com/brendangregg/FlameGraph)と呼ばれる関数の呼び出しと個々の関数での実行時間を一度に表示したものです。

FlameGraphに関する説明は以下が詳しいです

- [Flame Graphs](http://www.brendangregg.com/flamegraphs.html)
    - [SlideShare](https://www.slideshare.net/brendangregg/blazing-performance-with-flame-graphs)
- [perf + Flame Graphs で Linux カーネル内のボトルネックを特定する](https://yohei-a.hatenablog.jp/entry/20150706/1436208007)

Install
--------

```
cargo install flamegraph
```

これで`flamegraph`, `cargo-flamegraph`がインストールされます。flamegraphは背後でperfを外部プロセスとして実行するのでperfを入れておく必要があります。

### ubuntu/debian

`linux-tools`をインストールします。これはカーネルによって適切なものが変わるので注意してください。

```
sudo apt install linux-tools
```

とすると候補が出るのでその中から適切なものを選んでください。私の場合はGCP上の仮想マシンだったので`linux-tools-gcp`をインストールしました。

### RedHat/CentOS/Fedora

```
sudo yum install perf
```

### Windows
未調査です…

Usage
-------

```
cargo flamegraph
```

で`cargo run --release`の結果をプロファイルしてくれて結果を `flamegraph.svg` を生成してくれます。`-o another_name.svg` のようにすると別名で保存してくれます。また背後でperfを実行するので`perf.data`も作られますが、自分で触る必要はありません。

```toml:Cargo.toml
[profile.release]
debug = true
```

のようにすると関数の情報をある程度残したまま最適化するので結果が見やすくなります。

How to Read
------------

![image.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/30426/c8567a54-d91c-f48b-3289-4afbc1f73a4f.png)
[Blazing Performance with Flame Graphs](https://www.slideshare.net/brendangregg/blazing-performance-with-flame-graphs)より引用

- 関数の呼び出しのスタックに応じて関数を縦に積み上げて、関数での消費時間に応じて横幅を定めます
- `a()`の実行の中で、実際にどの関数で時間を消費しているかは一番上の端を見ればいい（上図の太い部分）。上図では`f()`, `d()`, `e()`, `h()`の順に時間を消費している。

