---
title: "ocipkg: OCI Registry for package distribution"
emoji: "📦"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust", "docker", "container"]
published: true
---

ocipkg 0.1.0をリリースしました

https://github.com/termoshtt/ocipkg/releases/tag/v0.1.0

この記事ではこのプロジェクトを作り始めたモチベーションについて説明しようと思います。

OCI Registry
-------------

OCI Registryとは標準化されたコンテナレジストリ、つまりコンテナをPushしたりPullするREST APIが定められており(OCI distribution spec)、これに則って実装されたコンテナレジストリの事です。例えばGitHub Container Registry (ghcr.io)やDockerHub (docker.io)があります。

ocipkg
-------

ocipkgはOCI Registryのクライアントとして動作するように設計されています：

- OCI RegistryへのコンテナのPushとPull
- ローカルに保存されたoci-archive形式のコンテナの読み書き

が実装されています。これらに加えて、パッケージ管理機構の一部としてOCI Registryを使うためのユーティティとして：

- ファイルやディレクトリ、あるいはRustプロジェクトからのコンテナの作成
- `build.rs`ヘルパーによるコンテナの取得とその中のライブラリのリンク

が可能です。これらは全てDocker等の外部のコンテナランタイムを使うこと無く独立に実装されています。

FFIするライブラリをどうやって手に入れるか
------------------------------------------
TBW
