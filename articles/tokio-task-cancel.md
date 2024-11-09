---
title: "重い計算をタイムアウトする"
emoji: "⌛"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust", "tokio", "async", "cancel"]
published: true
---

例えば数値計算のように純粋に計算として時間がかかる処理があるとします。この処理に長時間かかる場合、タイムアウトを設定して処理を中断したい場合があります。例えば予め設定した時間を超えて処理が終わらない場合、処理を中断してエラーを返すようにしたい場合です。Rustでこのような処理をどうやって実装するかを議論します。

# 自前でタイムアウトを実装する

まず素朴に自前でタイムアウトを実装してみましょう。次のような仕様を考えます：

- ある関数 `long_calculation` があり、これにタイムアウトを実装したい
- タイムアウトはこの関数の引数として与える
- タイムアウトを超えた場合、エラーを返す

例えば次のような実装をまず考えるでしょう：

```rust
use std::time::Duration;

fn long_calculation(n: usize, timeout: Duration) -> Result<Vec<usize>, String> {
    let start = std::time::Instant::now();
    let mut out = Vec::new();
    for i in 0..n {
        if start.elapsed() > timeout {
            return Err("timeout".to_string());
        }

        // 時間がかかる計算
        std::thread::sleep(Duration::from_secs(1));
        out.push(i /* 計算結果 */);
    }
    Ok(out)
}
```

`n`個の要素を計算する必要があって、一つ計算出来たら時刻を確認してタイムアウトを超えていたらエラーを返す、という実装です。ここでは重い計算の代わりに `std::thread::sleep` を使っています。

この実装だと例えば `long_calculation(100, Duration::from_secs(10))` のようなケースに対しては概ね期待通りに動きます。この時まず `i = 0` から処理を始めると `i=10` の処理を行うときに `start.elapsed() > timeout` が真になり、エラーを返すはずです。するとこの関数呼び出しは概ね10秒で終わるはずです。

では `long_calculation(100, Duration::from_millis(100))` のようなケースはどうでしょうか？この場合は `i=0` の時は `start.elapsed() > timeout` は真にはならないので `i=1` の処理を行うときにタイムアウトが発生するはずです。するとこの呼び出しは `i=0` の処理が終わるまで待つ必要があり、100ミリ秒でなくて1秒待つことになります。このようにタイムアウトしたい処理が実際にタイムアウトされるまでにかかる時間はその関数がどの程度の頻度でタイムアウトをチェックするかに依存します。

この計算を行っているスレッド自体でタイムアウトをチェックしているからこの問題が発生しているのでは無いでしょうか？重い計算を行うスレッドとは別のスレッドでタイムアウトを監視して、タイムアウトしたら計算を中断するようにすればこの問題は解決すると思うかもしれません。
しかしRustのスレッドは外部から強制終了することが出来ないので、タイムアウトが発生した時に出来るのはスレッドをデタッチするだけで、計算自体を中断することは出来ません。するとタイムアウトで処理が返ってきた後に別の計算を行うとすると既に開始した計算と新しい計算が同時に行われるようになります。これは望ましくない挙動です。
なのでネットワークの応答を待つ処理のように結果を捨てればいいだけの場合と違って、重い計算処理のタイムアウトにはこの方法は使えません。

:::message
スレッドを外部から強制停止することは出来ませんが、プロセスなら可能です。なので重い計算自体をサブプロセスで起動し、タイムアウトが発生したらそのプロセスを強制終了するという方法もあります。ただしサブプロセスを起動するのは別の面倒があるのでこの記事では考えません。
:::

# `tokio::task` でのタイムアウト
タイムアウトの精度を上げるには頻繁にタイムアウトをチェックする必要があるという事が分かりました。計算時間が長時間かかる処理というのは大抵の場合複数のサブルーチンを呼び出す事になるので、タイムアウトのチェックはそのサブルーチン内でも同じように行う必要があります。という事は上の例をそのまま使おうと思うと開始時刻とタイムアウトをサブルーチンに渡す必要があり、これは面倒です。

そこで合成可能な中断可能な計算の抽象であるasync/awaitを使ってタイムアウトを実装してみましょう。

```rust
use std::time::Duration;

async fn foo() {
    for i in 0..5 {
        // TODO: タイムアウトをチェックしたい
        println!("foo{}", i);
        std::thread::sleep(Duration::from_secs(1));
    }
}


#[tokio::main]
async fn main() {
    println!("start");
    tokio::select! {
        _ = foo() => println!("foo done"),
        _ = tokio::time::sleep(Duration::from_secs(2)) => println!("timeout"),
    }
    println!("end");
}
```

async修飾された関数はawaitの所で中断可能な関数になります。しかし `foo` は `await` がどこにも入ってないので当然中断できず、タイムアウトは発生しません。

```
start
foo0
foo1
foo2
foo3
foo4
foo done
end
```

この `foo` は一度開始してしまうと1秒x5回が終わるまで他に処理を譲りません。TODOの部分で他のタスク（`tokio::time::sleep`）に処理を一旦明け渡すにはどうすれば良いのでしょうか？
これを実現するには `tokio::task::yield_now` を使います。

```rust
use std::time::Duration;

async fn foo() {
    for i in 0..5 {
        tokio::task::yield_now().await;
        println!("foo{}", i);
        std::thread::sleep(Duration::from_secs(1));
    }
}


#[tokio::main]
async fn main() {
    println!("start");
    tokio::select! {
        _ = foo() => println!("foo done"),
        _ = tokio::time::sleep(Duration::from_secs(2)) => println!("timeout"),
    }
    println!("end");
}
```

これでタイムアウトが実現できます。awaitで中断されたタスクは `select!` で選ばれなかったらDropされるので、スレッドの時のように実行し続ける事はありません。

```
start
foo0
foo1
timeout
end
```

なおこれは実行順序により `foo2` が出るケースもあります。