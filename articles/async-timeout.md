---
title: "重い計算のタイムアウト（ランタイム自作編）"
emoji: "⌛"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust"]
published: false
---

[前回](https://zenn.dev/termoshtt/articles/tokio-task-cancel)の記事では `tokio` を使ってタイムアウト処理を実装しましたが、タイムアウト処理の為だけに `tokio` を使うのは少しオーバースペックかもしれません。前回説明したように非同期に（別スレッドを起動して）タイムアウトしてもスレッドを停止できないので、今回の目的は同期的に（重い処理を行っているスレッド自体で）タイムアウトを行うことであり、なので`tokio`のような非同期ランタイムは必要ないはずです。

今回は `tokio` を使わずに標準ライブラリだけでタイムアウト処理を実装してみます。

:::message
同期的にタイムアウトを行うのに非同期処理のプリミティブである `Future` やasync/awaitの話が始まって奇妙に思うかもしれません。しかしRustの `Future` やasync/awaitは非同期処理には欠かせないものですが、同期処理にも便利であることが分かります。
:::

この記事では [Waker::noop](https://doc.rust-lang.org/std/task/struct.Waker.html#method.noop) を使うのでNightlyを使います。このFeatureは既にStabilize PRが出ているので、近いうちに安定版でも使えるようになるかもしれません。
https://github.com/rust-lang/rust/pull/133089
これはNightlyでないと出来ないわけではなく自分で `Waker::noop` の代替物を用意することも可能なはずですが、少しややこしいのでこの記事では省略します。

# Future and Waker

まずはRustにおける `Future` の操作を振り返りましょう。Async Bookの該当ページを参考にします。
https://rust-lang.github.io/async-book/02_execution/02_future.html

このページでは以下のような簡単にした `SimpleFuture` トレイトを定義しています。

```rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

`Future` というのは非同期処理の為の抽象で、まだ完了していない処理の事を表します。
`Future` には1つだけ `poll` というメソッドがあり、これを呼び出すと `Future` は処理を進めます。処理を進めた結果最終的な値が得られた場合は `Poll::Ready` にその値を入れて返し、処理をそれ以上進められない状態になったとき、例えばネットワークやファイルの読み込みが完了するまで待つ必要がある場合は `Poll::Pending` を返します。

Rustの非同期処理の特徴として、あくまで `Future` は `poll` を呼び出されたときにしか進まないという点があります。`poll` は通常の関数であるので当然同期的に呼び出されます。`Future` というのは非同期処理の抽象ではありますが、非同期処理を行うわけでは無いことに注意してください。

## `Waker`
`poll` には `&mut self` に加えてもう1つ `wake` という引数があります。これは何のためにあるのでしょうか？
上で述べたようにRustの `Future` は `poll` を呼び出したときにしか進みません。この `poll` を呼び出す主体の事をExecutorと呼びましょう。Executorが `Future` に対して `poll` を実行したら `Pending` が帰ってきました場合を考えましょう。Executorはこの時どうするべきでしょうか？　もう一度 `poll` を呼び出すべきでしょうか？　1回 `Pending` したのに即座にもう一度 `poll` を呼び出すとまた `Pending` が返ってくるだけです。これは無駄な処理です。

`wake` はこのような状況を解決するために用意されています。`poll` が `Pending` を返す前に実行可能になった事が分かるように `wake` を仕込んでおきます。例えばファイルの呼び出しを待つ場合にはファイルの読み込みが完了したときのコールバックとして `wake` を登録しておき `Pending` を返します。Executorは `Pending` が返ってきたので `wake` 関数が呼び出されるまでスリープしたり別の `Future` を処理します。カーネルの処理によってファイルの読み込みが完了すると `wake` が非同期に呼び出されるので、Executorはそれを受けて再度 `poll` を呼び出すことが出来ます。

実際の `Future` の `wake` はもう少し複雑な情報をやりとりするために `Waker` や `Context` という構造体を使いますが、役割は概ね同じです。

## `Waker::noop`
以上の説明から分かる通り `wake` というのは「どのタイミングでどの `Future` を進めればいいか分からない」という状況を解決するためのもので、これが自明な場合には必要ありません。「何も通知する必要が無い」を表すための `Waker::noop` という関数が用意されています。これを使うと `wake` に何もしない関数を渡すことが出来ます。

```rust
#![feature(noop_waker)] // Waker::noop

use std::future::Future;
use std::task;

// これは `poll` で `Ready(10)` を返す Future を作る
let future = async { 10 };

// FutureをpollするにはPinする必要がある
let mut boxed_future = Box::pin(future);

// 何も通知しないWaker
let mut cx = task::Context::from_waker(task::Waker::noop());

assert_eq!(
    boxed_future.as_mut().poll(&mut cx),
    task::Poll::Ready(10)
);
```

このように `Waker` の機能を潰すと `Future` は `Iterator` に似ている事が分かります。つまり `poll` を呼び出すと（`next`を呼び出したように）内部の状態が進み、`Iterator` が途中だけ値を返してイテレートが終わったら `None` を返すのと逆で、`Future` は途中では値は返さずに最後だけ値を返します。この例では何も非同期に動作していません。`Future` のExecutorは非同期ランタイムである必要は無いのです。

# Make a Future

`Future` と `Iterator` が似てるのならば `Iterator` 使えばいいのでは？　と思うでしょう。`Future` には `Iterator` にはない特徴があります。それは `async`/`await` 構文です。これはRustコンパイラが `Future` traitを実装している特殊な構造体を自動的に作ってくれる機能で、これこそが非同期処理だけでなく同期処理においても `Future` を使う利点となります。

`Future` traitを実装したオブジェクトを作る方法は大まかに3種類あります。ここではそれらを順番に見ていきましょう。

## 手動で `Future` トレイトを実装する

最初は通常のTraitと同様に自分で構造体あるいはEnumを作って、それに `impl Future for` で実装する方法です。後でタイムアウト処理の為に使うコンポーネントをここで作っておきましょう：

```rust
#[derive(Default)]
struct PendingOnce {
    polled: bool, // boolのデフォルトはfalse
}

impl std::future::Future for PendingOnce {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        if self.polled {
            // 既に１回ポーリングされているので終了
            std::task::Poll::Ready(())
        } else {
            // 初回だけPendingを返す
            self.polled = true; // ２回目以降はReadyを返す

            // Pendingを返した直後からもうこのFutureは準備できているのでwakeを呼んでおく
            cx.waker().wake_by_ref();

            std::task::Poll::Pending
        }
    }
}
```

これは一度だけ `Pending` を返して、次回以降は `Ready` を返す `Future` です。最初に `Pending` を返した段階でもうすぐに次の `poll` の準備が出来ているので無条件に `wake` を呼び出しています。

これを `PendingOnce::default().await` すると（`bool`のデフォルトは`False`なので）`Pending` が返り `poll` からExecutorに処理が戻ります。タイムアウトを実装するならこのときにExecutorがタイムアウトを判定して、まだなら再度 `poll` することで処理を継続し、時間が来ていたらそこで処理を中断できます。

このように自分で `Future` を実装する時はステートマシンを自分で作ることになります。ちょうど `Iterator` を実装するような感覚で、ゼロから `Iterator` を実装した構造体を作るのが少し難しいように `Future` も同じように難しいです。この例では中断された時の状態というのは

- 一度も `poll` が呼ばれていない
- 一度 `poll` が呼ばれて `Pending` が返された

という2つだけなので1つの `bool` で表現出来ています。

:::message
`Future::poll` が一度 `Ready` を返した後に `poll` をもう一度Executorが呼び出した場合の挙動は規定されていません。`Future`の実装によって同じ `Ready` を返すこともパニックすることも許されます。ただし `poll` はSafeな関数で無くてはいけないので未定義動作になることはありません。
:::

通常の非同期処理の為の `Future` では非同期に実行している（`wake`を呼び出す）スレッド側から内部状態を変更するので `Mutex` 等を使う必要がありますが、今回は非同期に動作するものは何もないので必要ありません。

## `async` ブロックを使う

`async`ブロックは `Future` を実装した構造体を簡単に作るための構文です。これはクロージャに似ています。

```rust
let future = async { 42 };
```

これは概ね以下のような `Future` を実装した構造体を作っているのと同じです：

```rust
struct Future42;
impl std::future::Future for Future42 {
    type Output = i32;
    fn poll(self: std::pin::Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<i32> {
        std::task::Poll::Ready(42)
    }
}
```

クロージャ`|| 42` が `Fn` を実装した構造体を作るのに似ています。クロージャと同じように、この構造体はインラインに作られて名前は与えられません。`async`ブロックの重要な機能は `await` を使う事で `Future` を合成することが出来ることです

```rust
#![feature(noop_waker)] // Waker::noop
use std::{future::Future, task::{Context, Poll, Waker}};

let a = async { 10 };
let b = async { 20 };
let c = async { a.await + b.await };

let mut boxed = Box::pin(c);
let mut cx = Context::from_waker(Waker::noop());
assert_eq!(
    boxed.as_mut().poll(&mut cx),
    Poll::Ready(30)
);
```

`async`ブロック中に出現した後置 `.await` はその `Future<Output = T>` から `T` を取り出す、ちょうど `Option<T>` や `Result<T, E>` から `T` を取り出す `?` 演算子と同じような働きをします。`None`や`Err`が出たら処理を中断する `?` と違って、`.await` は `Pending` が返ってきても合成された `Future` が `Pending` になるだけで、以降の処理が続けられます。

`a`も`b`も一度も`Pending`を返さないので、それを合成した`c`も一度も`Pending`を返さず、最初の `Future::poll` で `Ready(30)` を返します。合成された `Future` からはもう `.await` されたのかどうかは分からない事に注意してください。上で作った `PendingOnce` を使って `await` する例を考えてみましょう：

```rust
#![feature(noop_waker)] // Waker::noop
use std::{future::Future, task::{Context, Poll, Waker}};
use article_test::async_timeout::PendingOnce; // 上で作ったのと同じもの

let a = async {
    PendingOnce::default().await;
    10
};
let b = async { 
    PendingOnce::default().await;
    20
};
let c = async { a.await + b.await };

let mut boxed = Box::pin(c);
let mut cx = Context::from_waker(Waker::noop());
assert_eq!(boxed.as_mut().poll(&mut cx), Poll::Pending);
assert_eq!(boxed.as_mut().poll(&mut cx), Poll::Pending);
assert_eq!(boxed.as_mut().poll(&mut cx), Poll::Ready(30));
```

こうすると `a` は一度 `Pending` を返し、次に`Ready(10)`を返します。`b`も同様に2回目の `poll` で `Ready(20)` を返します。これらを合成した `c` は最初の `poll` で `a`由来の `Pending` を返し、次の `poll` で `b` 由来の `Pending` を返し、3回目で `Ready(30)` を返します。

`a`と`b`の取りうる状態は `PendingOnce` 自体と同じでそれぞれ2つ、`c`については上で見たように（`poll`が3回必要だったので）3つの状態がありますが、`async`/`await`の機能によって`Future`を作ると我々は `PendingOnce` を作ったときのように明示的に内部状態を書き下す必要がありません。これが `async`/`await` の利点です。

## `async fn` を使う

`async`ブロックの例で `c` を作るときに `a` と `b` をキャプチャしている事に気を付けてください。クロージャと同じように `async` ブロックでも自動的に環境にある変数をキャプチャします。また戻り値の型が推定されているので注釈を書きたい時に書くところがありません。これらを明示的に書くために `async fn` を使うことが出来ます。

```rust
use article_test::async_timeout::PendingOnce;

async fn f(value: i32) -> i32 {
    PendingOnce::default().await;
    value
}
```

これは概ね次のように展開されます：

```rust
use article_test::async_timeout::PendingOnce;

fn f(value: i32) -> impl std::future::Future<Output = i32> {
    async move {
        PendingOnce::default().await;
        value
    }
}
```

`async move` というのはクロージャの時と同じように、キャプチャしている `value` を `move` するのでついています。`async fn` はパラメータを受け取って `Future` を実装した構造体を作る関数です。`async`ブロックと同じように `await` を使うことが出来ます。

```rust
#![feature(noop_waker)] // Waker::noop
use std::{future::Future, task::{Context, Poll, Waker}};
use article_test::async_timeout::PendingOnce;

async fn f(value: i32) -> i32 {
    PendingOnce::default().await;
    value
}

async fn g(a: i32, b: i32) -> i32 {
    f(a).await + f(b).await
}

let mut boxed = Box::pin(g(10, 20));
let mut cx = Context::from_waker(Waker::noop());
assert_eq!(boxed.as_mut().poll(&mut cx), Poll::Pending); // f(a)のPending
assert_eq!(boxed.as_mut().poll(&mut cx), Poll::Pending); // f(b)のPending
assert_eq!(boxed.as_mut().poll(&mut cx), Poll::Ready(30));
```

`async`ブロックの説明でこれの動作は概ね理解できるでしょう。この例はこの記事の目的にかなり近づいてきました。つまり元々

```rust
fn f(value: i32) -> i32 {
    value
}

fn g(a: i32, b: i32) -> i32 {
    f(a) + f(b)
}
```

のような複数の関数群からなる計算コードがあるときに、同期的にタイムアウトをチェックするタイミングをどうやって挟めばいいか？　というのが本来の目的でした。結論は

- 各関数に `async` をつけ、関数呼び出しに `.await` を挟む
- タイムアウトをチェックしたいところで `PendingOnce::default().await` を挟む

これで各関数の途中でタイムアウトをチェックするポイントを挟んだ計算を1つの大きな `Future` として合成する事が可能になります！

# タイムアウト処理

さて最後にタイムアウト処理を組み込んだExecutorを作りましょう。ここまでで見てきた仕組みを使えばほとんど自明に作れるはずです

```rust
#![feature(noop_waker)] // Waker::noop
use std::{future::Future, task::{Context, Poll, Waker}};
use std::time::{Duration, Instant};
use article_test::async_timeout::PendingOnce;

fn call_with_timeout<T>(timelimit: Duration, f: impl Future<Output = T>) -> Result<T, ()> {
    let start = Instant::now();
    let mut boxed = Box::pin(f);
    let mut cx = Context::from_waker(Waker::noop());

    loop {
        match boxed.as_mut().poll(&mut cx) {
            Poll::Ready(result) => return Ok(result),
            Poll::Pending => {
                if start.elapsed() > timelimit {
                    return Err(());
                }
            }
        }
    }
}

async fn foo() {
    for i in 0..5 {
        PendingOnce::default().await;
        println!("foo{}", i);
        std::thread::sleep(Duration::from_secs(1));
    }
}

assert!(
    call_with_timeout(Duration::from_secs(2), foo()).is_err() // タイムアウトする
);
```

このように非同期処理をしないなら `Future` のExecutorはほんの数行で作れます。またここにタイムアウトだけでなく[PyO3の`check_signals`](https://docs.rs/pyo3/latest/pyo3/marker/struct.Python.html#method.check_signals)でCtrl-Cによるキャンセルを検査するコードも同じように書けるでしょう。