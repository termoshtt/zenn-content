---
title: はじめに
---

Rust を学ぶ
------------

Rust 強い制約によって強い保証を得るタイプの言語であるため、その恩恵に預かるには言語機能に対する理解が必要不可欠です。言語機能に対する十分な理解がなければ、Rustのコードは不可思議に制約の強いパズルのように見え、どうしてわざわざこんなもので数値計算を記述しないといけないのかと思うことでしょう。またいくつかの言語機能はそれが導入された歴史的な経緯を知らないと、何故そのような回りくどい方法を取らなければならないのだろうと感じるでしょう。

Rust の入門の為の情報は既に多く存在している為この文章では扱いません。Rust の学習には公式のドキュメントが充実しています。

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)

また近年では日本語でも多くの Rust の入門書が出版されています。

- [実践Rustプログラミング入門](https://www.shuwasystem.co.jp/book/9784798061702.html)
- [実践Rust入門](https://gihyo.jp/book/2019/978-4-297-10559-4)
- ...

Rustに関する日本語コミュニティとして [rust-jp slack](http://rust-jp.slack.com/) があります。 [参加リンク](https://join.slack.com/t/rust-jp/shared_invite/enQtODAyODY0MDkyMjU5LTI1YmQxMjQxZGI2MTBkMzZhZTIzYzZmMjBiYmY2MmQyYWE5ZWZjZmVmMzljYWE5ZjM1YTBlYjY3ZjYzMmI5OWI) から誰でも参加することが出来ます。

Rust を使うべきで無い場合
--------------------------

- 対話的にデータを可視化する必要のある探索的データ解析(EDA)の用途には向きません。標準で対話的に実行することをサポートしている [Python][Python] や [Julia][Julia]、あるいは [Gnuplot][Gnuplot] 等を使うと良いでしょう。
- 数学的な記述をそのままコードとして表現したい、という要望を実現するのは難しいでしょう。一般的に、数値計算を計算機に適切に実行させるには数学的なアルゴリズムの記述よりも多くの事、例えばデータの配列をメモリ上にどうやって配置するかといった事をプログラマが計算機に指示する必要があります。Rust は数値計算専用の言語ではなく汎用なプログラミング言語です。[MATLAB][MATLAB] や [Mathematica][Mathematica] など、あるいは [SageMath][SageMath] を使うと良いでしょう。
- プロプライエタリなコンパイラが必要な環境、例えば [NVIDIA GPU][nvidia] や [SX-Aurora TSUBASA][SX] のようなアクセラレータ、あるいは [FX1000][FX] のような独自環境では Rust を上手く動作させるには多くの困難が伴います。もしあなたがこれらの環境へ Rust を対応させる事に喜びを見出すのではなく、単なるユーザーでありたいと望むなら提供されるプロプライエタリな環境をそのまま使用することをお勧めします。

[Python]: https://www.python.org/
[Julia]: https://julialang.org/
[Gnuplot]: http://www.gnuplot.info/
[MATLAB]: https://jp.mathworks.com/products/matlab.html
[Mathematica]: https://www.wolfram.com/
[SageMath]: https://www.sagemath.org/
[nvidia]: https://developer.nvidia.com/cuda-toolkit
[SX]: https://jpn.nec.com/hpc/sxauroratsubasa/index.html
[FX]: https://www.fujitsu.com/jp/products/computing/servers/supercomputer/