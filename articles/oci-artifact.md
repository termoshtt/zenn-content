---
title: "OCI Artifact"
emoji: "📦"
type: "tech"
topics: ["container", "podman", "docker"]
published: true
---

OCI image specification 1.1.0が正式リリースされたことでOCI Artifactと呼ばれていた仕様が確定したので、これについてまとめていきます。

https://github.com/opencontainers/image-spec/releases/tag/v1.1.0

## OCI Image specification
まずOCI Image specificationのおさらいです。OCI (Open Container Initiative)はコンテナの標準化のためのLinux Foundationのサブ団体で、次の3つを定めます：

- [Runtime specification](https://github.com/opencontainers/runtime-spec)
  - コンテナの実行環境に関する標準化です。今回は関係しません。
- [Image specification](https://github.com/opencontainers/image-spec)
  - コンテナのデータ形式に関する標準化です。今回の主題です。
- [Distribution specification](https://github.com/opencontainers/distribution-spec)
  - コンテナのデータをHTTPS上でやり取りするためのAPIの定義です。今回の記事に関係しますがあまり深掘りはしません。

Image specificationはいくつかのデータ形式を定義しており、少し関係がややこしいです。今回はOCI Artifactで必要になる部分について、ボトムアップに解説していきましょう。

### Blobs
- 一つのバイト列のことをBlobと呼びます。おそらくBinary Large OBjectが由来ですが、[Azure Blob Storage](https://learn.microsoft.com/ja-jp/azure/storage/blobs/storage-blobs-introduction)などでは特に説明なく使っているので略語というよりも用語として定着しているようです。
- コンテナの「中身」である任意のデータも、コンテナ自体のメタデータを記述したJSONも等しくBlobとして扱われます。つまりコンテナに含まれる任意のデータはBlobとして保存されます。
- Blobはそのハッシュアルゴリズムとハッシュ値の組みで一意に識別されます。
  - 多くの実装でハッシュ関数は[SHA256](https://ja.wikipedia.org/wiki/SHA-2)が使われます。

### [OCI Content Descriptors](https://github.com/opencontainers/image-spec/blob/v1.1.0/descriptor.md) (descriptor)

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

### [OCI Image Manifests](https://github.com/opencontainers/image-spec/blob/v1.1.0/manifest.md) 

個々のコンテナに対応するものがManifestです。Manifestは次のフィールドを持つJSONです：

- `schemaVersion` (int, REQUIRED)
  - このManifestのスキーマバージョンです。現在は `2` が使われます。
- `config` (descriptor, REQUIRED)
  - これはREQUIREDですが、OCI Artifactの場合には不要なケースがあり、その場合は `application/vnd.oci.empty.v1+json` を指定します。
- `layers` (array of descriptor, REQUIRED)
  - 互換性のためにこれは最低でも一つのDescriptorを持つ必要がありますが、OCI Artifactの場合には不要なケースがあり、その場合は `application/vnd.oci.empty.v1+json` を1つだけ入れます。

加えてOCI Artifactにとって重要なOptionalなフィールドもあります：

- `artifactType` (string, OPTIONAL)
  - Artifactを入れる場合に任意のMedia Typeを入れます。
- `annotations` (string-string map, OPTIONAL)
  - コンテナレベルのメタデータを格納します。Keyのルールについては後述します。

当初はOCI ArtifactはImage Manifestとは別の"Artifact Manifest"として定義しようとされていましたが、最終的にはImage Manifestに統合されました。なのでImage Manifestは（実行を行う）コンテナイメージもArtifactの両方を保存する事ができます。

実行するコンテナイメージの場合には `config` に [`application/vnd.oci.image.config.v1+json`](https://github.com/opencontainers/image-spec/blob/v1.1.0/config.md) のMedia Typeを持つdescriptorを指定します。この中にはManifestのファイルシステムのレイヤーの情報やコンテナで実行する際の環境変数やコマンドの情報が記述されていますが、今回は必要ないので省略します。

### [Annotations](https://github.com/opencontainers/image-spec/blob/v1.1.0/annotations.md)

TBW

## OCI Artifact
OCI Image specificationの構造を概ね把握したので、任意のArtifactをImage manifestとして保存する方法を見ていきます。[Guidelines for Artifact usage](https://github.com/opencontainers/image-spec/blob/v1.1.0/manifest.md#guidelines-for-artifact-usage)にガイドラインが書かれているので、この内容に沿って進めます。

OCI ArtifactではImage manifestの `config` と `layers` を使ってArtifactを格納します。基本的にArtifactをBlobとして保存し `layers` にそれらへのDescriptorを入れて、`config` にはそれをどのように扱うかの情報を載せます。大きく分けて3つのケースがあります：

### `config` も `layers` も不要なケース

ManifestのAnnotationだけがあれば十分なケースです。

TBW

### `layers` のみ使うケース

一つ以上のBlobが必要になるケースであり、それをどう扱うかの情報は必要ない、あるいはAnnotationのstring-string mapで十分なケースです。

TBW

### `config` と `layers` の両方を使うケース

TBW
