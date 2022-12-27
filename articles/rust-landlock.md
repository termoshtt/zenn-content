---
title: "Landlock: ユーザ権限によるアクセス制御"
emoji: "📦"
type: "tech"
topics: ["rust", "linux", "container", "landlock"]
published: true
---

LandlockはLinux 5.13で追加され、5.19で更新(ABI V2)されたプロセス単位のアクセス制御機構です。
https://landlock.io/

この機能は主に自分自身の権限を制限してサンドボックスを作るために使います。例えばこの記事の後半では信頼できない実行バイナリをサブプロセスとして起動する際にアクセス出来るファイルシステムの範囲を制限する例を見ます。

rust-landlock/examples
-----------------------
今回はLandlockをRustから使えるようにした[rust-landlock](https://github.com/landlock-lsm/rust-landlock)を試していきます。ドキュメントは[landlock.io/rust-landlock](https://landlock.io/rust-landlock/landlock/)に公開されています。
https://github.com/landlock-lsm/rust-landlock

特に[Cによるサンプル](https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/samples/landlock/sandboxer.c)を[Rustで書き直したサンプル](https://github.com/landlock-lsm/rust-landlock/tree/main/examples)を見ていきましょう。とりあえず実行してみるとヘルプを出してくれます：

```shell
cargo run --example sandboxer
```

```
usage: LL_FS_RO="..." LL_FS_RW="..." target/debug/examples/sandboxer <cmd> [args]...

Launch a command in a restricted environment.

Environment variables containing paths, each separated by a colon:
* LL_FS_RO: list of paths allowed to be used in a read-only way.
* LL_FS_RW: list of paths allowed to be used in a read-write way.

example:
LL_FS_RO="/bin:/lib:/usr:/proc:/etc:/dev/urandom" LL_FS_RW="/dev/null:/dev/full:/dev/zero:/dev/pts:/tmp" target/debug/examples/sandboxer bash -i
```

2つの環境変数`LL_FS_RO`と`LL_FS_RW`にそれぞれ読み込み専用にしたいパスと読み書きできるパスを指定して、引数に起動するプログラムを指定します。例を示してくれているので起動してみましょう：

```shell
LL_FS_RO="/bin:/lib:/usr:/proc:/etc:/dev/urandom" \
LL_FS_RW="/dev/null:/dev/full:/dev/zero:/dev/pts:/tmp" \
target/debug/examples/sandboxer bash -i
```

すると`bash`が開始されます。とりあえず`ls`を実行してみましょう：

```
[myname@mymachine rust-landlock]$ ls
ls: cannot open directory '.': Permission denied
```

現在のディレクトリが開けません。サンドボックスっぽいですね。では上で指定したディレクトリに行ってみましょう：

```
[myname@mymachine rust-landlock]$ cd /etc/
[myname@mymachine etc]$ ls | head -5
adjtime
alsa
alternatives
anacrontab
anthy-conf
```

読み込めますね。書き込めるかも見ておきましょう：

```
[myname@mymachine etc]$ echo "homhom" /tmp/homhom
homhom /tmp/homhom
[myname@mymachine etc]$ echo "homhom" > /tmp/homhom
[myname@mymachine etc]$ cat /tmp/homhom
homhom
```

上手く動いていますね。

sandboxer.rs
-------------
ではソースコードを見ていきましょう。前半に色々書いていますが、肝心なのは`main`の後半です：

```rust
    let abi = ABI::V2;

    // アクセス制御の為のルールを作る
    let status = Ruleset::new()
        .handle_access(AccessFs::from_all(abi))?
        .create()?
        // Read-onlyのパスの追加
        .add_rules(PathEnv::new(ENV_FS_RO_NAME, AccessFs::from_read(abi))?.iter())?
        // Read-Writeのパスの追加
        .add_rules(PathEnv::new(ENV_FS_RW_NAME, AccessFs::from_all(abi))?.iter())?
        .restrict_self()
        .expect("Failed to enforce ruleset");

    // Landlockをサポートしていないカーネルで動かした場合、制限に失敗する
    if status.ruleset == RulesetStatus::NotEnforced {
        bail!("Landlock is not supported by the running kernel.");
    }

    // サブプロセスとして引数で受け取ったプログラム(上の例だと`bash -i`)を起動する
    Err(Command::new(cmd_name)
        .env_remove(ENV_FS_RO_NAME)
        .env_remove(ENV_FS_RW_NAME)
        .args(args)
        .exec()
        .into())
```

コメントは私が追加しています。処理は単純で、`handle_access`と[`add_rules`](https://landlock.io/rust-landlock/landlock/trait.RulesetCreatedAttr.html#method.add_rules)によってルールセットを作り、[`restrict_self`](https://landlock.io/rust-landlock/landlock/struct.RulesetCreated.html#method.restrict_self)によって現在のプロセスにルールを適用し、サブプロセスを起動しています。サブプロセスには自動的に制約が継承されます。所々`abi`を引数にもらっているのは互換性の為です。

### `PathFd`, `O_PATH` in open(2)
処理は名前から期待される通りでドキュメントを順番に見れば詳細は分かりますが、1つ気になる点として[`PathFd`](https://landlock.io/rust-landlock/landlock/struct.PathFd.html)というのが出てきます。[`add_rules`](https://landlock.io/rust-landlock/landlock/trait.RulesetCreatedAttr.html#method.add_rules)にはパスとアクセス権の組を表す構造体である[`PathBeneath`](https://landlock.io/rust-landlock/landlock/struct.PathBeneath.html)を渡しますが、これはパスの表現としてファイル記述子を使います。これは[`std::os::unix::io::AsRawFd`](https://doc.rust-lang.org/1.65.0/std/os/unix/io/trait.AsRawFd.html)を使ってファイル記述子をもらうAPIになっており、例えば`std::fs::File`もこれは実装していますが、ここで代わりに`PathFd`を使うようになっています。これは何故かというとLandlockに渡すファイル記述子はファイルシステム上でのファイルの位置さえ分かればよく、ユーザーが実際にこのファイルを開く権限が無くても使えるようになっています。そのような用途のために`O_PATH`というオプションが`open`システムコールには存在し、`PathFd`はそれを使うようになっています。
