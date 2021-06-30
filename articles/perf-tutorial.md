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

CPUのパフォーマンスカウンタはあくまで事象に応じてカウンタを増やす事しかしてくれないので集計はperfが行います。

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

以下は特に断りが無ければ`perf_event_paranoid=-1`での結果を示します。

Counting mode (`perf stat`)
-------------
まず`perf stat`から見ていきましょう。このコマンドは引数でもらったサブコマンドを実行して、最後にそのプロセスを通してのパフォーマンスカウンタの合計値を出力します：

```
$ perf stat dd if=/dev/zero of=/dev/null count=1000000
1000000+0 レコード入力
1000000+0 レコード出力
512000000 bytes (512 MB, 488 MiB) copied, 0.492335 s, 1.0 GB/s

 Performance counter stats for 'dd if=/dev/zero of=/dev/null count=1000000':

            491.90 msec task-clock                #    0.997 CPUs utilized  
                49      context-switches          #    0.100 K/sec          
                 0      cpu-migrations            #    0.000 K/sec          
               116      page-faults               #    0.236 K/sec          
     2,212,164,742      cycles                    #    4.497 GHz            
     2,112,911,021      instructions              #    0.96  insn per cycle 
       450,714,861      branches                  #  916.270 M/sec          
         4,496,804      branch-misses             #    1.00% of all branches

       0.493196734 seconds time elapsed

       0.199588000 seconds user
       0.292739000 seconds sys
```

この例では`dd if=/dev/zero of=/dev/null count=1000000`をサブプロセスとして起動していて最初の3行はその出力です。5行目からが`perf stat`による統計情報の表示です。`perf stat`に特に何を表示するかを指定していないためデフォルト設定の量が集計され表示されています。`#`の右に表示されているのは計測値から計算されたメトリクスです。

同じコマンドを`perf_event_paranoid=2`で実行してみましょう：

```
$ cat /proc/sys/kernel/perf_event_paranoid
2
$ perf stat dd if=/dev/zero of=/dev/null count=1000000
1000000+0 レコード入力
1000000+0 レコード出力
512000000 bytes (512 MB, 488 MiB) copied, 0.491174 s, 1.0 GB/s

 Performance counter stats for 'dd if=/dev/zero of=/dev/null count=1000000':

            491.60 msec task-clock:u              #    0.999 CPUs utilized  
                 0      context-switches:u        #    0.000 K/sec          
                 0      cpu-migrations:u          #    0.000 K/sec          
               112      page-faults:u             #    0.228 K/sec          
       209,743,680      cycles:u                  #    0.427 GHz            
       298,844,447      instructions:u            #    1.42  insn per cycle 
        70,289,118      branches:u                #  142.981 M/sec          
            45,612      branch-misses:u           #    0.06% of all branches

       0.492151900 seconds time elapsed

       0.194119000 seconds user
       0.297878000 seconds sys
```

表示されるイベント名にユーザー空間での値であることを示す`:u`がついて値が変化している事が分かります。`/dev/zero`や`/dev/null`はカーネルモジュールで作られるものであり、そこからの読み出しはカーネル内の操作になるため一般ユーザーではこの部分の処理のカウントはとれず合計値が小さい値になります。この`:u`の部分は他に次のものがあります：

| Modifiers | Description | Example |
|:---------:|:------------|:--------|
| u | monitor at priv level 3, 2, 1 (user) | event:u |
| k | monitor at priv level 0 (kernel) | event:k |
| h | monitor hypervisor events on a virtualization environment | event:h |
| H | monitor host machine on a virtualization environment | event:H |
| G | monitor guest machine on a virtualization environment | event:G |


特定のイベントだけ集計するには`-e`(`--event`)フラグを使います：

