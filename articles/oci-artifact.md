---
title: "OCI Artifact based on image specification 1.1.0"
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

Image specificationはいくつかのデータ形式を定義しており、少し関係がややこしいです。今回はあえて一番下から解説していきましょう。

### Blobs
- 一つのバイト列がBlobであり、コンテナは複数のBlobから成り立つのでそれらを集めたものがBlobsです。
- コンテナの「中身」である任意のデータも、コンテナ自体のメタデータを記述したJSONも等しくBlobとして扱われます。つまりコンテナに含まれる任意のデータはBlobとして保存されます。
- Blobはそのハッシュ値（アルゴリズムは可変だが実用上はSHA256）で一意に識別されます。

### [OCI Content Descriptors](https://github.com/opencontainers/image-spec/blob/v1.1.0/descriptor.md) (descriptor)

Blobに任意のデータが入っていますが、これを読む際にはどのようなデータが入っているのかを知る必要があります。そのためにContent Descriptorsがあります。Descriptorというのは次のフィールドを持つJSONです：

- `digest` (REQUIRED)
  - このDescriptorが指すデータのハッシュ値です。Blobはハッシュ値で特定できるのでこれだけで参照として動作します。
- `size` (REQUIRED)
  - このDescriptorが指すBlobのサイズです。
- `mediaType` (REQUIRED)
  - このDescriptorが指すデータのMIMEタイプです。例えばこのDescriptorが指しているBlobがただのテキストデータなら `text/plain` が、PDFなら `application/pdf` が入ります。

任意のデータに対するリファレンス型です。

一番基礎となるデータです。

