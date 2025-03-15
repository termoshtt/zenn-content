---
title: "素手でコンテナを作る"
emoji: "📚"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["container", "docker", "podman", "tar"]
published: false
---

この記事では静的リンクされた実行可能ファイルを `tar` と手書きのJSONでコンテナイメージにパッケージし、`podoman` で実行する事でコンテナイメージの仕様を理解します。コンテナの実行する必要があるのこの記事ではLinux環境を前提とします。また筆者の環境の都合で `x86_64` の話をします。

# 静的リンクされた実行ファイルを用意する

どう作ってもいいですが今回はRustで作りましょう。`x86_64-unknown-linux-musl` ターゲットでビルドすると静的リンクされた実行ファイルができます。

```shell
cargo new --bin hello
cd hello
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

静的リンクれているかは `ldd` コマンドで確認できます。

```text
$ ldd target/x86_64-unknown-linux-musl/release/hello
        statically linked
```

この記事ではこれをコンテナイメージにパッケージします。