```
$ perf stat -e instructions:u dd if=/dev/zero of=/dev/null count=1000000
1000000+0 レコード入力
1000000+0 レコード出力
512000000 bytes (512 MB, 488 MiB) copied, 0.486783 s, 1.1 GB/s

 Performance counter stats for 'dd if=/dev/zero of=/dev/null count=1000000':

       298,838,248      instructions:u

       0.487783886 seconds time elapsed

       0.166980000 seconds user
       0.320599000 seconds sys
```

引数にとれるイベントのリストは`perf list`で確認できます。

同じ処理を複数回繰り替えして統計をとるには`-r`(`--repeat`)を使います：

```
$ perf stat -r 5 sleep 1

 Performance counter stats for 'sleep 1' (5 runs):

       0.36 msec task-clock        #    0.000 CPUs utilized   ( +-  3.21% )
          1      context-switches  #    0.003 M/sec           
          0      cpu-migrations    #    0.000 K/sec           
         66      page-faults       #    0.185 M/sec           ( +-  1.00% )
  1,263,237      cycles            #    3.532 GHz             ( +-  1.05% )
    967,275      instructions      #    0.77  insn per cycle  ( +-  0.62% )
    203,808      branches          #  569.824 M/sec           ( +-  0.62% )
      6,567      branch-misses     #    3.22% of all branches ( +-  1.06% )

  1.0009503 +- 0.0000363 seconds time elapsed  ( +-  0.00% )
```

これは`sleep 1`を5回実行するため約5秒かかります。各統計量について平均値と分散の値を出力してくれます。

Sampling mode (`perf record`)
--------------
SamplingモードではperfはCPUのパフォーマンスカウンタがオーバーフローした際の割り込み(PMU interrupt)を利用してその時のCPUの情報、特に命令のポインタ(プログラムカウンタ)を記録します。これにより実行時間の増加をある程度抑えたままどの命令を実行する時にどのイベントが発生しているかを統計的に評価できます。この情報は一旦`perf.data`ファイルに記録され、デバッグ情報を元にシンボル名や行の位置に翻訳されます。

Samplingを行うには`perf record`コマンドを使います：

```
$ perf record dd if=/dev/zero of=/dev/null count=1000000
1000000+0 レコード入力
1000000+0 レコード出力
512000000 bytes (512 MB, 488 MiB) copied, 0.487976 s, 1.0 GB/s
[ perf record: Woken up 1 times to write data ]
[ perf record: Captured and wrote 0.096 MB perf.data (1932 samples) ]
```

これで`perf.data`ファイルが作成されました。もし既に`perf.data`が存在している場合は古いものを`perf.data.old`に変更して新しく`perf.data`を作ります。`-o`(`--output`)フラグで出力ファイル名を変更することもできます。

`perf record`はデフォルトではコールグラフの情報を収集しません。例えば関数`a()`と`b()`がそれぞれ関数`c()`を呼び出しているとき、コールグラフ無しでは`c()`の中に居る事しか分からないため`a()`経由の分と`b()`経由の分を区別することが出来ません。コールグラフを収集させるには`-g`オプションを使います：

```
$ perf record -g -o perf.data.g dd if=/dev/zero of=/dev/null count=1000000
1000000+0 レコード入力
1000000+0 レコード出力
512000000 bytes (512 MB, 488 MiB) copied, 0.488534 s, 1.0 GB/s
[ perf record: Woken up 1 times to write data ]
[ perf record: Captured and wrote 0.202 MB perf.data.g (1951 samples) ]
```

以降の作業の為に別名`perf.data.g`として保存しました。コールグラフの取得はフレームポインタを使う方法(`fp`)とデバッグ情報([DWARF][dwarf])を使う方法(`dwarf`)、さらにLast Branch Record (LBR)を使う方法(`lbr`)があるようです。詳しくは`man perf-record`を参照してください。

[dwarf]: http://dwarfstd.org/

### Symbol level analysis (`perf report`)

`perf.data`はバイナリ形式になっているので直接は読めませんが`perf record`には中身を読み出す機能が備わっています：

