---
title: "Futureのタイムアウト"
emoji: "⌛"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust", "async", "cancel"]
published: true
---

[前回](https://zenn.dev/termoshtt/articles/tokio-task-cancel)の記事では `tokio` を使ってタイムアウト処理を実装しましたが、タイムアウト処理の為だけに `tokio` を使うのは少しオーバースペックかもしれません。今回は `tokio` を使わずに `std::future` だけでタイムアウト処理を実装してみます。

今回は [Waker::noop](https://doc.rust-lang.org/std/task/struct.Waker.html#method.noop) を使うのでNightlyを使います。このFeatureは既に Stabilize PRが出ているので、安定版でも使えるようになるかもしれません。
https://github.com/rust-lang/rust/pull/133089
これはNightlyでないと出来ないわけではなく自分で `Waker::noop` の代替物を用意することも可能なはずですが、少しややこしいのでこの記事では省略します。

# RustのFuture

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
`Future` には一つだけ `poll` というメソッドがあり、これを呼び出すと `Future` は処理を進めます。処理を進めた結果最終的な値が得られた場合は `Poll::Ready` にその値を入れて返し、処理をそれ以上進められない状態になったとき、例えばネットワークやファイルの読み込みが完了するまで待つ必要がある場合は `Poll::Pending` を返します。

Rustの非同期処理の特徴として、あくまで `Future` は `poll` を呼び出されたときにしか進まないという点があります。`poll` は通常の関数であるので当然同期的に呼び出されます。`Future` というのは非同期処理の抽象ではありますが、非同期処理を行うわけでは無いことに注意してください。

## `Waker`
`poll` には `&mut self` に加えてもう一つ `wake` という引数があります。これは何のためにあるのでしょうか？
上で述べたようにRustの `Future` は `poll` を呼び出したときにしか進みません。この `poll` を呼び出す主体の事をExecutorと呼びましょう。Executorが `Future` に対して `poll` を実行したら `Pending` が帰ってきました場合を考えましょう。Executorはこの時どうするべきでしょうか？もう一度 `poll` を呼び出すべきでしょうか？一回 `Pending` したのに即座にもう一度 `poll` を呼び出すとまた `Pending` が返ってくるだけです。これは無駄な処理です。

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

## async/await

`Future` と `Iterator` が似てるのならば `Iterator` 使えばいいのでは？と思うでしょう。`Future` にはもう一つの特徴があります。それは `async`/`await` 構文です。

`Future` traitを実装したオブジェクトを作る方法は大まかに３種類あります。

### 手動で `Future` トレイトを実装する

最初は通常のTraitと同様に自分で構造体あるいはEnumを作って、それに `impl Future for` で実装する方法です。後でタイムアウト処理の為に使うコンポーネントをここで作っておきましょう：

```rust
#[derive(Default)]
struct PendingOnce {
    polled: bool,
}

impl Future for PendingOnce {
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

### `async` ブロックを使う

### `async fn` を使う
