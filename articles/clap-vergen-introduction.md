---
title: "clapでサブコマンドを再利用する"
emoji: "🦀"
type: "tech"
topics: ["cli", "rust", "clap"]
published: true
---

[clap-verbosity-flag](https://github.com/rust-cli/clap-verbosity-flag)というログ出力を変更するフラグ(`-v`や`--debug`)を再利用するcrateが[Rust CLI WG](https://github.com/rust-cli)にあるのを見つけたので、そのアイデアを使ってみようという話です。

https://github.com/rust-cli/clap-verbosity-flag

clap-verbosity-flag crate
--------------------------

ほとんどのCLIアプリケーションはその出力を制御するフラグとして大まかに次のようなオプションを提供します：

- `-q` (`--quiet`): ほとんどの情報を出さない
- `-v` (`--verbose`): なるべく情報を出す
- `--debug`: 可能な限り多くの情報を出す

Rustではライブラリは[log](https://docs.rs/log/) crateを通してユーザーに知らせるべき情報を表示させますが、CLIアプリケーションを書く際はこれをその重要度に応じてフィルターする事が常です。多くのプロジェクトではこのフィルターの設定を上記のフラグによって調整しています。この処理はいつでも同じなので、再利用できるはずです。

```rust
use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Foo
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    log::error!("Engines exploded");
    log::warn!("Engines smoking");
    log::info!("Engines exist");
    log::debug!("Engine temperature is 200 degrees");
    log::trace!("Engine subsection is 300 degrees");
}
```

これによってCLIにオプション引数が追加され、さらにその結果を[`Verbosity::log_level_filter`](https://docs.rs/clap-verbosity-flag/latest/clap_verbosity_flag/struct.Verbosity.html#method.log_level_filter)関数で適切な引数に変換しています。

clap-vergen crate
------------------

CLIアプリケーションに問題がある時、それがいつのコードをどうやってコンパイルしたものかを調べる事が問題の解決への近道です。そのためにはアプリケーションにビルド時の情報を含めておく必要があります。それを行ってくれるのが[vergen](https://github.com/rustyhorde/vergen) crateです：

https://github.com/rustyhorde/vergen

これは`build.rs`中でその時点の時刻やrustcやGitの情報を読み取って[`cargo:rustc-env`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-env)を使って以降のビルドプロセス中の環境変数に書き込みます。アプリケーションはそれを[`env!`](https://doc.rust-lang.org/std/macro.env.html)を使って実行バイナリに埋め込みます。

埋め込まれた情報をCLIアプリケーションに表示させるには他の機能に混ぜ込むより独立したサブコマンドを用意するといいでしょう。この目的の為に`version`サブコマンドを作るのが[clap-vergen](https://github.com/termoshtt/clap-vergen)の目的です。

clapの[Derive API](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)を使って次のように書きます：

```rust
use clap::Parser;
use clap_vergen::Version;

#[derive(Debug, clap::Parser)]
enum Cli {
    Version(Version),
}

fn main() {
    match Cli::from_args() {
        Cli::Version(version) => {
            version.print().unwrap();
        }
    }
}
```

`Cli` enumに他のサブコマンドを作る想定です。これで`version`サブコマンドが作られます：

```
$ ./target/debug/main version --help
main-version
Output detail version of executable

USAGE:
    main version [OPTIONS]

OPTIONS:
    -h, --help    Print help information
        --json    Output version info as JSON
```

現在は標準の出力方法とJSON出力がサポートされています：

```
$ ./target/debug/main version
Build Timestamp:     2022-08-06T08:16:05.843030928Z
Build Version:       0.1.0
Commit SHA:          f1af7e4b9fc58b7aa73b1e14a617d9a341a9880d
Commit Date:         2022-08-06T08:12:22Z
Commit Branch:       main
rustc Version:       1.63.0-beta.8
rustc Channel:       beta
rustc Host Triple:   x86_64-unknown-linux-gnu
rustc Commit SHA:    7410ebb8f69516d0034cc99793bc3dcbc84d4a9b
cargo Target Triple: x86_64-unknown-linux-gnu
cargo Profile:       debug
```

```
$ ./target/debug/main version --json
{
  "build_timestamp": "2022-08-06T08:16:05.843030928Z",
  "build_semver": "0.1.0",
  "rustc_channel": "beta",
  "rustc_commit_date": "2022-08-04",
  "rustc_commit_hash": "7410ebb8f69516d0034cc99793bc3dcbc84d4a9b",
  "rustc_host_triple": "x86_64-unknown-linux-gnu",
  "rustc_llvm_version": "14.0",
  "rustc_semver": "1.63.0-beta.8",
  "cargo_features": "default",
  "cargo_profile": "debug",
  "cargo_target_triple": "x86_64-unknown-linux-gnu",
  "git_branch": "main",
  "git_commit_timestamp": "2022-08-06T08:12:22Z",
  "git_semver": "0.1.0",
  "git_sha": "f1af7e4b9fc58b7aa73b1e14a617d9a341a9880d"
}
```

このようにアプリケーション側のコードでは`clap_vergen::Version`の定義を全く見ること無く、CLIオプションの生成とそれを受け取った後の処理まで実装できます。