```
$ perf report -D --header-only
# ========
# captured on    : Wed Jun 30 18:02:06 2021
# header version : 1
# data offset    : 336
# data size      : 100528
# feat offset    : 100864
# hostname : my_super_machine_name
# os release : 5.12.13-arch1-2
# perf version : 5.12.g9f4ad9e425a1
# arch : x86_64
# nrcpus online : 12
# nrcpus avail : 12
# cpudesc : Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz
# cpuid : GenuineIntel,6,158,10
# total memory : 49255656 kB
# cmdline : /usr/bin/perf record dd if=/dev/zero of=/dev/null count=1000000 
# event : name = cycles, , id = { 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435 }, size = 120, { sample_period, sample_freq } = 4000, sample_type = IP|TID|TIME|PERIOD, read_format = ID, disabled = 1, inherit = 1, mmap = 1, comm = 1, freq = 1, enable_on_exec = 1, task = 1, precise_ip = 3, sample_id_all = 1, exclude_guest = 1, mmap2 = 1, comm_exec = 1, ksymbol = 1, bpf_event = 1
# pmu mappings: intel_pt = 8, software = 1, power = 20, uncore_cbox_4 = 15, uprobe = 7, uncore_imc = 10, cpu = 4, cstate_core = 18, uncore_cbox_2 = 13, breakpoint = 5, uncore_cbox_0 = 11, tracepoint = 2, cstate_pkg = 19, uncore_arb = 17, kprobe = 6, uncore_cbox_5 = 16, msr = 9, uncore_cbox_3 = 14, uncore_cbox_1 = 12
# time of first sample : 44812.047071
# time of last sample : 44812.535261
# sample duration :    488.190 ms
# cpu pmu capabilities: branches=32, max_precise=3, pmu_name=skylake
# missing features: TRACING_DATA BRANCH_STACK GROUP_DESC AUXTRACE STAT CLOCKID DIR_FORMAT COMPRESSED CLOCK_DATA 
# ========
```

このようなヘッダ情報に加えてサンプリングの結果が保存されています。

```
$ perf report -D

0x150 [0x38]: event: 79
.
. ... raw event: size 56 bytes
.  0000:  4f 00 00 00 00 00 38 00 1f 00 00 00 00 00 00 00  O.....8.........
.  0010:  8c d2 a1 22 00 00 00 00 20 aa d0 f6 a7 20 00 00  .ҡ"....  ..
.  0020:  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................
.  0030:  01 00 00 00 00 00 00 00                          ........        

0 0x150 [0x38]: PERF_RECORD_TIME_CONV: unhandled!

0x188 [0x50]: event: 1
.
. ... raw event: size 80 bytes
.  0000:  01 00 00 00 01 00 50 00 ff ff ff ff 00 00 00 00  ......P.....
.  0010:  00 00 40 a2 ff ff ff ff f7 20 e0 00 00 00 00 00  ..@ .....
.  0020:  00 00 40 a2 ff ff ff ff 5b 6b 65 72 6e 65 6c 2e  ..@[kernel.
.  0030:  6b 61 6c 6c 73 79 6d 73 5d 5f 74 65 78 74 00 00  kallsyms]_text..
.  0040:  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00  ................

0 0x188 [0x50]: PERF_RECORD_MMAP -1/0: [0xffffffffa2400000(0xe020f7) @ 0xffffffffa2400000]: x [kernel.kallsyms]_text

(以下同じような出力が続く)
```

この生情報を見ても分からないので`perf report`に集計してもらいましょう：

