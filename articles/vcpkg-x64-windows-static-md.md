---
title: "vcpkg の x86-windows-static-md triplet について"
emoji: "🦀"
type: "tech"
topics: ["cpp", "vcpkg"]
published: true
---

[vcpkg-rs][vcpkg-rs] を使うためにいくつか調べたのでメモ。[microsoft/vcpkg][vcpkg] は "C++ Library Manager for Windows, Linux and Mac OS" ですが今回はこれの説明はしません。

[vcpkg-rs]: https://github.com/mcgoo/vcpkg-rs
[vcpkg]: https://github.com/xianyi/OpenBLAS/pull/2256

Target Triplet in vcpkg
-----------------------
vcpkgにはライブラリを静的にリンクするか動的にリンクするかを指定するために `x64-windows-static` と `x64-windows` というビルドターゲット文字列 (triplet) を使います。例えば

```
vcpkg install openblas --triplet x64-windows
```

例えばこれでOpenBLASが動的ライブラリ (.dll) としてビルドされ、

```
vcpkg install openblas --triplet x64-windows-static
```

これでOpenBLASが静的ライブラリ (.a) としてビルドされます。

:::message
OpenBLASは独自にBLASとLAPACKの一部を最適化したC (+asm) のコードとnetlibのLAPACK実装をそのまま流用している部分があり、後者はFortan実装になります。上の方法はvcpkg -> cmake -> msbuildでVisual StudioのC compilerを用いてC部分だけをコンパイルするためnetlib由来のFortran部分はコンパイルしません。
詳しくは [OpenBLAS Wiki](https://github.com/xianyi/OpenBLAS/wiki/How-to-use-OpenBLAS-in-Microsoft-Visual-Studio) 及び [GitHub の該当 issue](https://github.com/xianyi/OpenBLAS/pull/2256) を参照してください。
:::

実はここにもう1つ種類があって `x64-windows-static-md` というものがあります

```
vcpkg install openblas --triplet x64-windows-static
```

これはライブラリ（この場合OpenBLAS）はstaticにC Runtime (CRT) はdynamicにリンクします。これによりビルド成果物にCRT分が含まれないのでサイズが小さくできます。代わりに実行時にCRTを検索する事になります。

このTripletは [vcpkg][vcpkg] 本家ではなく[コミュニティ管理](https://github.com/microsoft/vcpkg/blob/master/docs/users/triplets.md#community-triplets)になっています。[Why should x64-windows-static-md not be the preferred triplet on Windows?](https://github.com/microsoft/vcpkg/issues/16387) この辺が詳しいです。

[vcpkg-rs][vcpkg-rs] は3通りともサポートしており環境変数 `VCPKGRS_DYNAMIC` と `RUSTFLAGS` を見て処理を切り替えます。[Rust 側で CRT を静的リンクしたい場合](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes)は `RUSTFLAGS=-Ctarget-feature=+crt-static` と指定しますが、[vcpkg-rs][vcpkg-rs] はこの時 `x64-windows-static` を使用するように切り替えます。
