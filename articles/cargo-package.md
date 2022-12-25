---
title: "cargo package"
emoji: "📦"
type: "tech"
topics: ["rust", "cargo"]
published: true
---

あまり直接使わない`cargo`のコマンドに`cargo-package`というのがあります：

https://doc.rust-lang.org/cargo/reference/publishing.html

`cargo-publish`かさらにそれをラップした[cargo-release](https://github.com/crate-ci/cargo-release)を使うことが多いと思います。また`cargo publish --dry-run`と`cargo package`が同じ動作になります。

`.crate`ファイル
----------------
`cargo-package`コマンドは実行すると`target/release`以下に`{crate name}-{version}.crate`なる命名規則のファイルを作ります。少なくともLinuxではこれは`tar.gz`ファイルなので、`tar`コマンドで中身を見ることが出来ます：

```
$ tar tf ocipkg-0.2.8.crate | head
ocipkg-0.2.8/.cargo_vcs_info.json
ocipkg-0.2.8/Cargo.toml
ocipkg-0.2.8/Cargo.toml.orig
ocipkg-0.2.8/README.md
ocipkg-0.2.8/src/digest.rs
ocipkg-0.2.8/src/distribution/auth.rs
ocipkg-0.2.8/src/distribution/client.rs
ocipkg-0.2.8/src/distribution/mod.rs
ocipkg-0.2.8/src/distribution/name.rs
ocipkg-0.2.8/src/distribution/reference.rs
```

`.crate`に入るファイルは`Cargo.toml`で調整出来て、例えば特定のファイルを除きたい場合には次のように書きます：

```toml
[package]
# ...
exclude = [
    "public/assets/*",
    "videos/*",
]
```

このファイルのリストを取得する為に、`cargo pacakge --list`というコマンドも用意されています。

`cargo-package`コマンドはこの`.crate`ファイルを作った後、このアーカイブを別の場所に展開してちゃんとビルドできるかを検証してくれます。`--no-verify`を指定すると作るだけになります。`cargo-publish`ではこのファイルを[crates.io](https://crates.io/)、あるいは別のレジストリにアップロードします。この`.crate`ファイルのサイズ上限が`crates.io`では10MBとなります。
