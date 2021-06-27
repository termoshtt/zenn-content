---
title: "Perf Tutorial"
emoji: "🔖"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["Linux", "perf"]
published: false
---

[perf][perf]はLinuxにおけるCPUのパフォーマンスカウンタに基づいたプロファイラツールです。CPUにはその上で実行された命令やキャッシュミスの回数、分岐予測の成否の回数を記録しておくレジスタがハードウェア上に存在しています。Linuxカーネルの`perf_events`インタフェースはこれをハードウェアに依らない形で提供しており、`perf`コマンドはこの機能を使ってユーザー空間から操作するツールです。

[perf]: https://perf.wiki.kernel.org/index.php/Main_Page

Install
-------
これはLinuxカーネルの機能であるためLinuxのみが対象です。ほとんどのLinux distributionではカーネル側の`perf_events`インタフェースを使うために特別な設定をする必要は無いはずです。`perf`コマンドはLinuxカーネルの付属ツールとして別名で配布されている事があるので以下にDistribution毎のパッケージ名をまとめておきます：

| Distribution | Package name |
|:------------:|:------------:|
| Ubuntu       | [linux-tools-generic](https://packages.ubuntu.com/focal/linux-tools-generic) |
| Debian       | [linux-perf](https://packages.debian.org/buster/linux-perf) |
| Arch Linux   | [perf](https://archlinux.org/packages/community/x86_64/perf/) |

Docker等のコンテナ仮想化を使った場合ゲスト側はホスト側とLinuxカーネルを共有するため注意が必要です。例えばArchLinux(5.12.13-arch1)上で[ubuntu:20.04][ubuntu-20.04]コンテナを使って次に示すように`linux-tools-generic`をインストールした場合：

```
$ docker run -it --rm ubuntu:20.04
# apt update
# apt install -y linux-tools-generic
# perf
WARNING: perf not found for kernel 5.12.13-arch1
```

の様にLinuxカーネルのバージョンが不一致することに由来するエラーがでます。

[ubuntu-20.04]: https://hub.docker.com/layers/ubuntu/library/ubuntu/20.04/images/sha256-4c8dedb3298beeafd2f3ece9931531009f5622e314fa7803933e892f68114343?context=explore
