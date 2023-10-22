---
title: 複数のディレクトリを管理する
---

ディレクトリが分かれている場合
------------------------------

```
${PROJECT_HOME}/
  main.cpp
  mod1.hpp
  mod1/
    func1.cpp
    func2.cpp
  mod2.hpp
  mod2/
    func1.cpp
    func2.cpp
```

このくらいになってくるとcmakeによる管理の効果がでてくる。
この場合、

- 単一のCMakeLists.txtを使う方法
- 各ディレクトリにCMakeLists.txtを作る方法

がある。個人的には後者がおすすめである。

### 単一のCMakeLists.txtを用いる

```
${PROJECT_HOME}/
  CMakeLists.txt <- new
  main.cpp
  mod1.hpp
  mod1/
    func1.cpp
    func2.cpp
  mod2.hpp
  mod2/
    func1.cpp
    func2.cpp
```

上述のケースと同様にプロジェクトのトップにCMakeLists.txtを作り、

```cmake
cmake_minimum_required(VERSION 2.8)
add_executable(Main
  main.cpp
  mod1/func1.cpp
  mod1/func2.cpp
  mod2/func1.cpp
  mod2/func2.cpp
)
```

のようにする。上述のケースと本質的に同じである。


### 各ディレクトリにCMakeLists.txtを作る

```
${PROJECT_HOME}/
  CMakeLists.txt <- new
  main.cpp
  mod1.hpp
  mod1/
    CMakeLists.txt <- new
    func1.cpp
    func2.cpp
  mod2.hpp
  mod2/
    CMakeLists.txt <- new
    func1.cpp
    func2.cpp
```

```cmake:CMakeLists.txt
cmake_minimum_required(VERSION 2.8)
add_subdirectory(mod1)                # <- new
add_subdirectory(mod2)                # <- new
add_executable(Main main.cpp)
target_link_libraries(Main Mod1 Mod2) # <- new
```

```cmake:mod1/CMakeLists.txt
cmake_minimum_required(VERSION 2.8)
add_library(Mod1 STATIC
  func1.cpp
  func2.cpp
)
```

```cmake:mod2/CMakeLists.txt
cmake_minimum_required(VERSION 2.8)
add_library(Mod2 STATIC
  func1.cpp
  func2.cpp
)
```

のようにする。少し手間かもしれないが、この方がモジュール化が分りやすいし増えてくると管理が楽。
cmakeでは静的ライブラリが簡単に作れ、さらにリンクも簡単。

### add_library

STATICを付けると静的ライブラリを作る。
上の例だとUNIX上では`mod1/libMod1.a`,`mod2/libMod2.a`を作る。

### target_link_libraries
ライブラリを実行ファイルにリンクする。
具体的にはフラグに`-lMod1 -lMod2`が追加される形。
名前の解決はcmakeが行い、cmakeは自分で作ったライブラリの名前は
（ディレクトリ関係なく）グローバルに保持するので、
ここで`mod1/Mod1`のようにする必要はない。

### add_subdirectory
ディレクトリをcmakeの管理に追加する。
そのディレクトリにCMakeLists.txtが存在しないとエラーになる。
`add_definitions`や`set`による変数の定義は子ディレクトリには伝わりますが、
親ディレクトリには伝わらない。
