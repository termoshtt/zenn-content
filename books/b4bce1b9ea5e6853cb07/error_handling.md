---
title: エラーを管理する
---

Rustにおけるエラー処理については公式のドキュメントが非常によく書かれているので、そちらを参照することを強くお勧めします。

https://doc.rust-jp.rs/book-ja/ch09-00-error-handling.html

# 数値計算におけるエラー処理

数値計算と一言で言ってもどのように実行されるかは様々です。研究者が自分で書いたプログラムを自分の計算機上で実行する場合、例えば計算のためのパラメータファイルが見つからなかったらpanicして終了しても問題ないでしょう。一方ユーザーがCADデータをアップロードすると応力を解析してくれるウェブアプリケーションの場合にユーザーの提供したデータが応力の計算に必要な条件を満たさない場合、ユーザーにエラーメッセージを表示してファイルを修正して再度アップロードしてもらうことが望ましいでしょう。大型計算機で数日かかるような計算をするプログラムが何か計算が矛盾していることに気がついてそれ以上計算を続けることが不可能になったなら、その状況をもう一度計算しなくても良いように可能な限り詳細なレポートを生成してから終了したいはずです。このようにエラー処理の方法は状況によって異なります。

ソフトウェアというのは積み上げていくものなので、ある機能は他の高次の機能を実装するのに使われます。例えばあなたが[GMRES]や[BiCGStab]のような線型方程式の反復解法を実装したとすると、それはあなた自身や同僚、あるいはインターネット上の誰かによって[Newton-Krylov法]やさらに非線形最適化や分岐追跡ライブラリを作るのに使われるでしょう。

[BiCGStab]は行列の性質に応じて収束しないことがよくあります。[BiCGStab]の実装においては行列は所与なのでこれを変えるわけにはいかず、この関数にとってはどうしようもないのでエラーを返すことになります。これは行列の性質、特に固有値の分布のような静的にはわからない性質に依存しているのでエラーになるのかどうかは実行してみないと分かりません。

一方[Newton-Krylov法]のレイヤーで見ると[BiCGStab]が収束しないのはNewton法側のイテレーションによって点を移動させすぎたためで、[line search]や[trust region]等の減速法によって解決できるかもしれません。なので[BiCGStab]が返したエラーは[Newton-Krylov法]のレイヤーで処理できる回復可能なエラーとなります。もちろんこれらの方法でエラーが回復できない場合は[Newton-Krylov法]の関数がエラーを返すことになります。同じことが非線形最適化や分岐追跡のようなレイヤーが増えるたびに繰り返されます。

# エラー型
上で説明したように、エラーというのは一旦発生してもそれを回復できる場合とできない場合があります。またエラーが発生した場合にはそれをどのように扱うかも状況によって異なり、それは正常に動作する場合 (正常系) を実装した時には分からないこともよくあります。公式ドキュメントの中では回復可能なエラーが発生する関数では `Result<T, E>` 型を返すように言われていて、この `T` は正常系の戻り値で `E` はエラーの型です。この `E` をどのように実装すれば上にあるようなシナリオを上手く扱えるでしょうか？

重要なのは次の点です：

- 誰がどのようにエラーを扱う必要があるのか
- そのためにはどのような情報が必要なのか

## エラー型を作らない
エラー型`E`はそもそも自分で作らないといけないのでしょうか？　例えばあなたが自分の計算機上で自分のプログラムを実行する場合、エラーが発生したらそれをログに書いて終了するだけで良いかもしれません。この場合エラーが起きたこととその原因を文字列で表現するだけで十分です。つまり `E = String` として `Result<T, String>` とすればOKです。

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

この方法はシンプルですが良く動きます。この関数を使うプログラマはエラーが起きたか成功したかを取得でき、エラーの原因を知ることができます。`map_err`の中の関数は実際にエラーが起きた時だけ実行され正常時には無視されるので、エラーの原因を知るための情報を生成するのに余計なコストがかかることはありません。

しかしちょっと待ってください、文字列に変換する事すら面倒です。もっと簡単にできないでしょうか？　これを少し便利にしたものが[anyhow] crateです。

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

`map_err`も消えて文字列に変換する必要がなくなりました！　この`anyhow::Error`というのは`String`と違って、どんなエラー型でも入れることができる[トレイトオブジェクト]で、`Display`と`Debug`を実装しているのでいつでも文字列に変換できます。

