---
title: 計算時間を計測する
---

数値計算は数ミリ秒で終わる計算から、数時間あるいは数日かかる計算まであります。数値計算では一般的にディスクやネットワークからデータを取得し、データを使ってメモリ上で計算を行い、計算結果をディスクに書き出しますが、どの処理にどれだけの時間を費やしているかを把握しておく事は計算を高速化する際に極めて重要です。例えばある計算が

- データの読み出し 40%
- メモリ上での計算 40%
- データの書き出し 20%

という内訳になっている場合、メモリ上での計算が半分の時間で行えるようになったとしても、全体の処理時間は0.8倍にしかなりません。これは[アムダールの法則](https://ja.wikipedia.org/wiki/%E3%82%A2%E3%83%A0%E3%83%80%E3%83%BC%E3%83%AB%E3%81%AE%E6%B3%95%E5%89%87)として知られているものの例です。

手動で計測する
--------------
この記事ではいくつか便利な方法を紹介していきますが、最も簡単な方法は自分で計測するコードを追加する方法です。[std::time](https://doc.rust-lang.org/std/time/index.html)を使うと次の様に測れます：

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

出力は例えば次のようになります：

```
10.077057ms
```

このように数行コードを挿入するだけで専用のツールの使い方を覚えずとも目的である処理毎の所要時間が計測できます。この出力結果を紙のノートにメモっておけば結果も比較出来ます。まずは測り初めましょう。

cargo-bench
------------
cargoには[cargo-bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)というサブコマンドが存在して、[Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html)の[`[[bench]]`ターゲット](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#benchmarks)で設定してあるベンチマークを実行します。

```toml:Cargo.toml
[[bench]]
name = "bench-test"
path = "benches/test.rs"
harness = false
```

このように`Cargo.toml`に`[[bench]]`ターゲットを追加しておくと、

```
cargo bench bench-test
```

コマンドで`benches/test.rs`の`main`関数が開始されます。ベンチマーク名`bench-test`を省略すると登録されている全ての`[[bench]]`ターゲットを実行します。

`harness`というのはRustの標準ライブラリの中(libtest)にあるベンチマークの実行環境の事で、`harness = true`にするとそれを使いますが、この実行環境はNightly環境が必要なので今回は割愛します。`harness = false`の時は`[[bin]]`のターゲットと同様に`main`関数を開始します。

criterion.rs
-------------
TODO
