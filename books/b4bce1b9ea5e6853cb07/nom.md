---
title: データや設定ファイルのパーサを作る (nom crate)
---

数値計算ユーザーは時として太古に規定された独自形式の設定・データフォーマットをパースする必要があります。ここでは[nom][nom]というパーサコンビネータライブラリを使って現在でも良く使われるメッシュデータフォーマットである[VTK Legacy format][vtk]をパースする例を示すことでその使い方を見ていきます。

[nom]: https://github.com/Geal/nom
[vtk]: https://vtk.org/wp-content/uploads/2015/04/file-formats.pdf

参考
-----

[nom][nom]は汎用なパーサコンビネータなのでユーザーも多く、チュートリアルがいくつか存在しています：

- [benkay86/nom-tutorial - GitHub](https://github.com/benkay86/nom-tutorial/)
- [Rust: nom によるパーサー実装](https://hazm.at/mox/lang/rust/nom/index.html)
