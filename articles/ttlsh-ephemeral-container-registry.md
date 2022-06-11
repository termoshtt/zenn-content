---
title: "ttl.sh: 高速で一時的なコンテナレジストリ"
emoji: "📦"
type: "tech"
topics: ["ttlsh", "container", "docker"]
published: true
---

[sigstore](https://docs.sigstore.dev/)のドキュメントを調べてる際に言及されていたのでついでにメモしておきます。

[ttl.sh](https://ttl.sh/)は高速で一時的なコンテナレジストリです。

```shell
echo "FROM ubuntu:22.04" > Dockerfile
IMAGE_NAME=$(uuidgen)
docker build -t ttl.sh/${IMAGE_NAME}:1h .
docker push ttl.sh/${IMAGE_NAME}:1h
```

認証無しで誰でも無料で使えます。タグで時間制限をつけることができ、デフォルトで1時間、最大で24時間維持されます。さらにCloudflareの機能を使うことによってpullが非常に高速だとの事です。

このサービスを提供している[Replicated](https://www.replicated.com/)ではワークフロー間のアーティファクトの共有に利用しているようです。

補足
-----

uuidgenはUUIDを作ってくれるコマンドです。

```
$ uuidgen
17906427-c75e-48eb-8462-4a75ab42aa6a
```

Ubuntuでは[uuid-runtime](https://packages.ubuntu.com/jammy/uuid-runtime)パッケージに、ArchLinuxの場合には[core/util-linux](https://archlinux.org/packages/core/x86_64/util-linux/)に含まれているのでおそらく既に存在しています。
