---
title: シェルスクリプトの実行
---

他のファイルを生成しない場合
----------------------------

ctagsやdoxygenのように、他のソースのコンパイルに関係ない物を生成するためには
`add_custom_target`を使います:

```cmake
add_custom_target(ctags ALL COMMAND "ctags" "-R" "-f" ".tags" "--languages=C++,python" "--exclude='CMake*'")
add_custom_target(document COMMAND "doxygen" "doc/Doxyfile")
```

COMMAND以下に実行したいシェルスクリプトを記述します。
`ALL`を付けると毎回の`make`で自動的に実行されます。

ファイルを生成する場合
---------------------

例えばプロジェクトKSEで使用するProtocol buffersのコードを生成したい場合、
`KSE.pb.cc`と`KSE.pb.h`が生成され、それをコンパイルする必要があります。
この場合は`add_custom_command`を使用します:

```cmake
add_custom_command(
  OUTPUT KSE.pb.cc KSE.pb.h
  DEPENDS KSE.proto
  COMMAND "protoc" "KSE.proto" "--cpp_out=." "--python_out=."
)
```

これによりターゲットに`KSE.pb.cc`, `KSE.pb.h`が追加され

```shell
make KSE.pb.cc
```

で`KSE.pb.cc`が生成されます。
もちろん依存関係は自動で解決されますので、

```cmake
add_library(KSE STATIC
  logger.cpp
  KSE.pb.cc
)
```

のようにある場合は勝手に`KSE.pb.cc`を生成してくれます。

