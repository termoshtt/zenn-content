---
title: オプション付きの関数を定義する
---

cmakeの組み込み関数にはオプション引数をとれるものがありますね。例えば[install]は

```
install(TARGETS targets... [EXPORT <export-name>]
        [[ARCHIVE|LIBRARY|RUNTIME|OBJECTS|FRAMEWORK|BUNDLE|
          PRIVATE_HEADER|PUBLIC_HEADER|RESOURCE]
         [DESTINATION <dir>]
         [PERMISSIONS permissions...]
         [CONFIGURATIONS [Debug|Release|...]]
         [COMPONENT <component>]
         [NAMELINK_COMPONENT <component>]
         [OPTIONAL] [EXCLUDE_FROM_ALL]
         [NAMELINK_ONLY|NAMELINK_SKIP]
        ] [...]
        [INCLUDES DESTINATION [<dir> ...]]
        )
```
のようにたくさんのオプションを持ちます。これを自分で実装する話です。

[install]: https://cmake.org/cmake/help/v3.13/command/install.html

cmakeの関数の引数
-----------------
まず`function`を使って関数を定義してみましょう：

```cmake
function(myfunc arg)
  message("ARG1 = ${arg}")
endfunction()

myfunc("a1")
```

```
$ cmake .
ARG1 = a1
...
```

名前を付けた引数は`${arg}`のように参照できますね。実はcmakeは名前を付けなかった引数はすべて`${ARGN}`に入れます

```cmake
function(myfunc arg)
  message("ARG1 = ${arg}")
  message("ARGN = ${ARGN}")
endfunction()

myfunc("a1" "a2" 3)
```

`${ARGN}`はリスト型になるので、表示するときは各成分が`;`でつながれた形になります

```
$ cmake .
ARG1 = a1
ARGN = a2;3
...
```

cmake_parse_arguments
----------------------
この`${ARGN}`は手動でパースしないといけないのでしょうか？ここで登場するのが[cmake_parse_arguments](https://cmake.org/cmake/help/latest/command/cmake_parse_arguments.html)です。
この関数はちょっとややこしい引数をとるのでドキュメントを読んでも分かりにくいです。サンプルをいくつか試してみましょう。

```cmake
function(myfunc arg)
  cmake_parse_arguments(MYFUNC "" "OUTPUT" "" ${ARGN})
  message("ARG1 = ${arg}")
  message("ARGN = ${ARGN}")
  message("OUTPUT = ${MYFUNC_OUTPUT}")
endfunction()

myfunc("a1" "a2" 3 OUTPUT "/path/to/output")
```

これで`OUTPUT`というオプションを持たせることが出来ます。

```
$ cmake .
ARG1 = a1
ARGN = a2;3;OUTPUT;/path/to/output
OUTPUT = /path/to/output
...
```

`OUTPUT`は値を一つとるオプションです。関数に渡したオプションの値は`${MYFUNC_OUTPUT}`として参照できます。この`MYFUNC`はcmake_parse_argumentsの最初の引数で上げた文字列で、cmakeには名前空間が無いので関数名を付けて名前の衝突を回避します。

cmake_parse_argumentsの第2,3,4引数はそれぞれbool値のみを保持するフラグオプション、値を一つとるオプション、値を複数とるオプションです。

```cmake
function(myfunc arg)
  cmake_parse_arguments(MYFUNC "" "OUTPUT" "SOURCES" ${ARGN})
  message("ARG1 = ${arg}")
  message("ARGN = ${ARGN}")
  message("OUTPUT = ${MYFUNC_OUTPUT}")
  message("SOURCES = ${MYFUNC_SOURCES}")
endfunction()

myfunc("a1" "a2" 3
  OUTPUT "/path/to/output"
  SOURCES "vim" "emacs"
  )
```

```
$ cmake .
ARG1 = a1
ARGN = a2;3;OUTPUT;/path/to/output;SOURCES;vim;emacs
OUTPUT = /path/to/output
SOURCES = vim;emacs
...
```

それぞれの引数にリストを上げると複数のオプションが定義できます

```cmake
function(myfunc arg)
  cmake_parse_arguments(MYFUNC "" "OUTPUT" "SOURCES;DEPENDENCIES" ${ARGN})
  message("ARG1 = ${arg}")
  message("ARGN = ${ARGN}")
  message("OUTPUT = ${MYFUNC_OUTPUT}")
  message("SOURCES = ${MYFUNC_SOURCES}")
  message("DEPENDENCIES = ${MYFUNC_DEPENDENCIES}")
endfunction()

myfunc("a1" "a2" 3
  OUTPUT "/path/to/output"
  SOURCES "vim" "emacs"
  DEPENDENCIES "glibc" "linux"
  )
```

```
$ cmake .
ARG1 = a1
ARGN = a2;3;OUTPUT;/path/to/output;SOURCES;vim;emacs;DEPENDENCIES;glibc;linux
OUTPUT = /path/to/output
SOURCES = vim;emacs
DEPENDENCIES = glibc;linux
...
```
