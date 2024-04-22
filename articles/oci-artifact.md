---
title: "OCI Artifact"
emoji: "📦"
type: "tech"
topics: ["container", "podman", "docker"]
published: false
publication_name: "jij_inc"
---

OCI image specification 1.1.0が正式リリースされたことでOCI Artifactと呼ばれていた仕様が確定したので、これについてまとめていきます。

https://github.com/opencontainers/image-spec/releases/tag/v1.1.0

# OCI Image specification
まずOCI Image specificationのおさらいです。OCI (Open Container Initiative)はコンテナの標準化のためのLinux Foundationのサブ団体で、次の3つを定めます：

- [Runtime specification](https://github.com/opencontainers/runtime-spec)
  - コンテナの実行環境に関する標準化です。今回は関係しません。
- [Image specification](https://github.com/opencontainers/image-spec)
  - コンテナのデータ形式に関する標準化です。今回の主題です。
- [Distribution specification](https://github.com/opencontainers/distribution-spec)
  - コンテナのデータをHTTPS上でやり取りするためのAPIの定義です。今回の記事に関係しますがあまり深掘りはしません。

Image specificationはいくつかのデータ形式を定義しており、少し関係がややこしいです。今回はOCI Artifactで必要になる部分について、ボトムアップに解説していきましょう。

## Blobs
- 1つのバイト列のことをBlobと呼びます。おそらくBinary Large OBjectが由来ですが、[Azure Blob Storage](https://learn.microsoft.com/ja-jp/azure/storage/blobs/storage-blobs-introduction)などでは特に説明なく使っているので略語というよりも用語として定着しているようです。
- コンテナの「中身」である任意のデータも、コンテナ自体のメタデータを記述したJSONも等しくBlobとして扱われます。つまりコンテナに含まれる任意のデータはBlobとして保存されます。
- Blobはそのハッシュアルゴリズムとハッシュ値の組みで一意に識別されます。
  - 多くの実装でハッシュ関数は[SHA256](https://ja.wikipedia.org/wiki/SHA-2)が使われます。

## [OCI Content Descriptors](https://github.com/opencontainers/image-spec/blob/v1.1.0/descriptor.md) (descriptor)

Blobに任意のデータが入っていますが、これを読む際にはどのようなデータが入っているのかを知る必要があり、そのためにContent Descriptorsがあります。Descriptorというのは次のフィールドを持つJSONです：

- `digest` (string, REQUIRED)
  - このDescriptorが指すデータのハッシュ値です。Blobはハッシュ値で特定できるのでこれだけで参照として動作します。
- `size` (int, REQUIRED)
  - このDescriptorが指すBlobのサイズです。
- `mediaType` (string, REQUIRED)
  - このDescriptorが指すデータのMedia Typeタイプです。これは[RFC 6838](https://datatracker.ietf.org/doc/html/rfc6838)に則った文字列である必要があります。例えばこのDescriptorが指しているBlobがただのテキストデータなら `text/plain` が、PDFなら `application/pdf` が入ります。
  - OCI Image specificationではコンテナを構成するそれぞれのBlobの種類ごとにMedia Typeが `application/vnd.oci.*`の名前空間に定義されます。
  - 例えばファイルシステムのレイヤーをtar.gzで圧縮したデータを格納したblobは `application/vnd.oci.image.layer.v1.tar+gzip` が使われます。またDescriptor自体は `application/vnd.oci.descriptor.v1+json` というMedia Typeを持ちます。
- `annotations` (string-string map, OPTIONAL)
  - このDescriptorに関するメタデータを格納します。これは各実装が任意のデータを格納できますが、特にKeyについてルールがあります。このルールについては後述します。

Optionalなフィールドもありますが一旦省略します。このようにDescriptorはBlobにどのようなデータが入っているのかを識別できる参照として機能するので、以降に登場するデータ形式ではBlobをDescriptorで参照します。

なお仕様上Descriptorが必須になっているが実際には必要ないケースがあり、その場合のために[空のDescriptor](https://github.com/opencontainers/image-spec/blob/v1.1.0/manifest.md#guidance-for-an-empty-descriptor)が定義されています。

```json
{
  "mediaType": "application/vnd.oci.empty.v1+json",
  "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
  "size": 2,
  "data": "e30="
}
```

これはバイナリデータとして空のJSON `{}` (バイト列としては空ではない) を格納していることになり、これをBase64エンコードしたものが `data` フィールドに入っている `e30=` です。

## [OCI Image Manifests](https://github.com/opencontainers/image-spec/blob/v1.1.0/manifest.md) 

個々のコンテナに対応するものがManifestです。Manifestは次のフィールドを持つJSONです：

- `schemaVersion` (int, REQUIRED)
  - このManifestのスキーマバージョンです。現在は `2` が使われます。
- `config` (descriptor, REQUIRED)
  - これはREQUIREDですが、OCI Artifactの場合には不要なケースがあり、その場合は `application/vnd.oci.empty.v1+json` を指定します。
- `layers` (array of descriptor, REQUIRED)
  - 互換性のためにこれは最低でも1つのDescriptorを持つ必要がありますが、OCI Artifactの場合には不要なケースがあり、その場合は `application/vnd.oci.empty.v1+json` を1つだけ入れます。

加えてOCI Artifactにとって重要なOptionalなフィールドもあります：

- `artifactType` (string, OPTIONAL)
  - Artifactを入れる場合に任意のMedia Typeを入れます。
- `annotations` (string-string map, OPTIONAL)
  - コンテナレベルのメタデータを格納します。Keyのルールについては後述します。

当初はOCI ArtifactはImage Manifestとは別の"Artifact Manifest"として定義しようとされていましたが、最終的にはImage Manifestに統合されました。なのでImage Manifestは（実行を行う）コンテナイメージもArtifactの両方を保存できます。

実行するコンテナイメージの場合には `config` に [`application/vnd.oci.image.config.v1+json`](https://github.com/opencontainers/image-spec/blob/v1.1.0/config.md) のMedia Typeを持つdescriptorを指定します。この中にはManifestのファイルシステムのレイヤーの情報やコンテナで実行する際の環境変数やコマンドの情報が記述されていますが、今回は必要ないので省略します。

## [Annotations](https://github.com/opencontainers/image-spec/blob/v1.1.0/annotations.md)

DescriptorやManifestには任意のメタデータを格納するための `annotations` フィールドがあります。これはKeyとValueが文字列のペアで、Keyは次のルールに従う必要があります：

- Keyは `com.example.key` のようなReverse domain notation形式で、`com.example` のような名前空間を持つ必要があります。
- `org.opencontainers` という名前空間はOCI Image specificationのために予約されています。

# OCI Artifact
OCI Image specificationの構造を概ね把握したので、任意のArtifactをImage manifestとして保存する方法を見ていきます。[Guidelines for Artifact usage](https://github.com/opencontainers/image-spec/blob/v1.1.0/manifest.md#guidelines-for-artifact-usage)にガイドラインが書かれているので、この内容に沿って進めます。以下のManifestのJSONはここからの引用です。

OCI ArtifactではImage manifestの `config` と `layers` を使ってArtifactを格納します。Image manifestを扱うアプリケーションは基本的に

1. Image manifestのJSONを読み込み、
2. 必要であれば `config` を読み込み、
3. Manifestと `config` の内容を踏まえて `layers` を読み込む

というフローをとることになります。Artifactとして何を保存したいのかによって大きく次の３つのケースに分けられます：

## `config` も `layers` も不要なケース

ManifestのAnnotationだけがあれば十分なケースです。この場合はManifestのJSONだけで全てが完結します。

```json
{
  "schemaVersion": 2,
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "artifactType": "application/vnd.example+type",
  "config": {
    "mediaType": "application/vnd.oci.empty.v1+json",
    "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
    "size": 2
  },
  "layers": [
    {
      "mediaType": "application/vnd.oci.empty.v1+json",
      "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
      "size": 2
    }
  ],
  "annotations": {
    "oci.opencontainers.image.created": "2023-01-02T03:04:05Z",
    "com.example.data": "payload"
  }
}
```

`config` と `layers` にともに空のDescriptorを入れます。この場合必要なのは `artifactType` と `annotations` だけです。

`artifactType` は上の例では `application/vnd.example+type` となっていますが、ここには任意のMedia Typeをいれれるので、このArtifactを扱うアプリケーションが分かる識別子を入れます。アプリケーションはユーザーが指定したコンテナイメージのManifestを取得してみて、これが自分の想定していないMedia Typeだったらエラーを返すわけですね。このようにOCI Artifactはアプリケーションごとに何を入れるのかを自由にカスタマイズできます。

`annotations` がこのケースではArtifactの本体となり、任意のstring-stringのkey-valueペアをArtifactとして扱いたい場合にこれを使います。

## `layers` のみ使うケース

1つ以上のBlobが必要になるケースであり、それをどう扱うかの情報は必要ない、あるいはAnnotationのstring-string mapで十分なケースです。

```json
{
  "schemaVersion": 2,
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "artifactType": "application/vnd.example+type",
  "config": {
    "mediaType": "application/vnd.oci.empty.v1+json",
    "digest": "sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
    "size": 2
  },
  "layers": [
    {
      "mediaType": "application/vnd.example+type",
      "digest": "sha256:e258d248fda94c63753607f7c4494ee0fcbe92f1a76bfdac795c9d84101eb317",
      "size": 1234
    }
  ]
}
```

この例は1つの `application/vnd.example+type` 型のBlobだけが含まれるArtifactです。例えば1つのファイルをgzipで圧縮して保存する場合、このGzipファイルをBlobとして保存し、そのDescriptorのMedia Typeを `application/gzip` として `layers` に入れます。この場合 `config` は不要です。

あるいはユーザーのマシンにおける複数のディレクトリの内容をそれぞれ `tar.gz` にまとめて複数のBlobとして保存し、そのDescriptorたちを `layers` に保存することもできます。この場合どのディレクトリに対応するDescriptorか分からなくなるのでDescriptorの `annotations` に `vnd.yourapplication.directory.path` のようなKeyを作ってパスを入れておくと良いでしょう。この `annoations` はManifestのJSONに含まれることになるので、アプリケーションはManifestを見た段階でコンテナ全体をダウンロードせずに必要になったBlobだけを取得することもできます。

## `config` と `layers` の両方を使うケース

上の `annoations` を利用したメタデータでは扱いきれない場合には `config` に別のBlobを用意することになります。

```json
{
  "schemaVersion": 2,
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "artifactType": "application/vnd.example+type",
  "config": {
    "mediaType": "application/vnd.example.config.v1+json",
    "digest": "sha256:5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03",
    "size": 123
  },
  "layers": [
    {
      "mediaType": "application/vnd.example.data.v1.tar+gzip",
      "digest": "sha256:e258d248fda94c63753607f7c4494ee0fcbe92f1a76bfdac795c9d84101eb317",
      "size": 1234
    }
  ]
}
```

`config` には任意のMedia Typeを持つDescriptorを指定できるので、例えばBlobを読み出すための複雑な設定が記述されたJSONやYAML、あるいはJavaScriptやPythonのスクリプト、wasm binaryが入っているかもしれません。`layers` と `config` をどう使うかはArtifactを利用するアプリケーションが決めることになります。

# 最後に

＼Rustエンジニア募集中！　／
株式会社Jijでは、数学や物理学のバックグラウンドを活かし、量子計算と数理最適化のフロンティアで活躍するRustエンジニアを募集しています！
詳細は下記のリンクからご覧ください。 **皆様のご応募をお待ちしております！**
https://open.talentio.com/r/1/c/j-ij.com/pages/51062

JijのXのフォローもよろしくお願いします！

https://twitter.com/Jij_Inc_JP/status/1722874215060349290
