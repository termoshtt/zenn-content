---
title: "sigstoreでコンテナに署名する"
emoji: "📦"
type: "tech"
topics: ["sigstore", "cosign", "container", "docker"]
published: true
---

要約
-----

- cosignというsigstoreの提供するツールを使ってコンテナに署名してsignatureを同じレジストリに保存する事が出来る
- cosignにはkeyを管理する方法と、管理せずにOpenID Connectを使ってkeylessで署名する方法がある
- sgetという署名を確認してダウンロードするツールがある

cosign, sgetのインストール
---------------------
環境に応じていくつかインストール方法があります
https://docs.sigstore.dev/cosign/installation

- 自分でビルド
  ```
  go install github.com/sigstore/cosign/cmd/cosign@latest
  ```
- Binaryを取得 (Release 1.6.0)
  ```
  wget "https://github.com/sigstore/cosign/releases/download/v1.6.0/cosign-linux-amd64"
  mv cosign-linux-amd64 /usr/local/bin/cosign
  chmod +x /usr/local/bin/cosign
  ```
- RPM (Release 1.6.0)
  ```
  wget "https://github.com/sigstore/cosign/releases/download/v1.6.0/cosign-1.6.0.x86_64.rpm"
  rpm -ivh cosign-1.6.0.x86_64.rpm
  ```
- DEB (Release 1.6.0)
  ```
  wget "https://github.com/sigstore/cosign/releases/download/v1.6.0/cosign_1.6.0_amd64.deb"
  dpkg -i "cosign_1.6.0_amd64.deb
  ```
- Arch Linux
  ```
  pacman -S cosign
  ```
- Alpine Linux
  ```
  apk add cosign sget
  ```

なおYubiKeyの様なハードウェアトークンを使う機能はデフォルトで有効になっていないため自分でビルドする必要があります。
https://github.com/sigstore/cosign/blob/b01a173cab389e93c5f3b46d50fe503f9c2454c2/TOKENS.md

鍵の生成
---------
sigstoreではkeyless署名を実現するために色々な試みが行われていますが、鍵を管理する方式がデフォルトです。

```
cosign generate-key-pair
```

で秘密鍵のパスワードを入力した後、現在のディレクトリに`cosign.key`(秘密鍵)と`cosign.pub`(公開鍵)を生成されます。

また手元に秘密鍵をおかずにGitLab CIの変数として保存しておくことも出来ます:

```
export GITLAB_TOKEN=glpat-xxxxxxxxxxxxxx  # apiの権限が必要
cosign generate-key-pair gitlab://termoshtt/sigstore-testing
```

![Generated variables in GitLab CI](/images/cosign-generate-key-gitlab.png)
既に変数が存在しているとエラーになります(上書きはされない)。

GitHubでも同じ様にGitHub Actionsの変数として生成することは出来ますが、GitHubにはActionsの変数を取得するAPIが存在していないため、次に説明する署名が実行できません。

署名
-----

まず署名に使うコンテナを用意します。[前回](https://zenn.dev/termoshtt/articles/ttlsh-ephemeral-container-registry)で説明した[ttl.sh](https://ttl.sh)を使います:

```
echo "FROM alpine" > Dockerfile
IMAGE_NAME=$(uuidgen)
docker build -t ttl.sh/${IMAGE_NAME}:1h .
docker push ttl.sh/${IMAGE_NAME}:1h
```

このコンテナに署名するには秘密鍵として`cosign.key`ファイルを使う場合は:

```
cosign sign --key cosign.key ttl.sh/${IMAGE_NAME}:1h
```

GitLabに保存した鍵を使う場合は`--key`に`generate-key-pair`と同じように`gitlab://<user>/<repo>`を指定します:

```
cosign sign --key gitlab://termoshtt/sigstore-testing ttl.sh/${IMAGE_NAME}:1h
```

`cosign sign`は署名したシグネチャを同じレジストリに保存します。

上述したようにGitHubだとこれが出来ません。



検証
-----