```
$ perf report --stdio

# Total Lost Samples: 0
#
# Samples: 1K of event 'cycles'
# Event count (approx.): 2204433912
#
# Overhead  Command  Shared Object      Symbol                             
# ........  .......  .................  ...................................
#
    26.31%  dd       [kernel.kallsyms]  [k] syscall_return_via_sysret
    20.64%  dd       [kernel.kallsyms]  [k] __entry_text_start
     6.22%  dd       [kernel.kallsyms]  [k] __fsnotify_parent
     4.18%  dd       [kernel.kallsyms]  [k] __clear_user
     2.83%  dd       [kernel.kallsyms]  [k] __audit_syscall_exit
     2.78%  dd       [kernel.kallsyms]  [k] syscall_exit_to_user_mode
     2.68%  dd       [kernel.kallsyms]  [k] __fget_light
     2.34%  dd       libc-2.33.so       [.] read
     2.31%  dd       [kernel.kallsyms]  [k] syscall_enter_from_user_mode
     2.09%  dd       [kernel.kallsyms]  [k] vfs_write
     1.96%  dd       [kernel.kallsyms]  [k] read_zero
     1.94%  dd       libc-2.33.so       [.] __GI___libc_write
     1.89%  dd       [kernel.kallsyms]  [k] entry_SYSCALL_64_after_hwframe
     1.84%  dd       libc-2.33.so       [.] __memmove_avx_unaligned_erms
     1.69%  dd       [kernel.kallsyms]  [k] vfs_read
     1.44%  dd       [kernel.kallsyms]  [k] entry_SYSCALL_64_safe_stack
     1.30%  dd       [kernel.kallsyms]  [k] __audit_syscall_entry
     1.19%  dd       [kernel.kallsyms]  [k] syscall_trace_enter.constprop.0
     1.14%  dd       [kernel.kallsyms]  [k] ksys_read
(以下省略)
```

`perf report`は標準出力がTTYだとTUIを立ち上げるので`--stdio`の結果を示しています。

- `Overhead`列の値がそのシンボル中にサンプルが存在した割合で、そのシンボル中で消費された時間に対応します。`perf.data`の方にはコールグラフがついていないのでどのような経路でそのシンボルに入ってるか分からないので単純に合算した値が出力されます。
- `Command`列は実行ファイルの名前になっています。プロセスに対して`perf record`しているのでここは常にコマンド名になりますが、perfは`CPU`全体でSamplingする事も出きるのでその場合はここに個別のコマンドが表示されます。
- `Shared Object`は実際にシンボルが存在する共有ライブラリを表示しています
- `Symbol`にはシンボル名が表示されています。先頭の`[k]`はカーネル内のものであることを、`[.]`はユーザーレベルのシンボルであることを示します

次にコールグラフの情報を保存した場合を見ていきます。`perf report`は`-i`(`--input`)で解析する`perf.data`を変更できます：

