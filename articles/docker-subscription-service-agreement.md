---
title: "Docker Subscription Service Agreement"
emoji: "🐳"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["Docker"]
published: true
---

Docker Desktop 4.0のリリースと共に新たなサービス利用規約、[Docker Subscription Service Agreement](https://www.docker.com/legal/docker-subscription-service-agreement)(以下DSSAと略す)が発表された。

## 前提知識
まず利用規約の前にいくつか基本的な事をおさらいしておこう：

- Dockerは[Docker, Inc.](https://www.docker.com/company)が開発し配布しているソフトウェア
  - なので基本的にはプロプライエタリソフトウェア
- その一部はOSSライセンス(Apache License 2.0)で配布されている
  - この部分は[moby](https://github.com/moby/moby)とも呼ばれる
  - [MicrosoftのVisual Studio Codeと同様に](https://github.com/microsoft/vscode/wiki/Differences-between-the-repository-and-Visual-Studio-Code)コア部分をOSSとして提供する一方で、それをパッケージ化して配布及びマネージドサービスとして提供する部分はプロプライエタリな形を維持する方式
- コンテナランタイム、つまり仮想化とそれを操作するAPI群はOS（Windows, Linux）が提供しており、Dockerはこれらを使ってコンテナ仮想化を実現している
  - Linuxではcgroupsを使ったプロセスレベルのリソースの名前空間管理によって実現している
  - WindowsではHyper-Vを使った[Hyper-V container](https://docs.microsoft.com/ja-jp/virtualization/windowscontainers/manage-containers/hyperv-container)とLinuxと同様の[プロセスレベルの仮想化](https://docs.microsoft.com/ja-jp/virtualization/windowscontainers/about/)がOSで提供されており、Dockerはそれぞれを使用できる
  - 他のコンテナランタイム、例えば[RedHatが主導するpodman](https://access.redhat.com/documentation/ja-jp/red_hat_enterprise_linux/8/html-single/building_running_and_managing_containers/index)も同様にOSの機能を使用している
- コンテナを配布するサーバーはRegistryと呼ばれる
  - このサーバーの実装として最も普及しているのが[distribution](https://github.com/distribution/distribution)と呼ばれる実装であり、[Docker Registry](https://docs.docker.com/registry/)とはこの実装の事を指す（昔は[独自Python実装](https://github.com/docker-archive/docker-registry)があったらしい）
  - Registryのマネージドサービスとして存在しているのが[DockerHub](https://hub.docker.com/)や[GitHub Container Registry (ghcr.io)](https://docs.github.com/ja/packages/working-with-a-github-packages-registry/working-with-the-container-registry)

## Docker, Inc. の提供するソフトウェア・サービス

### Docker Engine
https://docs.docker.com/engine/
これはDSSA必要なくmobyの一部としてApache License version 2.0で引き続き利用できる。

Linuxユーザーにとって最もなじみのあるであろう`docker`コマンド、及び`docker`コマンドと通信しコンテナの面倒を見てくれるデーモン`dockerd`はこのDocker Engineの一部である。ちなみにDockerのRootless化というのは`dockerd`がroot以外の権限で実行できるようになったことを指す。`docker`/`dockerd`のクライアント+サーバー構成の為、`docker`コマンドはそのホスト以外で動作している`dockerd`に対してネットワーク経由で命令を発行できる。これにより例えばWindowsホスト上からHyper-V内のLinuxで動作している`dockerd`に命令を発行するなどが可能になる。

### Docker Compose (v1)
https://docs.docker.com/compose/
これもDSSA必要なく[docker/compose](https://github.com/docker/compose)で開発されApache License version 2.0で引き続き利用できる。

`docker-compose`コマンドは複数のコンテナと仮想ネットワークを構築し、コンテナオーケストレーションを実現するためのツールである。元々3rdパーティ製のPythonスクリプトであったものがDockerの管理下に入ったもので、これだけ設定ファイルや使い勝手が大きく異なったのはそのためである。なおGoで書き直された[docker compose v2](https://docs.docker.com/compose/cli-command/)がDocker Engineに含まれているのでこれ(v1)は時機に役目を終えるはず。

### Docker Desktop
https://docs.docker.com/desktop/
> Docker Desktop is an easy-to-install application for your Mac or Windows environment that enables you to build and share containerized applications and microservices. Docker Desktop includes Docker Engine, Docker CLI client, Docker Compose, Docker Content Trust, Kubernetes, and Credential Helper.

これはDSSAが必須になる、Windows/macOS向けの全部入り一括配布パッケージ。Docker Engineに加えて[Kubernetes](https://kubernetes.io/ja/docs/home/)もセットアップしてくれる。Hyper-Vバックエンド・WSL2バックエンドのLinuxコンテナ環境の構築に加えWindowsコンテナも使えるようになるため、現状Windows/macOSではDockerをセットアップする最も適切な手段だと言える。自分でDocker Engineをインストールしたりせずにおとなしく(場合によっては課金して)これを使うのがほとんどの人にとって有益。例えばHyper-V上のLinuxにDocker Engineをインストールし、Windows側でDocker CLIを自分でセットアップしてHyper-Vの仮想ネットワーク経由で`dockerd`を叩いて使うならDocker Desktopは使わずに済むはずだが明らかに労力に合わない。

### Docker Hub
https://docs.docker.com/docker-hub/
Docker Registryの公式マネージドサービス。Publicなイメージをpullするだけなら認証する必要はないが、pushしたりPrivateなリポジトリにpush/pullする場合にはログインが必要で、この際にDSSAを承認することが必要。
