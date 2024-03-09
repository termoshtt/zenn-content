---
title: エラーを管理する
---

Rustにおけるエラー処理については公式のドキュメントが非常によく書かれているので、そちらを参照することを強くお勧めします。

https://doc.rust-jp.rs/book-ja/ch09-00-error-handling.html

# 数値計算におけるエラー処理

数値計算と一言で言ってもどのように実行されるかは様々です。研究者が自分で書いたプログラムを自分の計算機上で実行する場合、例えば計算のためのパラメータファイルが見つからなかったらpanicして終了しても問題ないでしょう。一方ユーザーがCADデータをアップロードすると応力を解析してくれるウェブアプリケーションの場合にユーザーの提供したデータが応力の計算に必要な条件を満たさない場合、ユーザーにエラーメッセージを表示してファイルを修正して再度アップロードしてもらうことが望ましいでしょう。大型計算機で数日かかるような計算をするプログラムが何か計算が矛盾していることに気がついてそれ以上計算を続けることが不可能になったなら、その状況をもう一度計算しなくても良いように可能な限り詳細なレポートを生成してから終了したいはずです。このようにエラー処理の方法は状況によって異なります。

ソフトウェアというのは積み上げていくものなので、ある機能は他の高次の機能を実装するのに使われます。例えばあなたが[GMRES]や[BiCGStab]のような線型方程式の反復解法を実装したとすると、それはあなた自身や同僚、あるいはインターネット上の知らない誰かによって[Newton-Krylov法]やさらに非線形最適化や分岐追跡ライブラリを作るのに使われるでしょう。

[BiCGStab]は行列の性質に応じて収束しないことがよくあります。[BiCGStab]の実装においては行列は所与なのでこれを変えるわけにはいかず、この関数にとってはどうしようもないのでエラーを返すことになります。これは行列の性質、特に固有値の分布のような静的にはわからない性質に依存しているのでエラーになるのかどうかは実行してみないと分かりません。

一方[Newton-Krylov法]のレイヤーで見ると[BiCGStab]が収束しないのはNewton法側のイテレーションによって点を移動させすぎたためで、[line search]や[trust region]等の減速法によって解決できるかもしれません。なので[BiCGStab]が返したエラーは[Newton-Krylov法]のレイヤーで処理することができる回復可能なエラーとなります。もちろんこれらの方法でエラーが回復できない場合は[Newton-Krylov法]の関数がエラーを返すことになります。同じことが非線形最適化や分岐追跡のようなレイヤーが増えるたびに繰り返されます。

[GMRES]: https://en.wikipedia.org/wiki/Generalized_minimal_residual_method
[BiCGStab]: https://en.wikipedia.org/wiki/Biconjugate_gradient_stabilized_method
[Newton-Krylov法]: https://en.wikipedia.org/wiki/Newton%E2%80%93Krylov_method
[line search]: https://en.wikipedia.org/wiki/Line_search
[trust region]: https://en.wikipedia.org/wiki/Trust_region

# エラー型の設計
上で説明したように、エラーというのは一旦発生してもそれを回復できる場合とできない場合があります。またエラーが発生した場合にはそれをどのように扱うかも状況によって異なり、それは正常に動作する場合 (正常系) を実装した時には分からないこともよくあります。公式ドキュメントの中では回復可能なエラーが発生する関数では `Result<T, E>` 型を返すように言われていて、この `T` は正常系の戻り値で `E` はエラーの型です。この `E` をどのように実装すれば上にあるようなシナリオを上手く扱えるでしょうか？

エラー型 `E` の実装方針は以下で詳しく述べるように大きく分けて2通り存在します。とはいえ重要なのは

- 誰がどのようにエラーを扱う必要があるのか
- そのためにはどのような情報が必要なのか

ということです。これらの点にさえ気をつけていればどちらの方法を取ったとしても大きな問題はないでしょう。

## [std::error::Error] trait

エラー型はこの [std::error::Error] traitを実装する必要がありますが、 実質的にはこのtraitは `Debug + Display` のことです。

```rust
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    num_iteration: usize,
    last_residual: f64,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Iteration does not end after {} iterations. Last residual: {}", self.num_iteration, self.last_residual)
    }
}

impl std::error::Error for MyError {}
```

## 動的な管理: `anyhow` crate

動的な管理と静的な管理の一番の違いは発生しうるエラーのリストを作るか作らないかです。静的な管理ではEnumによって全てのエラーを列挙しますが、動的な管理では[トレイトオブジェクト]を用いることで [std::error::Error] traitを実装した任意のエラー型を扱うことができます。これはどちらもそれぞれ利点があるので、状況に応じて使い分けることが重要です。

[トレイトオブジェクト] `Box<dyn Error>` にいくつか便利機能とそれを生成するためのmacro群を追加したものが [anyhow] crateです

[トレイトオブジェクト]: https://doc.rust-jp.rs/book-ja/ch17-02-trait-objects.html
[std::error::Error]: https://doc.rust-lang.org/std/error/trait.Error.html
[anyhow]: https://docs.rs/anyhow/1.0.40/anyhow/

## 静的な管理: `thiserror` crate

