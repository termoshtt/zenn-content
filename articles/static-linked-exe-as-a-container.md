---
title: "素手でコンテナを作る"
emoji: "📚"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["container", "docker", "podman", "tar"]
published: false
---

この記事では静的リンクされた実行可能ファイルを `tar` と手書きのJSONでコンテナイメージにパッケージし、`podman` で実行する事でコンテナイメージの仕様を理解します。コンテナの実行する必要があるのこの記事ではLinux環境を前提とします。また筆者の環境の都合で `x86_64` の話をします。

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

# コンテナイメージの仕組み

[以前の記事](https://zenn.dev/jij_inc/articles/oci-artifact) でもOCI (Open Container Initiative) の定めるコンテナイメージの仕様について解説しましたが、この時はArtifactとして利用する想定だったので、ここで改めて実行可能なコンテナイメージの仕組みを解説します。

コンテナイメージは次のようなディレクトリ構造を持つtarアーカイブです

```text
.
├── index.json
├── oci-layout
└── blobs
    └── sha256
        ├── 3588d02542238316759cbf24502f4344ffcc8a60c803870022f335d1390c13b4
        ├── 4b0bc1c4050b03c95ef2a8e36e25feac42fd31283e8c30b3ee5df6b043155d3c
        └── 7968321274dc6b6171697c33df7815310468e694ac5be0ec03ff053bb135e768
```

この内容について順番に解説していきます。

## `oci-layout`

まずは簡単なものから見ていきましょう。`oci-layout` はOCIの仕様に従ったディレクトリ構造を持つことを示すファイルで、JSON形式でバージョンを記述します

```json:oci-layout
{ "imageLayoutVersion": "1.0.0" }
```

上で例示したディレクトリ構成が image layout version 1.0.0 です。

## `blobs`

BLOBというのは元々はBinary Large OBjectの略だったと思いますが、現在では固有名詞として単にバイナリ列のことを指すことが多いです。コンテナの構成要素であるファイルシステムのレイヤーや、実行のためのメタデータを記述したJSONなども全てBLOBとして保存されます。ファイル名を見るとわかるとおり、ここのBLOBはそのハッシュ値の名前で保存されます。ハッシュアルゴリズムは変更できるような設計になっていますが、基本的にSHA256が使われます。

BLOBは単なるバイナリ列なので、これが何のファイルであってどう読むべきなのかをどうにかして知る必要があります。そのためOCIの仕様では、BLOBを参照する時にそのハッシュ値とセットでMedia Typeを指定します。[Media Type](https://ja.wikipedia.org/wiki/%E3%83%A1%E3%83%87%E3%82%A3%E3%82%A2%E3%82%BF%E3%82%A4%E3%83%97)というのは `text/plain` や `application/json` などのようにファイルの種類を示すための文字列で、MIMEタイプやコンテンツタイプなどとも呼ばれます。例えばJSONを保存したBLOBがある時は、Media TypeとBLOBのハッシュ値をセットで

```json
{
  "mediaType": "application/json",
  "digest": "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f",
  "size": 7143
}
```

のように記述します。これを Descriptor と呼びます。

Media Typeはただの文字列で、アプリケーション毎に自分が使うMedia Typeを適当な名前空間の下に定義することができます。OCIによる標準化では `application/vnd.oci.*` の名前空間のMedia Typeを使ってファイルの種類を規定します。

## `index.json`

さてBLOBに実際のデータを保存することはわかりましたが、コンテナを取得した時BLOBしかなかったら最初にどれを読めばいいのかわかりません。これを指定するのが `index.json` です。`index.json` の中身はMedia Type `application/vnd.oci.image.index.v1+json` として規定されており、例えば次のようになっています

```json:index.json
{
  "schemaVersion": 2,
  "mediaType": "application/vnd.oci.image.index.v1+json",
  "manifests": [
    {
      "mediaType": "application/vnd.oci.image.manifest.v1+json",
      "size": 7143,
      "digest": "sha256:e692418e4cbaf90ca69d05a66403747baa33ee08806650b51fab815ad7fc331f",
      "platform": {
        "architecture": "ppc64le",
        "os": "linux"
      },
      "annotations": {
        "org.opencontainers.image.ref.name": "v1.0"
      }
    }
  ]
}
```

`schemaVersion` と `mediaType` は互換性のためにこのファイルのバージョンとMedia Typeを示したものです。`manifests` というのがコンテナの中身を記述する部分です。Manifestという英単語は船などの積荷の目録のことです。コンテナのManifestとはコンテナの中身のリストということですね。`index.json` の `manifests` は Descriptor のリストになっています。