ところで[`File::open`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open)が返すエラー型にはどのファイルを開こうとしたのかの情報が含まれていないので、このエラーを直接返してしまうとエラーの原因が分かりにくくなります。この場合は[`anyhow::Context`](https://docs.rs/anyhow/latest/anyhow/trait.Context.html)を使ってエラーに情報を追加できます。

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


## エラー型を作る

さてエラーが起きたことさえ知れればいいならエラー型をわざわざ自分で定義する必要は無いことは分かりました。ではエラー型を定義する必要があるのはどのようなケースでしょうか？

ここで最初に説明した[Newton-Krylov法]のケースを考えましょう。Newton法が減速法を適用しようとしたとき、[BiCGStab]の最終的な誤差あるいは誤差の履歴によって適用する減速法を変えたいかもしれません。[BiCGStab]が文字列でエラーを返しているとすると誤差の履歴を文字列で書き込んで、Newton法側でそれを解析するのでしょうか？　それはあまりに辛すぎます。そこで誤差の履歴を格納したエラー型を定義しましょう：

```rust
struct Matrix { /* ... */ };
struct Vector { /* ... */ };

#[derive(Debug)]
struct BiCGStabError {
    residual_history: Vec<f64>,
}

fn bicgstab(a: &Matrix, b: &Vector, threshold: f64, max_iteration: usize) -> Result<Vector, BiCGStabError> {
    let mut residual_history = Vec::new();
    for _ in 0..max_iteration {
        // BiCGStab iteration

        let residual = todo!();
        residual_history.push(residual);

        if residual < threshold {
            return Ok(todo!());
        }
    }
    Err(BiCGStabError { residual_history })
}
```

このようにエラー型を定義することでエラーの原因を知ることができます。つまり失敗したときの情報を呼び出し元に伝える必要がある場合は、その情報を伝えるためのエラー型を定義することになります。

### [std::error::Error] trait

エラー型は[std::error::Error] traitを実装する必要があります。いくつか追加の機能がありますが、単純なケースでは`Debug + Display`を実装すれば十分です。今回のケースでは`Display`も対して効果的な表示が出来ないので自動で実装される`Debug`をそのまま使いましょう：

```rust
use std::fmt;

#[derive(Debug)]
struct BiCGStabError {
    residual_history: Vec<f64>,
}

impl fmt::Display for BiCGStabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BiCGStab failed with residual history: {:?}", self.residual_history)
    }
}

impl std::error::Error for BiCGStabError {
    // 必要なものは全て既に実装されているので、何も書かなくても良い
}
```

これは[thiserror] crateを使うと簡単に定義できます：

```rust
#[derive(Debug, thiserror::Error)]
#[error("BiCGStab failed with residual history: {residual_history:?}")]
struct BiCGStabError {
    residual_history: Vec<f64>,
}
```

`std::error::Error`を実装しておくと `anyhow::Error` に格納することが可能になり、また `downcast_ref` で動的に復元することも可能です：

```rust
#[derive(Debug)]
struct Matrix { /* ... */ };
#[derive(Debug)]
struct Vector { /* ... */ };

#[derive(Debug, thiserror::Error)]
#[error("BiCGStab failed with residual history: {residual_history:?}")]
struct BiCGStabError {
    residual_history: Vec<f64>,
}

fn bicgstab(a: &Matrix, b: &Vector, threshold: f64, max_iteration: usize) -> anyhow::Result<Vector> {
    let mut residual_history = Vec::new();
    for _ in 0..max_iteration {
        // BiCGStab iteration

        let residual = todo!();
        residual_history.push(residual);

        if residual < threshold {
            return Ok(todo!());
        }
    }
    Err(BiCGStabError { residual_history }.into()) // into() で anyhow::Error に変換
}

fn use_bicgstab() {
    match bicgstab(&Matrix { /* ... */ }, &Vector { /* ... */ }, 1e-6, 100) {
        Ok(x) => println!("Solution: {:?}", x),
        Err(e) => {
            // `e` には何が入っているのか分からないので試しに `BiCGStabError` に変換してみる
            if let Some(bicgstab) = e.downcast_ref::<BiCGStabError>() {
                // `BiCGStabError`に変換できたので `residual_history` を表示
                println!("BiCGStab failed with last redisual={:?}", bicgstab.residual_history.last());
            } else {
                // `BiCGStabError` では無い別のエラー型が入っていた
                println!("Unknown Error: {}", e);
            }
        }
    }
}
```

このような型レベルでは[トレイトオブジェクト]を使ってエラー型を消去して、実行時にエラー型を復元する方法を動的管理と呼びます。このように自分でエラー型を定義するとエラーが発生した際の状況を呼び出し元に詳しく伝えることができます。複数のエラーが発生する場合でも個々のケースに対してエラー型を定義し、それらに `Error` traitを実装することで、呼び出し元でそれぞれのエラーを区別できます。

## 発生するエラーを列挙する

動的管理の方法は型情報を一旦消してそれからエラーハンドリング時に再構成しているので、発生しうるエラー全てのケースを網羅するのは大変で、あくまで意図しないエラーが来たときはより上流側に投げなおすことが前提の手法です。この意味でC++やPythonの例外処理と似ています。発生するエラーを列挙したEnumを用意することによってその関数のユーザーは発生するエラーを網羅的に処理することが容易になります。ファイルを読み込んで和を出すプログラムの例で考えてみましょう：

```rust
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("File '{0}' not found")]
    FileNotFound(String),

    #[error("Failed to parse line: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to parse float: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
}

fn read_and_sum(filename: &str) -> Result<f64, Error> {
    let file = File::open(filename).map_err(|_| Error::FileNotFound(filename.to_string()))?;
    let reader = io::BufReader::new(file);
    let mut sum = 0.0;
    for line in reader.lines() {
        let num = line?.parse::<f64>()?;
        sum += num;
    }
    Ok(sum)
}
```

多くのRustのライブラリのコードはこのように発生するエラー型を列挙したEnumを使っています。このようにすることでエラーが発生したときにはそれが何かを知ることができ、またそれに対して適切な処理を行うことができます。`thiserror::Error`マクロはこのようなEnumに対して本領を発揮し、`#[error]`によるエラーメッセージの生成に加え、`#[from]`属性を使って他のエラー型を変換できます。

[BiCGStab]: https://en.wikipedia.org/wiki/Biconjugate_gradient_stabilized_method
[GMRES]: https://en.wikipedia.org/wiki/Generalized_minimal_residual_method
[Newton-Krylov法]: https://en.wikipedia.org/wiki/Newton%E2%80%93Krylov_method
[anyhow]: https://docs.rs/anyhow/1.0.40/anyhow/
[thiserror]: https://docs.rs/thiserror/1.0.40/thiserror/
[line search]: https://en.wikipedia.org/wiki/Line_search
[std::error::Error]: https://doc.rust-lang.org/std/error/trait.Error.html
[trust region]: https://en.wikipedia.org/wiki/Trust_region
[トレイトオブジェクト]: https://doc.rust-jp.rs/book-ja/ch17-02-trait-objects.html
