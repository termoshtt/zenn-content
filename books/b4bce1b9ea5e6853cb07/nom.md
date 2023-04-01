---
title: データや設定ファイルをパースする
---
数値計算ユーザーは時として独自形式の設定・データフォーマットをパースする必要があります。ここでは`nom`というパーサコンビネータライブラリを使って複雑なフォーマットのパーサーを書いてみましょう。

https://github.com/Geal/nom

`FromStr` trait
----------------
`nom`の説明に入る前にまず標準ライブラリの基本的なパーサーから見ていきましょう。Rustの標準ライブラリには[std::str::FromStr](https://doc.rust-lang.org/stable/std/str/trait.FromStr.html)というTraitがあり、これを実装している型は

```rust
use std::str::FromStr;

let s = "5";
let x = i32::from_str(s).unwrap();

assert_eq!(5, x);
```

のように文字列からその型に変換することができます。これは上の例の`i32`のように組み込み型には一通り実装されており、またユーザーが自由に実装することが出来ます。また文字列側にも[str::parse](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse)という関数が生えているので

```rust
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

のようにすることも出来ます。

ユーザー定義の構造体に対して`FromStr`を実装する際、単純なフォーマットであれば自分で文字列操作を書いたり正規表現を使ったり出来ますが、複雑なフォーマットになると難しいです。そこで複数の小さなパーサーを組み上げて複雑なフォーマットをパースできるようにするパーサコンビネータが必要になります。

パーサコンビネータ
-------------------
多くのテキストフォーマットは複数のコンポーネントからなっています。例えば現在でもよく使われるメッシュデータフォーマットVTK Legacyでは次の様にヘッダーがあり、いくつかのブロックが続くような構造になっていきます：

```text
# vtk DataFile Version 1.0
Unstructured Grid Example
ASCII

DATASET UNSTRUCTURED_GRID
POINTS 27 float
0.0 0.0 0.0
1.0 0.0 0.0
2.0 0.0 0.0
0.0 1.0 0.0
...
```

この`DATASET`には非構造データ`UNSTRUCTURED_GRID`の他にも構造化データ`STRUCTURED_GRID`やポリゴンデータ`POLYDATA`が来る事もできます。これをヘッダーのパーサーとそれぞれの`DATASET`のパーサーを書いて、それを組み合わせる事で実装できるようにするのがパーサコンビネータです。

整数を`u64`としてパースするパーサは次の様に書きます。エラー処理が少し複雑ですが、文字列から連続した数値を抜き出して整数型に変換します：

```rust
use nom::{Finish, character::complete::digit1};
use num_traits::Unsigned;
use std::str::FromStr;

pub fn uint<I: Unsigned + FromStr>(input: &str) -> Result<I> {
    // 引数の最初から一つ以上の数値を探してくる
    // 見つからなかったら?演算子によってこの段階でパースを諦めてエラーを返す
    let (residual, digits) = digit1(input)?;

    // 見つけた数値の列を整数値として変換する
    let num: I = digits
        .parse()
        .map_err(
          // 桁が多すぎる等、数値の列がIとして変換できない場合
          |_| failure(input, "unsigned integer")
        )?;

    // 使わなかった分の文字列と一緒に整数値を返す
    Ok((residual, num))
}

// エラー型
use nom::error::{VerboseError, VerboseErrorKind};

// 復帰可能なエラーと復帰不可能なエラーの両方を表現出来るエラー型が用意されているのでそれを使う
pub type Result<'input, T> = nom::IResult<&'input str, T, VerboseError<&'input str>>;

// 復帰不可能なエラーを生成する
fn failure<'input>(input: &'input str, msg: &'static str) -> nom::Err<VerboseError<&'input str>> {
    nom::Err::Failure(VerboseError {
      errors: vec![(input, VerboseErrorKind::Context(msg))],
    })
}

// テスト
assert_eq!(uint::<u64>("1234").finish().unwrap(), ("", 1234));
assert!(uint::<u64>("abcd").finish().is_err());
```

パーサコンビネータでは個別のパーサのレベルで入力文字列にマッチしなくてもより上の所で復帰出来る事があります。フォーマットの仕様によっては文字列が来ても整数が来てもいい場所というのはよくあります。例えば上のVTK Legacyでは`DATASET`として複数の複雑なコンポーネントが複数回続く可能性があります。このような場合一旦整数としてパースしてみて違うなら文字列として受け入れる、という処理を行います。その為にエラー型`nom::Err`にはコンビネータ内で復帰可能な`Err::Error`と復帰不可能な`Err::Failure`があります。例えば`u32`整数としてパースしないといけないけど`u32`では収まらない整数が来た場合は復帰不可能なエラー`Err::Failure`を返します。そして`nom::Finish`によってこれらを通常の`Result`に変換します。

いくつか基本的な応用例をあげていきましょう。

- `# vtk DataFile Version x.x` という文字列をパースしてバージョンの整数を取得する
  ```rust
  use nom::{
    bytes::complete::tag,
    character::complete::{char, space0, space1},
    sequence::tuple,
    Parser
  };
  use rust_math_book_test::nom::{Result, uint};

  pub fn head_line(input: &str) -> Result<(u64, u64)> {
      // 複数の要素に連続してマッチしたい場合は`tuple`を使う
      let (input, _) = tuple((
          char('#'),   // `#`一文字
          space0,      // 0個以上のスペース
          tag("vtk"),  // 決まった文字列
          space1,      // 1個以上のスペース
          tag("DataFile"),
          space1,
          tag("Version"),
          space1,
      ))
      .parse(input)?; // ここで一括してパースして、失敗したら一番最初に戻る

      tuple((uint, char('.'), uint))
          .map(|(major, _dot /* `.`は要らないので捨てる */, minor)|
            (major, minor)
          )
          .parse(input)
  }
  ```

- `ASCII`か`BINARY`のどちらかの文字列にマッチさせる
  ```rust
  use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, space0},
    sequence::tuple,
    Parser
  };
  use rust_math_book_test::nom::Result;

  // enumとしてどっちにマッチしたのかを返す
  #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum Format {
      ASCII,
      BINARY,
  }
  pub fn format(input: &str) -> Result<Format> {
      // 先頭のスペースは捨てる
      let (input, _) = space0(input)?;
      // タプルで指定されたどれかにマッチさせる
      // 前から順番にマッチさせていき成功した段階で返す
      alt((
          // 文字列にマッチした場合はenumに変換する
          tag("ASCII").map(|_| Format::ASCII),
          tag("BINARY").map(|_| Format::BINARY),
      ))
      .parse(input)
  }
  ```

Link
-----

`nom`は数値計算用でなく汎用なパーサコンビネータなのでユーザーも多く、チュートリアルがいくつか存在しています：

- [benkay86/nom-tutorial - GitHub](https://github.com/benkay86/nom-tutorial/)
- [Rust: nom によるパーサー実装](https://hazm.at/mox/lang/rust/nom/index.html)

また`nom`にはバイナリデータや、あるいは全部のデータが届いてない状態でパースを行うStreaming向けのAPIも存在します。詳しくは本家のドキュメントを確認してください。
https://docs.rs/nom/latest/nom/index.html
