---
title: ccmake, cmake-gui
---

コンパイル毎に変えたい場合は`ccmake`コマンドあるいは`cmake-gui`コマンドを使う

```
ccmake .
```

```
cmake-gui .
```

`cmake`本体とは別パッケージになっている事が多いのでapt,yum,pacman等で検索する。
`ccmake`を実行すると
`CMAKE_BUILD_TYPE`
`CMAKE_INSTALL_PREFIX`
の2つだけ表示される。変更したいのは
`CMAKE_CXX_FLAGS`であって、これは`t`を入力すると出現する。
ここで必要なフラグを調整する。
