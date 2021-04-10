---
title: "Container registry of GitHub Package を使ってみる"
emoji: "🗂"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["docker", "GitHub"]
published: true
---

ドキュメントが見当たらなかったのでメモ程度に。最小構成はこちら:
https://github.com/termoshtt/github-docker-package-example

GitHub Package と GitHub container registry
----------------------------------------------

現在(2021/4/10) GitHub には二つの Container registry service が存在しています。それぞれ GitHub Package (docker.pkg.github.com) と GitHub container registry (ghcr.io) という名前になっており、仕様が少しずつ異なります。

https://docs.github.com/en/packages/guides/about-github-container-registry

|        | docker.pkg.github.com   | ghcr.io                                 |
|:-------|:-----------------------:|:---------------------------------------:|
|認証方法| `GITHUB_TOKEN` のみ     | `GITHUB_TOKEN` と Personal Access Token |
|名前空間| Project 毎              | User / Organization 毎                  |
|権限    | Repository と同一       | 個別に設定                              |
|アクセス| GitHub Actions からのみ | Public なら誰でも読み込み可能           |

この記事では ghcr.io ではなく docker.pkg.github.com を扱います

上の表に書いたとおり docker.pkg.github.com では `GITHUB_TOKEN` しかサポートされません。`GITHUB_TOKEN` とは GitHub Actions 中のみで有効なトークンで、つまり docker.pkg.github.com は実質 GitHub Actions からしか使えません。ただし GitHub の別プロジェクトの GitHub Actions からは GitHub 上での権限があれば読み込めるわけです。
用途としては知らない第三者に配布するのではなく、主に Private Repository 等で GitHub 上で管理されている権限を使ってコンテナのアクセスも管理したいという目的での使用を想定しているのでしょう。

GitHub Actions の設定
----------------------

`hello-world:latest` を取得して `docker.pkg.github.com/org-name/repository-name/hello-world:latest` として公開するには次の様に設定します

```yaml
name: Docker

on:
  push:
    branches:
      - main

jobs:
  push:
    runs-on: ubuntu-20.04
    steps:
    - uses: actions/checkout@v1
    - name: build and push docker image
      run: |
        echo "${{ github.token }}" | docker login https://docker.pkg.github.com -u ${{ github.actor }} --password-stdin
        docker pull hello-world:latest
        docker tag hello-world:latest docker.pkg.github.com/${{ github.repository }}/hello-world:latest
        docker push docker.pkg.github.com/${{ github.repository }}/hello-world:latest
```

アクセストークンは `${{ github.token }}` で、ユーザー名は `${{ github.actor }}` を使います。あとは通常どおり `docker tag` で別名をつけて `docker push` で送信します。
