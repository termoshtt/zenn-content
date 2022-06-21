---
title: "コンテナの名前として許される文字列"
emoji: "📦"
type: "tech"
topics: ["container", "docker"]
published: true
---

コンテナの名前について複数のルールがあることに気がついたのでメモ

Docker
-------

```
$ docker --version
Docker version 20.10.16, build aa7e414fdc
```

https://docs.docker.com/engine/reference/commandline/tag/
> Name components may contain lowercase letters, digits and separators. A separator is defined as a period, one or two underscores, or one or more dashes. A name component may not start or end with a separator.

http://docs.docker.jp/engine/reference/commandline/tag.html
> 構成名は（アルファベット）小文字、数字、セパレータ（分離記号）を含みます。セパレータの定義はピリオド、１つか２つのアンダースコア、複数のダッシュです。セパレータは、コンポーネント名の始めと終わりで使えません。

アルファベットと数字はいいとして注意すべきはセパレータの定義です。Dockerでは `.` `_`, `__`, 及び複数のダッシュ `----` を許可します。

```
$ echo "FROM alpine" > Dockerfile
$ docker build -t a___b .
invalid argument "a___b" for "-t, --tag" flag: invalid reference format
```

複数のセパレータは連続させられません。

```
$ docker build -t a_.b . 
invalid argument "a_.b" for "-t, --tag" flag: invalid reference format
```

Podman
-------

```
$ podman --version
podman version 4.1.0
```

どこに制約が書いてあるか分かりませんでしたが、Dockerと同じ挙動をします

https://docs.podman.io/en/latest/markdown/podman-build.1.html

```
$ podman build -t a___b .
Error: tag a___b: invalid reference format
```

```
$ podman build -t a-.b . 
Error: tag a-.b: invalid reference format
```

OCI image spec
---------------
さて制約が異なるのがこれです。

https://github.com/opencontainers/image-spec/blob/main/annotations.md#pre-defined-annotation-keys
> org.opencontainers.image.ref.name Name of the reference for a target (string).
>  - SHOULD only be considered valid when on descriptors on index.json within image layout.
>  - Character set of the value SHOULD conform to alphanum of A-Za-z0-9 and separator set of -._:@/+
>  - The reference must match the following [grammar](https://github.com/opencontainers/image-spec/blob/main/considerations.md#ebnf): 
>
> ```
> ref       ::= component ("/" component)*
> component ::= alphanum (separator alphanum)*
> alphanum  ::= [A-Za-z0-9]+
> separator ::= [-._:@+] | "--"
> ```

名前の規則がEBNFで定義されいます。コンテナの名前はannotationとしてコンテナのメタデータに書き込みます。この際`org.opencontainers.image.ref.name`というタグでアノテーションすると決められていますが、この名前の制約が上のとおりでDockerと少し違います。

例えば`a__b`, `a---b`等はDockerでは有効ですが、OCI image specではセパレータは`-`, `.`, `_`, `:`, `@`, `+`, あるいは`--`でなければならず、セパレータは連続出来ないので無効になります。このような無効な名前を使った場合、`podman`ではoci-archive formatで保存しようとすると失敗します。

```
$ podman build -t a---b .
$ podman save -o oci-alpine.tar --format oci-archive a---b
Error: Invalid image localhost/a---b:latest
```
