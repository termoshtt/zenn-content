---
title: 計算時間を計測する
---

数値計算は数ミリ秒で終わる計算から、数時間あるいは数日かかる計算まであります。数値計算では一般的にディスクやネットワークからデータを取得し、データを使ってメモリ上で計算を行い、計算結果をディスクに書き出しますが、どの処理にどれだけの時間を費やしているかを把握しておく事は計算を高速化する際に極めて重要です。例えばある計算が

- データの読み出し 40%
- メモリ上での計算 40%
- データの書き出し 20%

という内訳になっている場合、メモリ上での計算が半分の時間で行えるようになったとしても、全体の処理時間は0.8倍にしかなりません。これは[アムダールの法則](https://ja.wikipedia.org/wiki/%E3%82%A2%E3%83%A0%E3%83%80%E3%83%BC%E3%83%AB%E3%81%AE%E6%B3%95%E5%89%87)として知られているものの例です。

残念ながらあなたのプログラムの処理時間の内訳はおそらくあなたの想像とは異なります。プログラムの高速化において最も重要な事はまず正確に現在のプログラムにおける個々の処理時間を計測する事です。どの処理にどのくらいの時間を使っているかを把握さえできれば解決方法は自ずと明らかになります。

手動で計測する
--------------
この記事ではいくつか便利な方法を紹介していきますが、最も簡単な方法は自分で計測するコードを追加する方法です。

[std::time](https://doc.rust-lang.org/std/time/index.html)を使うと次の様に測れます：

```rust
use std::{thread, time};

fn main() {
    // 今の時刻を取得
    let now = time::Instant::now();

    // 10ms sleepする
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);

    // 最初の時刻からの経過時間を表示
    println!("{:?}", now.elapsed());
}
```

[`now.elapsed()`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed)は[std::time::Duration](https://doc.rust-lang.org/std/time/struct.Duration.html)型を返します。この型は`Debug` traitを実装しているので、出力は例えば次のようになります：

```text
10.077057ms
```

他にも[Duration::as_secs()](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs)で秒数に変換したりもできます。このように数行コードを挿入するだけで専用のツールの使い方を覚えずとも目的である処理毎の所要時間が計測できます。この出力結果を紙のノートにメモっておけば結果も比較出来ます。まずは測り初めましょう。

cargo-bench
------------
全体の経過時間の内訳でなく、関数なりの単位に切り出した個々の処理の時間だけを比較したい場合は`cargo-bench`と次に説明するマイクロベンチマークフレームワーク`criterion`が便利です。

cargoには[cargo-bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)というサブコマンドが存在して、[Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html)の[`[[bench]]`ターゲット](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#benchmarks)で設定してあるベンチマークを実行します。`benches/`ディレクトリ以下に`*.rs`ファイルを置くと自動的にベンチマーク用のコードだと認識されますが、今回はオプションとして`harness = false`を指定するので次の様に記述します：

```toml:Cargo.toml
[[bench]]
name = "test"
harness = false
```

ここで指定出来るオプションについて詳しくは[cargoにおけるターゲット](https://zenn.dev/termoshtt/articles/cargo-targets)を見てください。このように`Cargo.toml`に`[[bench]]`ターゲットを追加しておくと、

```shell
cargo bench
```

コマンドで`benches/test.rs`の`main`関数が開始されます。`harness`というのはRustの標準ライブラリの中にあるベンチマークの実行環境の事ですが、これはstable toolchainでは使えないので今回は使いません。

デフォルト値である`harness = true`にするとそのターゲットに含まれる`#[bench]`で修飾された関数に対してベンチマークを行います。これは`cargo-test`における`#[test]`の挙動と基本的に同じですが、テストと異なりベンチマークなので複数回計測して平均を取ったりしてくれます。この機能をstableでも動作する形で提供してくれるのが次に述べる`criterion` crateです。

criterion
----------
https://github.com/bheisler/criterion.rs

`criterion` crateは開発中にしか使わないので`[dev-dependencies]`に追加します：

```toml:Cargo.toml
[dev-dependencies]
criterion = { version = "0.4.0", features = ["html_reports"] }
```

`0.4`から以下で述べるHTML出力機能がオプションになったので、これも追加しておきます。

例えばフィボナッチ数を求めるテストは次の様に記述します：

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// フィボナッチ数を求める
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

// ラムダ関数の形でベンチマークを登録する
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// ベンチマークグループを定義する
criterion_group!(benches, criterion_benchmark);

// main関数を用意
criterion_main!(benches);
```

途中で出てくる`black_box`というのは最適化を阻害するための関数です。Rustは非常に強力な最適化機構を備えているので、コンパイルの段階で定数だと分かってしまう項は定数に評価されてしまったり、使わない事が分かる項は計算されずに削除されてしまいます。これでは本来の処理時間が計測出来ません。しかしベンチマークはあくまで最適化された状態での性能を測りたいので、最適化を切る事は出来ません。そこで入力値が実行時まで決まらないように見せかける事で、最適化された状態で処理を行わせる為の機構が`black_box`です。

これを実行すると例えば次のような出力が得られます：

```text
     Running target/release/deps/example-423eedc43b2b3a93
fib 20                  time:   [26.029 us 26.251 us 26.505 us]
Found 11 outliers among 99 measurements (11.11%)
  6 (6.06%) high mild
  5 (5.05%) high severe
```

`criterion`が生成する`main`関数がベンチマークに登録された関数の実行時間を計測して統計的に評価してくれます。現代の計算機では基本的に単一のプロセスCPUを占有している時間は短く複数のプロセスが互いに実行時間を奪い合っているので、処理時間は一般的に他のプロセスの影響を受けてバラつきます。上の例では99回の測定のうち11回の外れ値が出たことが報告されています。

### Report
`criterion`はベンチマーク結果のレポートをHTMLに出力してくれます。この際[Gnuplot](http://www.gnuplot.info/)か[plotters](https://github.com/plotters-rs/plotters)を使ってグラフを書きます。

