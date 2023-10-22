---
title: Git submodule
---

複数のプロジェクトで共通して必要なプロジェクトや、外部のライブラリを使用したいが、システムにインストールしたく無い場合の方法として、
Git Submoduleの機能を使う方法があります。

前提条件
--------

C++プロジェクトprojに外部のC++プロジェクトsubを使用する場合を例に取って考えます。
上述のように以下の方法を実行するには

- projがcmakeで管理されている事
- proj, subがGitで管理されている事
- subがcmakeで管理されているか、ヘッダのみで構成されているテンプレートライブラリである事

が必要です。

手順
-----

まずGitのリポジトリは

- `yourhost:repos/proj.git`
- `anotherhost:repos/sub.git`

のようになっているとします。各自の環境に合わせて読み替えてください。

アイデアは以下の通りです。

- `git submodule`を使用すれば外部のプロジェクトを取得できる
- cmakeの`include_directories`マクロを使用すれば`git submodule`で取得したライブラリを簡単にincludeできる
- 外部プロジェクトがヘッダだけなら、それらをコンパイルする必要がないからincludeできれば十分
- 外部プロジェクトがcmakeで管理されていればそのツリーを親プロジェクトのツリーに取り込める

よって手順としては、

1. projにおいて`git submodule`によりsubを取得する
2. projの`CMakeLists.txt`に`include_directories`を記述する
3. subがcmake管理の場合は`add_subdirectory`マクロで取り込む

のみです。

### 例

```
proj/CMakeLists.txt
     mod1/CMakeLists.txt
          func1.cpp
          func2.cpp
     main.cpp
     sub/
```

```
sub/mod1/func1.hpp
         func2.hpp
    mod2/func1.hpp
```

のような場合

```cmake:proj/CMakeLists
...
include_directories(.)
...
```

としておけば例えば`mod1/func1.cpp`においてincludeパスを指定する事なく:

```cmake:proj/mod1/CMakeLists.txt
add_library(mod1 STATIC
  func1.cpp
  func2.cpp
)
# -Iは特に指定しなくていい
```

```cpp:proj/mod1/func1.cpp
#include "sub/mod2/func1.hpp"
...
```

のようにincludeする事が可能となります。

