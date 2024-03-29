---
title: はじめに
---

Rustを学ぶ
------------

Rust強い制約によって強い保証を得るタイプの言語なのでその恩恵に預かるには言語機能に対する理解が必要不可欠です。言語機能に対する十分な理解がなければ、Rustのコードは不可思議に制約の強いパズルのように見え、どうしてわざわざこんなもので数値計算を記述しないといけないのかと思うことでしょう。またいくつかの言語機能はそれが導入された歴史的な経緯を知らなければ、何故そのような回りくどい方法を取らなければならないのだろうと感じるでしょう。

しかしこの本の目的はRustへの入門ではなくRustで数値計算ソフトウェアを開発する為に必要な知識をまとめることなので、Rustの入門的な内容については扱いません。Rustの学習には公式のドキュメントが充実しています。

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)

また近年では日本語でも多くのRustの入門書が出版されています。

- [実践Rustプログラミング入門](https://www.shuwasystem.co.jp/book/9784798061702.html)
- [実践Rust入門](https://gihyo.jp/book/2019/978-4-297-10559-4)
- ... (他にも多数)

Rustに関する日本語コミュニティとして [rust-jp (Zulip)](https://rust-lang-jp.zulipchat.com/) があります。 これは誰でも参加することが出来、またログインせずにほとんどの投稿を見ることも出来ます。

Rustを使う
-----------
入門的な要素を省く代わりにこの本ではより実践的なRustに限らない一般的なソフトウェアエンジニアリング手法についてもまとめていきます。

数値計算を用いて解決するべき問題の多くは単に対象となる現象を仮想的に計算することではなく、そこで構築された仮想的な現象に対して実現象では適用しがたい様々な数理的操作を用いて解析する事が必要となります。これには様々なソフトウェア、例えば対象を仮想的に再現するシミュレーター、その計算結果の出力から現象の応答のうち重要な部分を切り出し可視化するツールなど多くのしかもそれぞれ専門的な知識が必要となるソフトウェアが必要になります。数理的な研究を行う上で研究者の職責は現象の性質を明らかにすることですが、それを解析するためのソフトウェアが既に存在していないことは多く、研究者自身あるいは共同研究者がソフトウェアを作成することになる事が良くあります。数値計算が主要な研究手法となっている分野においてソフトウェアの開発と保守は科学の発展に対して最も重要な貢献となるでしょう。

電子計算機がまだ大学や研究所にしか無かった頃に比べて現代の計算機は非常に多くのソフトウェアから成り立っており数値計算の為のソフトウェアも例外ではありません。数値計算ソフトウェアを開発する際の困難の多くは実装される数理的なアルゴリズム自体に由来せず通常のソフトウェア開発における困難と同様なので、既存のソフトウェアエンジニアリングの知識がとても役に立ちます。Rustは数値計算専用の言語ではなく比較的新しい汎用のプログラミング言語なので、今まで人類が目にしてきた多くのソフトウェア開発上の困難を解決するための方法が搭載されており、数値計算ソフトウェアを開発・運用する上でもその恩恵に与る事が出来ます。

Rustを使うべきで無い場合
--------------------------

- 対話的にデータを可視化する必要のある探索的データ解析(EDA)の用途には向きません。標準で対話的に実行することをサポートしている [Python][Python] や [Julia][Julia]、あるいは [Gnuplot][Gnuplot] 等を使うと良いでしょう。
- 数学的な記述をそのままコードとして表現したい、という要望を実現するのは難しいでしょう。一般的に、数値計算を計算機に適切に実行させるには数学的なアルゴリズムの記述よりも多くの事、例えばデータの配列をメモリ上にどうやって配置するかといった事をプログラマが計算機に指示する必要があります。Rustは数値計算専用の言語ではなく汎用なプログラミング言語です。[MATLAB][MATLAB] や [Mathematica][Mathematica] など、あるいは [SageMath][SageMath] を使うと良いでしょう。
- プロプライエタリなコンパイラが必要な環境、例えば [NVIDIA GPU][nvidia] や [Vector Engine (a.k.a SX-Aurora TSUBASA)][SX] のようなアクセラレータ、あるいは [FX1000][FX] のような独自環境ではRustを上手く動作させるには多くの困難が伴います。もしあなたがこれらの環境へRustを対応させる事に喜びを見出すのではなく、単なるユーザーでありたいと望むなら提供されるプロプライエタリな環境をそのまま使用することをお勧めします。

[Python]: https://www.python.org/
[Julia]: https://julialang.org/
[Gnuplot]: http://www.gnuplot.info/
[MATLAB]: https://jp.mathworks.com/products/matlab.html
[Mathematica]: https://www.wolfram.com/
[SageMath]: https://www.sagemath.org/
[nvidia]: https://developer.nvidia.com/cuda-toolkit
[SX]: https://jpn.nec.com/hpc/sxauroratsubasa/index.html
[FX]: https://www.fujitsu.com/jp/products/computing/servers/supercomputer/

この本の構成
------------

この本は以下の4章からなります。

- 言語機能の補足と応用
- ソフトウェア開発技術
- 数値計算用ライブラリ紹介
- 別言語との相互運用

これらの章は互いに独立しており、さらにそれらに含まれる個々のページも独立しています。この本は前述のとおり数値計算を題材としてRustを学ぶための本ではなく、Rustを使って数値計算の実務を行う人を対象としています。読者は自分の興味に応じて章の順序を入れ替えて、あるいは特定のページだけを読むことが出来ます。

なおZennにはチャプターを階層化する機能がないので、これらの章は開始チャプターを用意することで実現しています。それぞれの開始チャプターはタイトルの先頭に絵文字が入っています。