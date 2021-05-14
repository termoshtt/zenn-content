---
title: "nom の VerboseError を使って失敗した部分を出力する"
emoji: "🦀"
type: "tech"
topics: ["Rust"]
published: true
---

[nom](https://github.com/Geal/nom) はRustのパーサコンビネータライブラリです。この記事の内容はversion 6.1.2で確認しています。

```rust
use nom::{
    character::complete::{digit1, multispace1},
    sequence::tuple,
    Finish, IResult, Parser,
};

/// Parse "1 2" to `(1, 2)`
fn two_digits(input: &str) -> IResult<&str, (u64, u64)> {
    tuple((digit1, multispace1, digit1))
        .map(|(d1, _space, d2): (&str, &str, &str)| (d1.parse().unwrap(), d2.parse().unwrap()))
        .parse(input)
}

fn main() {
    // correct case
    let (_, (a, b)) = two_digits("1 2").finish().unwrap();
    dbg!(a, b);
}
```

典型的にはこの様に使います。[digit1][digit1] の戻り値が `<T as InputTakeAtPosition>::Item: AsChar` の様に少し抽象的な型を指定しているので具体的に `&str` だと注釈しています。

[digit1]: https://docs.rs/nom/6.1.2/nom/character/complete/fn.digit1.html

正常な場合はこれでいいのですがユーザーの入力した文字列が不正の場合、それをユーザーに教える必要があるので、エラーの場合の処理も必要です

```rust
fn main() {
    // wrong case
    let (_, (a, b)) = two_digits("1 a").finish().unwrap();
    dbg!(a, b);
}
```

例えば数値でなくアルファベット `a` が入ってきた場合、`unwrap()` でパニックするので

```
thread 'main' panicked at 'called `Result::unwrap()` on
an `Err` value: Error { input: "a", code: Digit }', src/main.rs:17:50
```

の様にエラーが表示されます。これは1つ目の `digit1` が `1` を食べて、`multispace1` が ` ` を食べた後、`a` を `digi1` が食べようとして失敗していると開発者は分かりますが、ユーザーにはどこで失敗したか分かりません。

これを後から表示するために使えるのが `VerboseError` です。これは追加で `alloc` featureを必要とします。nomは通常高速にパースを実行するためにallocation無しで動作しますが、どこでエラーが起きたかを記録しておくにはそれをどこかに追加で保持する必要があるわけです（多分）

```rust
use nom::{
    character::complete::{digit1, multispace1},
    error::VerboseError,
    sequence::tuple,
    Finish, IResult, Parser,
};

/// Parse "1 2" to `(1, 2)`
fn two_digits(input: &str) -> IResult<&str, (u64, u64), VerboseError<&str> /* set explicitly */> {
    tuple((digit1, multispace1, digit1))
        .map(|(d1, _space, d2): (&str, &str, &str)| (d1.parse().unwrap(), d2.parse().unwrap()))
        .parse(input)
}

fn main() {
    // wrong case
    let input = "1 a"; // second one is not digit
    match two_digits(input).finish() {
        Ok((_, (a, b))) => {
            dbg!(a, b);
        }
        Err(err) => {
            println!("{}", nom::error::convert_error(input, err));
        }
    }
}
```

これは次の様に表示されます。

```
0: at line 1, in Digit:
1 a
  ^
```

鍵になるのは [convert_error][convert_error] で、これはパースエラーの `VerboseError` 構造体と、パースしようとした文字列本体 `input` を必要とします。

[convert_error]: https://docs.rs/nom/6.1.2/nom/error/fn.convert_error.html