```
$ perf report --stdio -i perf.data.g

# Total Lost Samples: 0
#
# Samples: 1K of event 'cycles'
# Event count (approx.): 2187200828
#
# Children      Self  Command  Shared Object      Symbol                                
# ........  ........  .......  .................  ......................................
#
    96.14%     0.00%  dd       [unknown]          [k] 0000000000000000
            |
            ---0
               |          
               |--51.57%--read
               |          |          
               |          |--23.29%--entry_SYSCALL_64_after_hwframe
               |          |          |          
               |          |          |--17.63%--do_syscall_64
               |          |          |          |          
               |          |          |          |--15.43%--ksys_read
               |          |          |          |          |          
               |          |          |          |          |--13.43%--vfs_read
               |          |          |          |          |          |          
               |          |          |          |          |          |--6.96%--read_zero
               |          |          |          |          |          |          |          
               |          |          |          |          |          |           --4.76%--__clear_user
               |          |          |          |          |          |          
               |          |          |          |          |          |--3.90%--__fsnotify_parent
               |          |          |          |          |          |          
               |          |          |          |          |           --1.18%--security_file_permission
               |          |          |          |          |          
               |          |          |          |           --1.02%--__fdget_pos
               |          |          |          |                     |          
               |          |          |          |                      --0.87%--__fget_light
               |          |          |          |          
               |          |          |          |--0.92%--syscall_trace_enter.constprop.0
               |          |          |          |          
               |          |          |           --0.67%--__x64_sys_read
               |          |          |          
               |          |           --4.07%--syscall_exit_to_user_mode
               |          |                     |          
               |          |                      --2.48%--syscall_exit_work
               |          |                                |          
               |          |                                 --1.97%--__audit_syscall_exit
               |          |          
               |          |--13.27%--syscall_return_via_sysret
               |          |          
               |          |--11.57%--__entry_text_start
               |          |          
               |           --1.74%--entry_SYSCALL_64_safe_stack
               |          
               |--42.06%--__GI___libc_write
               |          |          
               |          |--16.71%--entry_SYSCALL_64_after_hwframe
               |          |          |          
               |          |          |--11.25%--do_syscall_64
               |          |          |          |          
               |          |          |          |--8.59%--ksys_write
               |          |          |          |          |          
               |          |          |          |          |--5.77%--vfs_write
               |          |          |          |          |          |          
               |          |          |          |          |          |--2.13%--__fsnotify_parent
               |          |          |          |          |          |          
               |          |          |          |          |           --1.33%--write_null
               |          |          |          |          |          
               |          |          |          |           --1.49%--__fdget_pos
               |          |          |          |                     |          
               |          |          |          |                      --1.33%--__fget_light
               |          |          |          |          
               |          |          |          |--1.44%--syscall_trace_enter.constprop.0
               |          |          |          |          
               |          |          |           --0.71%--__x64_sys_write
               |          |          |          
               |          |           --3.64%--syscall_exit_to_user_mode
               |          |                     |          
               |          |                      --2.66%--syscall_exit_work
               |          |                                |          
               |          |                                 --2.51%--__audit_syscall_exit
               |          |          
               |          |--11.92%--__entry_text_start
               |          |          
               |          |--9.80%--syscall_return_via_sysret
               |          |          
               |           --2.04%--entry_SYSCALL_64_safe_stack
               |          
                --0.51%--0x55a1d35518cf
```

最初のツリーだけを表示しています。先程の結果と違い、`Children`列と`dd`コマンドのうち`libc`の`read`に使っている時間が`51%`で`__GI___libc_write`(これは`write`システムコールのラッパー)に使っている時間が`42%`です。`dd`なので読み込みと書き込みでほとんどの時間を使っているのは正しそうですね。そこからさらに`read`と`write`の内訳がグラフになっています。

このグラフだと項目が増えると見づらい為、これを一枚のSVGにまとめたものが[flamegraph](https://github.com/brendangregg/FlameGraph)です：

```
$ perf script -i perf.data.g | stackcollapse-perf.pl | flamegraph.pl > out.svg
```

flamegraphは他にも`dtrace`などの様々なログに対応しており、`stackcollapse-xxx.pl`で一旦ログを共通の形式に変換して`flamegraph.pl`でSVGを生成しているようです。このSVGはインタラクティブに動作しカーソルを合わせると詳細が表示されクリックするとその部分にズームします。

[![flamegraph](https://raw.githubusercontent.com/termoshtt/zenn-content/perf-tutorial/articles/perf-tutorial.svg)](https://raw.githubusercontent.com/termoshtt/zenn-content/perf-tutorial/articles/perf-tutorial.svg)

Links
------

- [perf Examples](http://www.brendangregg.com/perf.html)
  - [Systems Performance: Enterprise and the Cloud, 2nd Edition (2020)](http://www.brendangregg.com/systems-performance-2nd-edition-book.html)及び[BPF Performance Tools](http://www.brendangregg.com/bpf-performance-tools-book.html)の著者(Brendan Gregg氏)による用例集と機構の解説
- [Perf Wiki](https://perf.wiki.kernel.org/index.php/Main_Page)
  - Linuxカーネルの公式Wiki。この記事もこのWikiの[Tutorial](https://perf.wiki.kernel.org/index.php/Tutorial)の内容に基づいて書かれている。
