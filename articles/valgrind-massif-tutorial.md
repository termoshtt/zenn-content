---
title: "valgrind を使って使用メモリ量を調べる"
emoji: "🦀"
type: "tech"
topics: ["valgrind"]
published: true
---

valgrindは良くメモリリークを調べるのに使われますが、使用メモリ量を調べるのにも使えます。

```cpp:test.cpp
#include <vector>

int main() {
  std::vector<std::vector<double>> vecs;

  for(int i=0; i < 100; i++) {
    std::vector<double> a(1000);
    vecs.push_back(a);
  }
  return 0;
}
```

valgrindコマンドは `--tool` で起動するサブプログラムを変更でき、`--tool=massif` とするとheap profilerである [massif](https://www.valgrind.org/docs/manual/ms-manual.html) を起動できます

```bash
g++ test.cpp
valgrind --tool=massif ./a.out
```

これを実行すると `massif.out.{PID}` の様なファイルが出来上がります。この中身は次の様にある時刻における確保されたメモリの情報のスナップショットの情報が順番に保存されています。

```
desc: (none)
cmd: ./a.out
time_unit: i
#-----------
snapshot=0
#-----------
time=0
mem_heap_B=0
mem_heap_extra_B=0
mem_stacks_B=0
heap_tree=empty
#-----------
snapshot=1
#-----------
time=3236472
mem_heap_B=72704
mem_heap_extra_B=8
mem_stacks_B=0
heap_tree=empty
#-----------
snapshot=2
#-----------
time=3258268
mem_heap_B=88728
mem_heap_extra_B=40
mem_stacks_B=0
heap_tree=detailed
n3: 88728 (heap allocation functions) malloc/new/new[], --alloc-fns, etc.
 n1: 72704 0x491B3FA: pool (eh_alloc.cc:123)
  n1: 72704 0x491B3FA: __static_initialization_and_destruction_0 (eh_alloc.cc:262)
   n1: 72704 0x491B3FA: _GLOBAL__sub_I_eh_alloc.cc (eh_alloc.cc:338)
    n1: 72704 0x400FE8D: call_init.part.0 (in /usr/lib/ld-2.33.so)
     n1: 72704 0x400FF77: _dl_init (in /usr/lib/ld-2.33.so)
      n0: 72704 0x40010C9: ??? (in /usr/lib/ld-2.33.so)
 n1: 16000 0x10A923: __gnu_cxx::new_allocator<double>::allocate(unsigned long, void const*) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
  n1: 16000 0x10A587: std::allocator_traits<std::allocator<double> >::allocate(std::allocator<double>&, unsigned long) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
   n1: 16000 0x10A223: std::_Vector_base<double, std::allocator<double> >::_M_allocate(unsigned long) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
    n1: 16000 0x109D40: std::_Vector_base<double, std::allocator<double> >::_M_create_storage(unsigned long) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
     n2: 16000 0x10973C: std::_Vector_base<double, std::allocator<double> >::_Vector_base(unsigned long, std::allocator<double> const&) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
      n1: 8000 0x109494: std::vector<double, std::allocator<double> >::vector(unsigned long, std::allocator<double> const&) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
       n0: 8000 0x10921A: main (in /home/teramura/tmp/cpp_alloc_eval/a.out)
      n1: 8000 0x10A2FA: std::vector<double, std::allocator<double> >::vector(std::vector<double, std::allocator<double> > const&) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
       n1: 8000 0x109E4C: void __gnu_cxx::new_allocator<std::vector<double, std::allocator<double> > >::construct<std::vector<double, std::allocator<double> >, std::vector<double, std::allocator<double> > const&>(std::vector<double, std::allocator<double> >*, std::vector<double, std::allocator<double> > const&) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
        n1: 8000 0x10985B: void std::allocator_traits<std::allocator<std::vector<double, std::allocator<double> > > >::construct<std::vector<double, std::allocator<double> >, std::vector<double, std::allocator<double> > const&>(std::allocator<std::vector<double, std::allocator<double> > >&, std::vector<double, std::allocator<double> >*, std::vector<double, std::allocator<double> > const&) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
         n1: 8000 0x1099C3: void std::vector<std::vector<double, std::allocator<double> >, std::allocator<std::vector<double, std::allocator<double> > > >::_M_realloc_insert<std::vector<double, std::allocator<double> > const&>(__gnu_cxx::__normal_iterator<std::vector<double, std::allocator<double> >*, std::vector<std::vector<double, std::allocator<double> >, std::allocator<std::vector<double, std::allocator<double> > > > >, std::vector<double, std::allocator<double> > const&) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
          n1: 8000 0x109585: std::vector<std::vector<double, std::allocator<double> >, std::allocator<std::vector<double, std::allocator<double> > > >::push_back(std::vector<double, std::allocator<double> > const&) (in /home/teramura/tmp/cpp_alloc_eval/a.out)
           n0: 8000 0x109239: main (in /home/teramura/tmp/cpp_alloc_eval/a.out)
 n0: 24 in 1 place, below massif's threshold (1.00%)
#-----------
snapshot=3
#-----------
...
```

これを時系列で表示するに `ms_print` というコマンドがvalgrindに付属しているはずですが、テキストベースで表示するため見づらいので、[massif-viewer](https://github.com/KDE/massif-visualizer) を使いましょう

![massif-viewer-screenshot1.png](https://raw.githubusercontent.com/termoshtt/zenn-content/main/articles/massif-viewer-screenshot1.png)

縦軸が確保しているメモリ量、横軸は経過時間です。横軸の時間はCPU命令が実行された数で表示する様ですが、短いプログラムでは `main` に到達するまでの時間が長いのでほとんどが右端によってしまいます。横軸として替わりに確保・開放されたメモリ量を使うオプション `--time-unit=B` も存在します

![massif-viewer-screenshot2.png](https://raw.githubusercontent.com/termoshtt/zenn-content/main/articles/massif-viewer-screenshot2.png)

