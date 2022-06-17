---
title: "sigstoreでコンテナに署名する(with key)"
emoji: "📦"
type: "tech"
topics: ["sigstore", "cosign", "container", "docker"]
published: true
---

要約
-----

- cosignというsigstoreの提供するツールを使ってコンテナに署名して同じレジストリに保存する事が出来る
- sigstoreではkeylessで署名を行う為の仕組みを構築しているが、cosignにはkeyを管理する方法もあるのでここではこれを説明する

cosignのインストール
---------------------
環境に応じていくつかインストール方法があります
https://docs.sigstore.dev/cosign/installation

- 自分でビルド
  ```shell
  go install github.com/sigstore/cosign/cmd/cosign@latest
  ```
- Binaryを取得 (Release 1.6.0)
  ```shell
  wget "https://github.com/sigstore/cosign/releases/download/v1.6.0/cosign-linux-amd64"
  mv cosign-linux-amd64 /usr/local/bin/cosign
  chmod +x /usr/local/bin/cosign
  ```
- RPM (Release 1.6.0)
  ```shell
  wget "https://github.com/sigstore/cosign/releases/download/v1.6.0/cosign-1.6.0.x86_64.rpm"
  rpm -ivh cosign-1.6.0.x86_64.rpm
  ```
- DEB (Release 1.6.0)
  ```shell
  wget "https://github.com/sigstore/cosign/releases/download/v1.6.0/cosign_1.6.0_amd64.deb"
  dpkg -i "cosign_1.6.0_amd64.deb
  ```
- Arch Linux
  ```shell
  pacman -S cosign
  ```
- Alpine Linux
  ```shell
  apk add cosign sget
  ```

なおYubiKey等のハードウェアトークンを使う機能はデフォルトで有効になっていないため[自分でビルドする必要があります](https://github.com/sigstore/cosign/blob/main/TOKENS.md)。

```shell
go build -tags=pivkey,pkcs11key ./cmd/cosign
```

鍵の生成
---------
sigstoreではkeyless署名を実現するために色々な試みが行われていますが、鍵を管理する方式がデフォルトです。

```shell
cosign generate-key-pair
```

で秘密鍵のパスワードを入力した後、現在のディレクトリに`cosign.key`(秘密鍵)と`cosign.pub`(公開鍵)を生成されます。

また手元に秘密鍵をおかずにGitLab CIの変数として保存しておくことも出来ます:

```shell
export GITLAB_TOKEN=glpat-xxxxxxxxxxxxxx  # apiの権限が必要
cosign generate-key-pair gitlab://termoshtt/sigstore-testing
```

![Generated variables in GitLab CI](/images/cosign-generate-key-gitlab.png)
既に変数が存在しているとエラーになります(上書きはされない)。

署名
-----

まず署名に使うコンテナを用意します。[前回](https://zenn.dev/termoshtt/articles/ttlsh-ephemeral-container-registry)で説明した[ttl.sh](https://ttl.sh)を使います:

```shell
echo "FROM alpine" > Dockerfile
IMAGE_NAME=$(uuidgen)
docker build -t ttl.sh/${IMAGE_NAME}:1h .
docker push ttl.sh/${IMAGE_NAME}:1h
```

このコンテナに署名するには秘密鍵として`cosign.key`ファイルを使う場合は:

```shell
cosign sign --key cosign.key ttl.sh/${IMAGE_NAME}:1h
```

GitLabに保存した鍵を使う場合は`--key`に`generate-key-pair`と同じように`gitlab://<user>/<repo>`を指定します:

```shell
cosign sign --key gitlab://termoshtt/sigstore-testing ttl.sh/${IMAGE_NAME}:1h
```

GitHubでも同じ様にGitHub Actionsの変数として生成することは出来ますが、GitHubにはActionsの変数を取得するAPIが存在していないためActions以外の環境ではその鍵を使って署名出来ません。

`--key`を省略すると`COSIGN_PRIVATE_KEY`環境変数から鍵を読み取り、`COSIGN_PASSWD`環境変数からパスワードを読み込みます。GitHub/GitLab上に鍵を作成した場合は自動的にこの変数に保存されるので指定する必要はありません。

`cosign sign`は署名した内容を同じレジストリの別タグに保存します。タグはコンテナイメージのdigestを使って決めます。digestはImage IDとは別に振られるハッシュ値なので注意です。
https://docs.docker.jp/engine/reference/commandline/images.html#digest

```
cosign triangulate ttl.sh/${IMAGE_NAME}:1h
```

で書き込まれるコンテナのイメージ名(タグ込み)が表示されます。triangulate(三角測量)なのはコンテナのイメージとdigestから署名のタグを定めているからなのでしょうか？このイメージはtarになっていないので`docker pull`することは出来ません。例えば[crane](https://github.com/google/go-containerregistry/blob/main/cmd/crane/README.md)などを使ってメタデータを表示できます:

```
crane manifest $(cosign triangulate ttl.sh/${IMAGE_NAME}:1h) | jq
```

検証
-----

コンテナの署名を検証するには公開鍵を指定します:

```shell
cosign verify --key cosign.pub ttl.sh/${IMAGE_NAME}:1h
```

これは指定された公開鍵に対して一つでも有効な署名があったら正常終了します。というのも、コンテナには複数の鍵で署名することが可能です。例えば上で説明した通りローカルに生成した鍵とGitLabに生成した鍵でそれぞれ署名した場合、同じタグに二つの署名が保存されます(これはコンテナのレイヤーとして処理されます)。`verify`コマンドはこれの内一つでも有効な署名があるかどうかを検証します。
