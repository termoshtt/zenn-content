---
title: "Perf Tutorial"
emoji: "🔖"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: ["Linux", "perf"]
published: false
---

[perf][perf]はLinuxにおけるCPUのパフォーマンスカウンタに基づいたプロファイラツールです。CPUにはその上で実行された命令やキャッシュミスの回数、分岐予測の成否の回数を記録しておくレジスタがハードウェア上に存在しています。典型的には各イベント毎にレジスタの値を一つインクリメントします。例えば命令を一つ実行したら`instructions`のカウンタを一つ増やし、L2キャッシュミスが発生したら`l2_cache_miss`のカウンタを一つ増やすといった方法で実装されています。我々は実際のCPUの内部状態について基本的に知り得ないのでこれが一次情報になります。これはCPUのアーキテクチャに強く依存する為CPUベンダーから提供されますが、Linuxカーネルの`perf_events`インタフェースはこれをハードウェアに依らない形で提供してくれています。`perf`コマンドはこの機能を使ってユーザー空間から操作するツールです。

[perf]: https://perf.wiki.kernel.org/index.php/Main_Page

perfには大きく分けて次の機能があります：

- プロセス全体でのパフォーマンス計測([Counting][counting], `perf stat`)
- 命令レベルでのプロファイリング([Sampling][sampling], `perf record`)
  - シンボルレベルでの解析(`perf report`)
  - コード行レベルでの解析(`perf annotate`)

CPUのパフォーマンスカウンタはあくまで事象に応じてカウンタを増やす事しかしてくれないので集計はperfが行います。[Countingモード][counting]ではperfはプロセス中に発生したイベントの回数を単に集めていきます。[Samplingモード][sampling]ではperfはCPUのパフォーマンスカウンタがオーバーフローした際の割り込みを利用してその時のCPUの情報、特に命令のポインタ(プログラムカウンタ)を記録します。これにより実行時間の増加をある程度抑えたままどの命令を実行する時にどのイベントが発生しているかを統計的に評価できます。この情報は一旦`perf.data`ファイルに記録され、デバッグ情報を元にシンボル名や行の位置に翻訳されます。

[ftrace]: https://www.kernel.org/doc/html/latest/trace/ftrace.html
[counting]: https://perf.wiki.kernel.org/index.php/Tutorial#Counting_with_perf_stat
[sampling]: https://perf.wiki.kernel.org/index.php/Tutorial#Sampling_with_perf_record

Install
-------
これはLinuxカーネルの機能であるためLinuxのみが対象です。ほとんどのLinux distributionではカーネル側の`perf_events`インタフェースを使うために特別な設定をする必要は無いはずです。`perf`コマンドはLinuxカーネルの付属ツールとして別名で配布されている事があるので以下にDistribution毎のパッケージ名をまとめておきます：

| Distribution | Package name |
|:------------:|:------------:|
| Ubuntu       | [linux-tools-generic](https://packages.ubuntu.com/focal/linux-tools-generic) |
| Debian       | [linux-perf](https://packages.debian.org/buster/linux-perf) |
| Arch Linux   | [perf](https://archlinux.org/packages/community/x86_64/perf/) |

Docker等のコンテナ仮想化を使った場合ゲスト側はホスト側とLinuxカーネルを共有するため注意が必要です。例えばArchLinux(5.12.13-arch1)上で[`ubuntu:20.04`][ubuntu-20.04]コンテナを使って次に示すように`linux-tools-generic`をインストールした場合：

```
$ docker run -it --rm ubuntu:20.04
# apt update
# apt install -y linux-tools-generic
# perf
WARNING: perf not found for kernel 5.12.13-arch1
```

の様にLinuxカーネルのバージョンが不一致することに由来するエラーがでます。

[ubuntu-20.04]: https://hub.docker.com/layers/ubuntu/library/ubuntu/20.04/images/sha256-4c8dedb3298beeafd2f3ece9931531009f5622e314fa7803933e892f68114343?context=explore

### 非特権ユーザーからCPUイベントへのアクセスを許可する

セキュリティ上の理由で非特権ユーザー(`CAP_PERFMON`を持たないユーザー)からアクセスできる`perf_events`の情報は制限されています。これは[perf_event_paranoid](https://www.kernel.org/doc/html/latest/admin-guide/sysctl/kernel.html#perf-event-paranoid)パラメータで制御されており、デフォルト値は`2`となっています：

| 値 | 権限 |
|:-------------------:|:-----|
| -1  | 全てのユーザーから全てのイベントへのアクセスを許可します |
| >=0 | `CAP_PERFMON`を持たないユーザーから`ftrace function tracepoint`と`raw tracepoint`へのアクセスを禁止します |
| >=1 | `CAP_PERFMON`を持たないユーザーからCPUイベントへのアクセスを禁止します |
| >=2 | `CAP_PERFMON`を持たないユーザーにカーネルのプロファイリングを禁止します|

現在の設定を確認するには`/proc/sys/kernel/perf_event_paranoid`ファイルを読みます：

```
$ cat /proc/sys/kernel/perf_event_paranoid
2
```

一時的に上書きするならこのファイルを書き換えます：

```
# echo -1 > /proc/sys/kernel/perf_event_paranoid
```

Links
------

- [perf Examples](http://www.brendangregg.com/perf.html)
  - [Systems Performance: Enterprise and the Cloud, 2nd Edition (2020)](http://www.brendangregg.com/systems-performance-2nd-edition-book.html)及び[BPF Performance Tools](http://www.brendangregg.com/bpf-performance-tools-book.html)の著者(Brendan Gregg氏)による用例集と機構の解説
- [Perf Wiki](https://perf.wiki.kernel.org/index.php/Main_Page)
  - Linuxカーネルの公式Wiki。この記事もこのWikiの[Tutorial](https://perf.wiki.kernel.org/index.php/Tutorial)の内容に基づいて書かれている。
