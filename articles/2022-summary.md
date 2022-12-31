---
title: "2022年まとめ"
emoji: "📅"
type: "tech"
topics: ["rust"]
published: true
---

この記事では2022年にやった事を振り返り、2023年にやりたいことを雑にまとめていきます。

Overview
---------

- 1-6月お仕事@[RICOS](https://www.ricos.co.jp/)
- 7-12月無職

2022/6末で2019/7から勤めていた[科学計算総合研究所(RICOS)](https://www.ricos.co.jp/)を退職して自由を求めて無職をしています。無職の間は主にOSS活動と新しいことの勉強をしていました。

OSS活動
========
https://github.com/termoshtt
![GitHub contributions](https://storage.googleapis.com/zenn-user-upload/7e6031176155-20221231.png)

ruststep
---------
https://github.com/ricosjp/ruststep

STEP(Standard for the Exchange of Product model data)というのはISO 10303で規定されるCAD(computer-aided design)やCAE(computer-aided engineering)の為の互換データ形式で、今で言う[Procol Buffers](https://developers.google.com/protocol-buffers/)に相当するものです。

STEPはどのようなデータを扱うのかを定めるスキーマ言語(EXPRESS言語)と共通スキーマ(例えば3D CADの為のデータの定義)からなり、ruststepプロジェクトは`espr`というEXPRESS言語のスキーマからRustの構造体を生成するコンパイラと、`ruststep`というランタイムからなります。

これはRICOSにおいて主にCADライブラリである[truck](https://github.com/ricosjp/truck)でSTEPをサポートするために始まったプロジェクトが独立したものです。個人的なモチベーションとしては、流体や構造のような現実的なシミュレーションを行うためには実際の形状を計算機から簡単に扱える必要があり、そのためには軽量なCADシステムが必要という気持ちが大きいです。

ocipkg
-------
https://github.com/termoshtt/ocipkg
https://zenn.dev/termoshtt/articles/introduction-to-ocipkg

無職になってまずやり始めたのがocipkgです。これはコンテナのエコシステムをパッケージ配布に使おうという企てを実現する為のものです。とはいえ現状では[ORAS(OCI Registry as storage)](https://oras.land/)としてGo実装が存在しているものをRustのcrateとして使えるようにした程度の進捗です。

[OCI(Open Container Initiative)](https://opencontainers.org/)によるコンテナAPIの標準化というのは非常によくできていて、メタデータ付きのアーカイブとしてのOCI image spec、HTTPS上に構築された単純なPush/PullのAPIとしてのOCI distribution spec、そして様々なレベルでの仮想化をサポートできるOCI runtime specからなります。このうちimage/distribution specだけを使って様々なものを公開・配布できるようにしたものがORASです。

ocipkgは現在[intel-mkl-src 0.8.0](https://github.com/rust-math/intel-mkl-src/releases/tag/intel-mkl-src-v0.8.0)以降でIntel MKLのバイナリを再配布するために使用されています。これは今までAWS S3から配布されていましたが、現在はGitHub Container registry (ghcr.io)から配布されています。

einsum-derive
--------------
https://github.com/termoshtt/einsum-derive
https://zenn.dev/termoshtt/articles/einsum-derive

NumPy等でよく知られた`einsum`の機能を[`rust-ndarray`](https://github.com/rust-ndarray/ndarray)で使えるようにしたものです。特に[opt-einsum](https://github.com/dgasmith/opt_einsum)として知られる計算量を自動的に減らす機能を試しに実装する事を目的としていました。つまり行列-行列-ベクトル積$ABv$を計算する添字`ij,jk,k->i`が入力された時、$A(Bv)$のように計算量が少なくなる順序を自動的に判定して、それに基づいて計算するコードを生成します。

これは将来的に数値計算の(手続きでなく)アルゴリズム自体を記述するような処理系、例えば[formura](https://github.com/formura/formura)はステンシル計算を抽象的に記述すると京向けのMPIのコードを生成して実行するような処理系ですが、このような処理系に延長していければいいなと思っています。

Rustで数値計算
---------------
https://zenn.dev/termoshtt/books/b4bce1b9ea5e6853cb07
この本は元々Zennが本をサポートしたときに、過去のQiitaに散らばっていた記事を順番に並べたて本という体にしただけのものです。今年中に全体として整合性のある形に更新していきたいと思っていましたが、実際には1つ記事を追加できただけでした。

私がRustで数値計算を始めたときはLAPACKやFFTWのラッパーすら存在せず、これらを自分で作って公開してきました。この本ではこれらの経験を元に自分で必要なライブラリをRustで作成し公開しメンテナンスしていくための技術をまとめたいと思っています。Rustの入門書にも数値計算の入門書でもなく、(Rustに限らない)数値計算ライブラリを開発していく人向けの本になる予定です。

Rustオンラインもくもく会
-------------------------
https://rust-online.connpass.com/
これは毎週土曜日の午後3時からオンラインで集まって作業をする為の会です。特に音声通話などは行わず、最初に今日やることを宣言して最後に今日やったことを報告します。この記事もこの会中に書いています。これは元々2019/7に初回[#1](https://rust-online.connpass.com/event/139900/)が行われ今年で3年目です。2022年初回が[#125](https://rust-online.connpass.com/event/234985/)で本日が[#176](https://rust-online.connpass.com/event/270514/)です。

今年はrust-jpのコミュニティがslackからZulipに移行した事を受けて、報告は[Zulip](https://rust-lang-jp.zulipchat.com/#streams/124298/mokumoku)で行うようになりました。rust-jpのZulipは基本公開になっているので、もくもく会の報告も公開されています。

新分野開拓
==========
今年新たに勉強した内容をまとめます

圏論
-----
学生の頃から積んであった[圏論の基礎](https://www.maruzen-publishing.co.jp/item/?book_no=294317)を真面目に読み始めました。この記事を書いている段階では第V章極限のあたりです。演習問題をやりながらなのであまり進みません。主なモチベーションとしては`einsum-derive`のときに説明した`ndarray`の性質をもう少し理論的に扱いたいという事があります。ごく最初だけ議論したものがこちらです：

https://mathlog.info/articles/3774

特に流体や構造のような偏微分方程式を解く数値計算では本来連続体で議論すべき内容を有限個の浮動小数点数で扱うことになりますが、この時に元の構造の情報が落ちてしまったりします。この点を上手くモデル化するときに圏論の知識が役に立つといいなという期待もあります。

型システム
-----------
`einsum-derive`のところでも少し触れましたが、[formura](https://github.com/formura/formura)を参考に数値計算を記述する言語を作りたいので型システムを少しずつ勉強しています。主に[型システム入門 プログラミング言語と型の理論(TaPL和訳)](https://www.ohmsha.co.jp/book/9784274069116/)を読んでいますが、来年はサンプルコードをRustで実装とかしたいですね。

最後に
=======
それでは良いお年を(´・ω・｀)！

過去のまとめ
- [2019](https://termoshtt.hatenablog.com/entry/2020/01/01/000313)
- [2018](https://termoshtt.hatenablog.com/entry/2018/12/31/184108)
