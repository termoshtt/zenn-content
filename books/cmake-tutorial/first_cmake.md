---
title: はじめてのcmake
---

ここではcmakeを使い始めるために小さいプロジェクトをビルドしてみましょう。
まず単一のC++ソースコードからなるプロジェクトを考えます:

```
${PROJECT_HOME}/
  main.cpp
```

この時は以下のように設定を記述します

```cmake
cmake_minimum_required(VERSION 3.0)  # cmakeの最小バージョン
project("HelloCMake")                # プロジェクトの名前
add_executable(Main main.cpp)        # 実行ファイルを追加する
```

これを`CMakeLists.txt`として`main.cpp`と同じディレクトリにおきます。
このファイル名は特別で CMakeList**s**.txt な事に注意してください

```
${PROJECT_HOME}/
  main.cpp
  CMakeLists.txt
```

これで準備が出来ました。同じディレクトリでを次のコマンドを実行すれば実行ファイルMainが作成されます

```console
cmake . # .を忘れずに
make
```

複数のソースとヘッダの場合
-------------------------

```
${PROJECT_HOME}/
  main.cpp
  mod.hpp
  mod_func1.cpp
  mod_func2.cpp
```

この場合も同じディレクトリに以下の`CMakeLists.txt`を追加する。

```cmake
cmake_minimum_required(VERSION 3.0)
project("HelloCMake")
add_executable(Main
  main.cpp
  mod_func1.cpp
  mod_func2.cpp
)
```

`add_executable`に引数を追加しました。cmakeのスクリプトではスペースが引数の区切り文字になります。
この時ヘッダーファイル `mod.hpp` を追加する必要はありません。
ソースファイルが依存しているヘッダファイルはcmakeが自動的に検出して適切に依存関係を構築してくれます。

コンパイルフラグを指定する
-------------------------

常にコンパイル必要な場合は`add_definitions`を使う

```cmake
cmake_minimum_required(VERSION 3.0)
project("HelloCMake")
add_executable(Main
  main.cpp
  mod_func1.cpp
  mod_func2.cpp
)
```

Links
------
- [add_executable](https://cmake.org/cmake/help/latest/command/add_executable.html)
- [add_definitions](https://cmake.org/cmake/help/latest/command/add_definitions.html)
