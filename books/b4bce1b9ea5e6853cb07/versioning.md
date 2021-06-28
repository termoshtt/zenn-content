---
title: ライブラリの配布とバージョン管理
---

数値計算をRustで行う為のライブラリを紹介していく前に、まずライブラリについての考え方をまとめておきましょう。たとえあなたが必要な全てを自分で作り上げるとしても、ライブラリの扱いについて習熟しておくことは極めて重要です。

ソフトウェア、特に自由ソフトウェアはいくらでも複製が行えるため容易に他人と共有する事が出来ます。標準的な方法でソフトウェアが配布されていることで、あなたと面識の無い他人があなたのソフトウェアを使って研究を行う事が出きるでしょう。ではあなたが作ったソフトウェアを他人と共有したい場合あなたは何をする必要があるでしょうか。複数の人間でソフトウェアを共有して継続的に開発し続けていく方法論はソフトウェアエンジニアリングの文脈で長い時間をかけて試行錯誤が繰り替えされてきました。これはエンジニアリングの問題なので以下に述べる方法も未来に渡って常に有効とは限りませんが、それでも何を問題としどうやって解決してきたかを理解することは十分に価値があります。

crate
------
Rustでは開発者がライブラリを公開する際の単位としてcrateというものが言語レベルで定められています。例えば配布方法が言語側で定められていないC++ではユーザーの環境毎にどのように外部ライブラリを取得するかに膨大な選択肢とそれに応じた手順が存在し、それによって外部のライブラリを使用することそれ自体が極めて専門性の高い技術となります。一方PythonやRuby、Node.jsなどでは配布方法を標準化しており、これにより自動的に外部ライブラリを決まった手順で取得する事が可能になるため外部ライブラリを使用するコストが激減し、より細かい単一の責任を負う単純なライブラリを開発する事が可能となります。ユーザーは多くのライブラリから自分が必要なものだけを選択し、それらを組み合わせる事で自分の実現したいソフトウェアを作成することになります。

crateを作る部分については公式のチュートリアル[Cargoでプロジェクトを作成する][cargo-new]が詳しいのでこちらを確認してください。

crateの配布と取得
-----------------
ライブラリはGitによる個々の変更に対するバージョン管理機構に加えて、リリースという単位でバージョンを管理します。典型的にはGitのタグと対応します。開発者はソフトウェアが他人の使用に耐えうると判断した段階でリリースします。一度リリースされたものは番号が付与され将来に渡って同一となり、新たなリリースでは別の番号を付与します。これによりいつでも過去のバージョンのソフトウェアを取得出きるようになります。

crateは`cargo publish`コマンドによって[crates.io][crates.io]に公開されます。これは[Rust Foundation](https://foundation.rust-lang.org/)によって運営されている公式のcrateレジストリです。別のレジストリを使うことも出来ますがここでは割愛します。この際`major.minor.patch`の形で番号が付与されます。これは`cargo publish`時の`Cargo.toml`にある：

```toml:Cargo.toml
[package]
version = "0.1.2"
```

によって定まります。この例では`major=0`, `minor=1`, `patch=2`になります。これは実体としてはソースコードのアーカイブになります。`cargo`ビルドシステムは`Cargo.toml`内の`[dependencies]`句にしたがって依存しているライブラリのソースコードを[crates.io][crates.io]から取得し、それをビルドしてリンクします。

```toml:Cargo.toml
[dependencies]
num-traits = "0.3.0"
```

例えば次の節で解説する[num-traits][num-traits]を取得して使用するには上のように書きます。この際名前`num-triats`とバージョン番号`0.3.0`を頼りに`cargo`は[crates.io][crates.io]からcrateを探します。

Semantic Versioning
--------------------
現代のプログラミングでは多くのパッケージを組み合わせて使うことが前提となりますが、一方で正しいAPIの設計を最初から行う事は非常に困難です。ライブラリの開発が進むにつれて互換性を破壊するような更新を入れるタイミングはどうしても必要となります。ライブラリのAPIが変更されればそれに依存しているソフトウェアにも変更が必要になりますが、ライブラリの変更が互換性のあるものなのか破壊的なのかを毎回具体的な変更履歴に基づいて判断するのは非常に骨の折れる作業です。そこで互換性の有無を機械可読な形で提供するのが[semantic versioning][semver]の目的です。

上述の例の`0.1.2`のように`major.minor.patch`の形で振られた番号がバージョン番号になります。crateを更新する際開発者はその更新がどのように互換性を維持するかに応じて3つのうちいずれかのバージョン番号を更新します。更新のルールについては[セマンティック バージョニング仕様書][semver-rule]を参照してください。

Cargo.lock
-----------
Semantic Versioningは有益な仕組みですが人間が作業する以上間違えることもあります。例えば本来非互換になるはずの変更を互換だと認識してリリースしてしまう事もあるでしょう。現在のところこれを自動的に解決する容易な方法が無いため人間の注意力に依存しています。間違いを見つけたときはなるべく報告するべきですが、多くのユーザーにとっては自分の仕事を完遂することの方が重要でしょう。

上述したようにcargoはsemantic versioningに基づいて取得するパッケージのバージョンを決定しますが、具体的にどのバージョンのパッケージを取得したかをCargo.lockファイルに記録します。逆にCargo.lockファイルが既に存在している時cargoはsemantic versioningに基づいてバージョン解決せずCargo.lockに基づいてパッケージを取得します。この機構によりCargo.lockをGit管理にしてしまえば、常に決まったcrateを使い続ける事が出来ます。これはライブラリでなく最終的なアプリケーションを書く際に有用です。一般的にRustではライブラリ用(lib)のcrateとアプリケーション用(bin)のcrateが区別されており、ライブラリを作る際にはCargo.lockはバージョン管理せずアプリケーションを作る際はバージョン管理する事が推奨されています。

[cargo-new]: https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html#cargo%E3%81%A7%E3%83%97%E3%83%AD%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E3%82%92%E4%BD%9C%E6%88%90%E3%81%99%E3%82%8B
[crates.io]: https://crates.io/
[num-traits]: https://github.com/rust-num/num-traits
[semver]: https://semver.org/lang/ja/
[semver-rule]: https://semver.org/lang/ja/#%E3%82%BB%E3%83%9E%E3%83%B3%E3%83%86%E3%82%A3%E3%83%83%E3%82%AF-%E3%83%90%E3%83%BC%E3%82%B8%E3%83%A7%E3%83%8B%E3%83%B3%E3%82%B0%E4%BB%95%E6%A7%98%E6%9B%B8-semver