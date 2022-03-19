---
title: "Apptainer導入"
emoji: "📦"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["singularity", "apptainer", "container"]
published: false
---

## Linux Foundationへの移管とApptainerへの改名

HPC向けコンテナ環境としてよく使われている[Singularity][Singularity]プロジェクトが[Linux Foundation][LF]に移管されるに伴って名称を変更したものが[Apptainer][Apptainer]です。

https://apptainer.org/news/community-announcement-20211130/
https://www.linuxfoundation.org/press-release/new-linux-foundation-project-accelerates-collaboration-on-container-systems-between-enterprise-and-high-performance-computing-environments/

Singularityというプロジェクトはやや込み入った状況があります。例えば理化学研究所のスーパーコンピュータ富岳で採用されているコンテナ環境はSingularityPROと呼ばれているものですが、これは元々Sylabsという会社が上記のプロジェクトをフォークしたもので、現在では実質的に独立して開発が続けられています。
https://www.hpcwire.com/off-the-wire/rikens-fugaku-utilizes-sylabs-singularitypro/

コンテナ技術は登場から時間が経つにつれて多くの実装や周辺ツールが整備され、またいくつか重要な団体が設立されています。中でも重要となるのが

- [Cloud Native Computing Foundation (CNCF)](https://www.cncf.io/)
- [Open Container Initiative (OCI)](https://opencontainers.org/)

の二つでしょう。CNCFはLinux Foundationの一部で、Kubernetesやcontainerdといったコンテナ技術やenvoyやlinkerdといったネットワーク技術を支えるプロジェクトを支援しています。一方OCIもLinux Foundationのプロジェクトの一つですが、こちらはコンテナ技術の標準化を目標に掲げるプロジェクトです。OCIによるコンテナの実行環境・コンテナイメージ・配布レジストリの標準化のおかげで現在では複数の異なるコンテナ実行環境、例えば[Docker](https://www.docker.com/)や[Podman](https://github.com/containers/podman)といった異なる実行環境で同じコンテナを同じように使うことができます。

残念ながらSingularityプロジェクトはこれらのモダンなコンテナ技術と上手く統合出来ているとは言い難い状況です。今後のApptainerの開発においてはこれらの利用、特にコンテナの署名のための[Sigstore][Sigstore]、コンテナレジストリをストレージとして扱う技術である[ORAS][ORAS]、及び自動テストを支えるCIと自動デプロイを支えるCDとの連携に注力すると述べられています。

[LF]: https://www.linuxfoundation.org/
[Singularity]: https://github.com/apptainer/singularity
[Apptainer]: https://github.com/apptainer/apptainer
[Sigstore]: https://www.sigstore.dev/
[ORAS]: https://oras.land/

## Apptainerのインストール

この記事ではArchLinuxで行った場合について書く

```
pacman -S apptainer
```

https://archlinux.org/packages/community/x86_64/apptainer/

`community/apptainer`として登録されている。既に`singularity-container`を入れている場合は置き換わるので注意する。

## Singularityからの移行

既にSingularityを使っていた場合、設定ファイル等を更新する必要がある

https://apptainer.org/docs/admin/main/singularity_migration.html
