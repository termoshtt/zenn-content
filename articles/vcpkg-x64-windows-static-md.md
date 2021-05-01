---
title: "vcpkg の x86-windows-static-md triplet について"
emoji: "🦀"
type: "tech"
topics: ["cpp", "vcpkg"]
published: true
---

[vcpkg-rs][vcpkg-rs] を使うためにいくつか調べたのでメモ。[microsoft/vcpkg][vcpkg] は "C++ Library Manager for Windows, Linux and MacOS" ですが今回はこれの説明はしません。

[vcpkg-rs]: https://github.com/mcgoo/vcpkg-rs
[vcpkg]: https://github.com/xianyi/OpenBLAS/pull/2256

Target Triplet in vcpkg
-----------------------
vcpkg にはライブラリを静的にリンクするか動的にリンクするかを指定するために `x64-windows-static` と `x64-windows` というビルドターゲット文字列 (triplet) を使います。例えば

```
vcpkg install openblas --triplet x64-windows
```

例えばこれで OpenBLAS が動的ライブラリ (.dll) としてビルドされ、

```
vcpkg install openblas --triplet x64-windows-static
```

これで OpenBLAS が静的ライブラリ (.a) としてビルドされます。

:::message
OpenBLAS は独自に BLAS と LAPACK の一部を最適化した C (+asm) のコードと netlib の LAPACK 実装をそのまま流用している部分があり、後者は Fortan 実装になります。上の方法は vcpkg -> cmake -> msbuild で Visual Studio の C compiler を用いて C 部分だけをコンパイルするため netlib 由来の Fortran 部分はコンパイルしません。
詳しくは [OpenBLAS Wiki](https://github.com/xianyi/OpenBLAS/wiki/How-to-use-OpenBLAS-in-Microsoft-Visual-Studio) 及び [GitHub の該当 issue](https://github.com/xianyi/OpenBLAS/pull/2256) を参照してください。
:::

実はここにもう一つ種類があって `x64-windows-static-md` というものがあります

```
vcpkg install openblas --triplet x64-windows-static
```

これはライブラリ（この場合 OpenBLAS）は static に C Runtime (CRT) は dynamic にリンクします。これによりビルド成果物に CRT 分が含まれないのでサイズが小さくできます。代わりに実行時に CRT を検索する事になります。

この Triplet は [vcpkg][vcpkg] 本家ではなく[コミュニティ管理](https://github.com/microsoft/vcpkg/blob/master/docs/users/triplets.md#community-triplets)になっています。[Why should x64-windows-static-md not be the preferred triplet on Windows?](https://github.com/microsoft/vcpkg/issues/16387) この辺が詳しいです。

[vcpkg-rs][vcpkg-rs] は 3 通りともサポートしており環境変数 `VCPKGRS_DYNAMIC` と `RUSTFLAGS` を見て処理を切り替えます。[Rust 側で CRT を静的リンクしたい場合](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes)は `RUSTFLAGS=-Ctarget-feature=+crt-static` と指定しますが、[vcpkg-rs][vcpkg-rs] はこの時 `x64-windows-static` を使用するように切り替えます。
