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

# エラー型
上で説明したように、エラーというのは一旦発生してもそれを回復できる場合とできない場合があります。またエラーが発生した場合にはそれをどのように扱うかも状況によって異なり、それは正常に動作する場合 (正常系) を実装した時には分からないこともよくあります。公式ドキュメントの中では回復可能なエラーが発生する関数では `Result<T, E>` 型を返すように言われていて、この `T` は正常系の戻り値で `E` はエラーの型です。この `E` をどのように実装すれば上にあるようなシナリオを上手く扱えるでしょうか？

重要なのは次の点です：

- 誰がどのようにエラーを扱う必要があるのか
- そのためにはどのような情報が必要なのか

## エラー型を管理しない
エラー型`E`はそもそも自分で作らないといけないのでしょうか？例えばあなたが自分の計算機上で自分のプログラムを実行する場合、エラーが発生したらそれをログに書いて終了するだけで良いかもしれません。この場合エラーが起きたこととその原因を文字列で表現するだけで十分です。つまり `E = String` として `Result<T, String>` とすればOKです。

例えばファイルの各行に浮動小数点数が書かれているファイルを読み込んで和を返す関数を考えましょう。

```rust
use std::fs::File;
use std::io::{self, BufRead};

fn read_and_sum(filename: &str) -> Result<f64, String> {
    let file = File::open(filename).map_err(|e| format!("File '{filename}' not found: {e}"))?;
    let reader = io::BufReader::new(file);
    let mut sum = 0.0;
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let num = line.parse::<f64>().map_err(|e| format!("Failed to parse line: {line})"))?;
        sum += num;
    }
    Ok(sum)
}
```

この方法はシンプルですが良く動きます。この関数を使うプログラマはエラーが起きたか成功したかを取得することができ、エラーの原因を知ることができます。`map_err`の中の関数は実際にエラーが起きた時だけ実行され正常時には無視されるので、エラーの原因を知るための情報を生成するのに余計なコストがかかることはありません。

しかしちょっと待ってください、文字列に変換する事すら面倒です。もっと簡単にできないでしょうか？これを少し便利にしたものが[anyhow] crateです。

```rust
use std::fs::File;
use std::io::{self, BufRead};

fn read_and_sum(filename: &str) -> Result<f64, anyhow::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0.0;
    for line in reader.lines() {
        let num = line?.parse::<f64>()?;
        sum += num;
    }
    Ok(sum)
}
```

`map_err`も消えて文字列に変換する必要がなくなりました！この`anyhow::Error`というのは`String`と違って、どんなエラー型でも入れることができる[トレイトオブジェクト]で、`Display`と`Debug`を実装しているのでいつでも文字列に変換することができます。

ところで[`File::open`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)が返すエラー型にはどのファイルを開こうとしたのかの情報が含まれていないので、このエラーを直接返してしまうとエラーの原因が分かりにくくなります。この場合は[`anyhow::Context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html)を使ってエラーに情報を追加することができます。

```rust
use std::fs::File;
use std::io::{self, BufRead};
use anyhow::Context;

fn read_and_sum(filename: &str) -> Result<f64, anyhow::Error> {
    let file = File::open(filename)
        .with_context(|| format!("Failed to open data file {filename}"))?;
    let reader = io::BufReader::new(file);
    let mut sum = 0.0;
    for line in reader.lines() {
        let num = line?.parse::<f64>()?;
        sum += num;
    }
    Ok(sum)
}
```

[トレイトオブジェクト]: https://doc.rust-jp.rs/book-ja/ch17-02-trait-objects.html
[anyhow]: https://docs.rs/anyhow/1.0.40/anyhow/

## エラー型を作る

`anyhow::Error`は発生したエラーの型を持っているので、それを使うことでエラーの原因を知ることができます。

```rust
use std::fs::File;
use std::io::{self, BufRead};
use anyhow::Context;

fn read_and_sum(filename: &str) -> Result<f64, anyhow::Error> {
    let file = File::open(filename)
        .with_context(|| format!("Failed to open data file {filename}"))?;
    let reader = io::BufReader::new(file);
    let mut sum = 0.0;
    for line in reader.lines() {
        let num = line?.parse::<f64>()?;
        sum += num;
    }
    Ok(sum)
}

fn use_read_and_sum() {
    match read_and_sum("data.txt") {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => {
            if let Some(io) = e.downcast_ref::<io::Error>() {
                println!("I/O error: {}", io);
            } else {
                println!("Error: {}", e);
            }
        }
    }
}
```

### [std::error::Error] trait

エラー型はこの [std::error::Error] traitを実装する必要があります。いくつか追加の機能がありますが、単純なケースでは `Debug + Display` のことです。

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

このようにエラーを表現する構造体を定義して、それに `std::error::Error` traitを実装することでこれはエラー型として使うことができます。

### [thiserror] crate
[thiserror]: https://docs.rs/thiserror/1.0.58/thiserror/

[thiserror] crateを使って `thiserror::Error` マクロによって定義を簡単に済ますこともできます：

```toml:Cargo.toml
[dependencies]
thiserror = "1.0.58"
```

```rust
#[derive(Debug, thiserror::Error)]
#[error("Iteration does not end after {num_iteration} iterations. Last residual: {last_residual}")]
pub struct MyError {
    num_iteration: usize,
    last_residual: f64,
}
```

これは手続きマクロ(procedural macro, proc-macro)という機能を使って `MyError` の構造体を定義しているRustのコードを読み取って、それから `impl std::error::Error for MyError` 句を生成していて、それをコンパイラが処理しています。[`cargo expand`](https://github.com/dtolnay/cargo-expand)を使うと展開後のコードを確認することができます。生成されたコードは概ね上のものと同じです。

https://doc.rust-lang.org/reference/procedural-macros.html

proc-macroには３種類あり、上の`thiserror::Error`はcustom-deriveと呼ばれるものです。proc-macroはRustのASTを受け取って変更して新しいASTを返すRustで実装された関数を含む特殊なcrateとして提供されます。数値計算ではコンパイル時に確定できるものをコンパイル時に処理しておいて実行時のオーバーヘッドを減らすような工夫が必要になることがありますが、proc-macroを使うと別のテンプレートエンジンやプリプロセッサを使うことなくRustでそれらの機能を実装することができます。しかしそれはここの本題では無いので別のページで詳しく説明することにしましょう。


## 動的な管理: `anyhow` crate

動的な管理と静的な管理の一番の違いは発生しうるエラーのリストを作るか作らないかです。静的な管理ではEnumによって全てのエラーを列挙しますが、動的な管理では[トレイトオブジェクト]を用いることで [std::error::Error] traitを実装した任意のエラー型を扱うことができます。これはどちらもそれぞれ利点があるので、状況に応じて使い分けることが重要です。

[トレイトオブジェクト] `Box<dyn Error>` にいくつか便利機能とそれを生成するためのmacro群を追加したものが [anyhow] crateです

[トレイトオブジェクト]: https://doc.rust-jp.rs/book-ja/ch17-02-trait-objects.html
[std::error::Error]: https://doc.rust-lang.org/std/error/trait.Error.html
[anyhow]: https://docs.rs/anyhow/1.0.40/anyhow/

## 静的な管理: `thiserror` crate

