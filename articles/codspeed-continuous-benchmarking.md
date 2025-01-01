---
title: "CodSpeedによる継続的ベンチーマーク"
emoji: "⏱️"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["rust", "criterion", "benchmark", "codspeed"]
published: true
---

CodSpeedは継続的ベンチマークを行うためのサービスです。CodSpeedを使うことで、ベンチマークを自動で実施し可視化できます。CodSpeedは、ベンチマークの結果を比較できるため、コードの変更によるパフォーマンスの変化を追跡できます。

対応言語はRustとPython及びNode.jsです。この記事ではRustを使う場合を説明します。

https://docs.codspeed.io/

PublicなOSSプロジェクトでは無料で使えます。Privateなリポジトリを使う場合は有料です。

https://codspeed.io/pricing

# CodSpeedの概略

:::message
以下の説明はこの記事の執筆時点でのCodSpeedの機能です。今後変更される可能性があることに注意してください。
:::

自分のプロジェクトへの導入方法の前に、[私が導入したプロジェクト](https://github.com/termoshtt/cdcl)で何ができるのかを説明します。これは私が練習でSATソルバーを書いているプロジェクトです。まずGitHubのリポジトリ毎にCodSpeed側にもページが作成されます。このページはログインせずに見ることができます。

https://codspeed.io/termoshtt/cdcl

## Overview

![CodSpeed -- HEAD](/images/codspeed-head.png)

まずデフォルトブランチの履歴毎にパフォーマンスの変化履歴が表示されてます。これは登録してあるベンチマークの結果がそれぞれのコミットで1つ前のコミットの結果と比較されています。このリポジトリでは複数のベンチマークが登録されていますが、おそらく見やすいように適当に集約されています。それ程役に立つグラフではないですが、どのコミットで性能が変化したかくらいは分かりますね。

## Branches

続いて [`Branches` タブ](https://codspeed.io/termoshtt/cdcl/branches)を見てみます。

![CodSpeed -- Branches](/images/codspeed-branches.png)

GitHub上のブランチ毎にパフォーマンスの変化が表示されています。上部のグラフはコミット毎の変化の履歴をブランチ毎に表示しているようです。さて[ブランチ毎のページ](https://codspeed.io/termoshtt/cdcl/branches/faster-cdcl)を見てみましょう。

![CodSpeed -- Branches](/images/codspeed-branches-detail.png)

ようやく複数登録してあるベンチマーク毎の結果の変化が表示されるところまで来ました。このリポジトリではSATソルバーのアルゴリズムとSATの問題毎に求解時間を計測しています。このブランチではCDCLというアルゴリズムのコードの高速化を試みているので、それに伴う性能の変化を追跡しています。

- 中央の `Benchmarks` とあるリストの一番上にある `cdcl[sat2]` というベンチマークはmainブランチでは816msかかっていたものが732msに高速化されていることが分かります。
- 右側の `Commits` とある欄ではコミット毎の性能向上が表示されていて、最初のコミットでは1％ しか改善していないが、次のコミットでは10% 改善していることが分かります。

加えて個々のベンチマークに対するプロファイル結果も見ることが出来ます。

![CodSpeed -- Branches](/images/codspeed-branches-flamegraph.png)

この図は[Flamegraph](https://github.com/brendangregg/FlameGraph)と呼ばれるもので、関数の呼び出し関係を反映した各関数の消費時間を表しています。横幅が計算時間に対する割合を表していて、上にある関数が呼び出し元、下にある関数が呼び出された関数を表しています。ユーザーが関数Aを呼び出しそれが内部で関数Bを呼び出したとき、ユーザーから見たら関数Bの計算時間は関数Aの計算時間の一部なので、このように関数の呼び出し順序と計算時間を同時に表示します。

この図はベンチマークを計測している処理のボトルネックを特定するのに非常に有益ですが、生成するのが面倒なのが問題でした。CodSpeedは自動で生成してくれるので非常に便利です。

なおブランチ毎のベンチマーク結果はGitHub側のPull Requestでもコメントとして表示されます

![CodSpeed -- GitHub](/images/codspeed-github.png)

# CodSpeedの導入

以下の公式ドキュメントに従ってCodSpeedを導入します。

https://docs.codspeed.io/benchmarks/rust

## codspeed-criterion-compat

Rustで有名なベンチマーキングライブラリとして [criterion](https://github.com/bheisler/criterion.rs) がありますが、CodSpeedはcriterionの互換レイヤーを提供しているので、`Cargo.toml`で `criterion` を `codspeed-criterion-compat` に置き換えるだけでCodSpeedを使うことができます。

```toml
[dev-dependencies]
criterion = { package = "codspeed-criterion-compat", version = "*" }
```

criterionの使い方については例えば以下の参照してください

https://zenn.dev/termoshtt/books/b4bce1b9ea5e6853cb07/viewer/criterion#criterion

## GitHub Actionsの設定

まず以下のドキュメントに従ってCodSpeed側にGitHubのリポジトリを登録します。

https://docs.codspeed.io/integrations/providers/github

GitHub ActionsのSecretsに `CODSPEED_TOKEN` を登録すればOKです。最後に[ドキュメントの通り](https://docs.codspeed.io/benchmarks/rust#running-the-benchmarks-in-your-ci)にGitHub Actionsの設定を行えば完了です。

```yaml:.github/workflows/codspeed.yml
name: CodSpeed

on:
  push:
    branches:
      - "main" # or "master"
  pull_request:
  # `workflow_dispatch` allows CodSpeed to trigger backtest
  # performance analysis in order to generate initial data.
  workflow_dispatch:

jobs:
  benchmarks:
    name: Run benchmarks
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup rust toolchain, cache and cargo-codspeed binary
        uses: moonrepo/setup-rust@v1
        with:
          channel: stable
          cache-target: release
          bins: cargo-codspeed

      - name: Build the benchmark target(s)
        run: cargo codspeed build

      - name: Run the benchmarks
        uses: CodSpeedHQ/action@v3
        with:
          run: cargo codspeed run
          token: ${{ secrets.CODSPEED_TOKEN }}
```