---
title: "Apptainer導入"
emoji: "📦"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["singularity", "apptainer", "container"]
published: false
---

## Linux Foundationへの移管とApptainerへの改名

HPC向けコンテナ環境としてよく使われているSingularityプロジェクトがLinux Foundationに移管されるに伴って名称を変更したものがApptainerです。

https://apptainer.org/news/community-announcement-20211130/
https://www.linuxfoundation.org/press-release/new-linux-foundation-project-accelerates-collaboration-on-container-systems-between-enterprise-and-high-performance-computing-environments/

Singularityというプロジェクトはやや込み入った状況があります。例えば理化学研究所のスーパーコンピュータ富岳で採用されているコンテナ環境はSingularityPROと呼ばれているものですが、これは元々Sylabsという会社が上記のプロジェクトをフォークしたもので、現在では実質的に独立して開発が続けられています。
https://www.hpcwire.com/off-the-wire/rikens-fugaku-utilizes-sylabs-singularitypro/

コンテナ技術は登場から時間が経つにつれて多くの実装や団体が設立されています。中でも重要となるのが

- Cloud Native Computing Foundation (CNCF)
- Open Container Initiative (OCI)

の二つでしょう。

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
