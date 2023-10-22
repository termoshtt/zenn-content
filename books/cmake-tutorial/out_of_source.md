---
title: Out-of-Source Build
---

ここまで簡単のために

```
cmake .
make
```

のようにビルドして来ましたがこれについて少し詳しく見ていきましょう。
`cmake`コマンドは引数にディレクトリを取ります。
まずシェル上では`.`は現在のディレクトリ `$PWD` を指すので、
上のコマンドは`cmake`に現在のディレクトリにある`CMakeLists.txt`を探しに行けという命令をしていることに成ります。

さて `cmake .` を実行すると次のようなファイルが`$PWD`に生成されているはずです

```
CMakeFiles/
Makefile
cmake_install.cmake
CMakeCache.txt
```

これらはcmakeが生成したビルド用の設定ファイル群で、ユーザーは中身を見る必要の無いものです。
これらをどこに出力するかはcmakeの`-B`, `--build`オプションで指定でき、デフォルトでcmakeコマンドを実行したディレクトリになります。

```
cmake . -B build
```

このように`CMakeLists.txt`を探すパス`.`と設定を出力するパス`build`を指定することが出来ます。
それぞれcmakeのスクリプト中では`${CMAKE_SOURCE_DIR}`, `${CMAKE_BINARY_DIR}` として参照できます

Links
-----
- [CMAKE_SOURCE_DIR](https://cmake.org/cmake/help/latest/variable/CMAKE_SOURCE_DIR.html)
- [CMAKE_BINARY_DIR](https://cmake.org/cmake/help/latest/variable/CMAKE_BINARY_DIR.html)
