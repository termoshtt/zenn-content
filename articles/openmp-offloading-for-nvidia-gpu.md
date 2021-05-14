---
title: "OpenMP Offloading for NVIDIA GPU"
emoji: "🦀"
type: "tech"
topics: ["OpenMP", "OpenMPOffloading", "OpenACC", "GPU", "LLVM"]
published: true
---

OpenMP Offloadingを使うためではなく、どうやって動作しているのかを理解するためのメモ

Programming model by NVIDIA
----------------------------
- 最適化されたライブラリの使用 (cuBLAS, cuDNN等)
  - 特別なコンパイラ無しで使用できる、用意されたコードしかGPU上で動作させられないので柔軟性は低い
- OpenACC言語拡張を用いて既存のコードをGPU向けに自動的に変換する
  - for文に `#pragma acc` を追加すればGPU上で動作するようになる
- 独自言語拡張CUDA/C++ を用いる
  - NVIDIAが提供している全ての機能にアクセスできる
- [C++17 Parallel Algorithm, Fortran 2008 concurrent を GPU で実行する](https://developer.nvidia.com/blog/accelerating-standard-c-with-gpus-using-stdpar/)
    - nvc++, nvfortran (NVIDIA HPC SDK) で標準のC++ / Fortranをコンパイルする

How NVIDIA GPU / CUDA works?
-----------------------------
- 何かしらの方法でGPU上で実行されるアセンブラ ([PTX](https://docs.nvidia.com/cuda/parallel-thread-execution/index.html)) を生成し、それをGPU上に転送し、起動する必要がある
- ハードウェアとしてのGPUはカーネル (Linux, Windows) にロードされたNVIDIAドライバが面倒を見る
    - macOSはカーネルドライバが提供されないので全く使用できない
- ユーザー空間からカーネルへの命令はCUDA Runtimeを経由して行われる
    - CPUの場合のC Runtime (glibc, CRT) のようなもの
- CUDA Runtimeには高水準な [Runtime API (libcudart.so)](https://docs.nvidia.com/cuda/cuda-runtime-api/index.html) と低水準な [Driver API (libcuda.so)](https://docs.nvidia.com/cuda/cuda-driver-api/index.html) がある
    - 基本的にRuntime APIはDriver APIを呼び出して動いているはず？
- CUDA/C++ compiler (nvcc) はプログラム中の `__device__` や `__global__` 修飾されている関数をPTXにコンパイルし、カーネル呼び出し構文 `kernel<<<block, thread>>>` をCUDA Runtimeの呼び出しに置き換える
    - nvccの機能をライブラリとして提供する [NVRTC](https://docs.nvidia.com/cuda/nvrtc/index.html) というものも存在する
- PTXさえ生成出来れば特別なコンパイラ無しに、それをCUDA Runtimeを用いてGPU上で実行させることが出来る
    - [PTX  ISA reference](https://docs.nvidia.com/cuda/parallel-thread-execution/index.html) を読みながら手で書く
    - GCC, LLVMはPTXをターゲットとしてコード生成する事が可能
        -  ビルド時にNVPTXが有効になっているclangがあれば `clang --target=nvptx64-nvidia-cuda -S test.cpp` で `test.s` にPTXが出来る
        - 例えば [Rust](https://qiita.com/termoshtt/items/b98d5c46ab9c1ab1f7b6) や [Julia](https://github.com/JuliaGPU/CUDA.jl), あるいは [Python](https://numba.readthedocs.io/en/stable/cuda/index.html) からLLVM IRを経由してPTXへコンパイルすることが出来る
        - OpenMP Offloadingもこの仕組みを使う

OpenMP Offloading
------------------
- GPU等のアクセラレータをOpenMPコンパイラから使えるようにしたもの
  - OpenMPとはC/C++/Fortran向けスレッド並列化用言語拡張 (`#pragma omp`)
  - OpenMP OffloadingはCUDA/C++ コンパイラ (nvcc) と同じようにユーザーコードからGPU用のアセンブリを生成し、それをGPUに転送・実行をするための処理を記述するための言語拡張
- [OpenMP Offloading](https://www.openmp.org/) と [OpenACC](https://www.openacc.org/) は別の言語拡張
  - コンパイラの裏側では同じ実装だったりする
- OpenMPはコンパイラでの言語拡張なので各コンパイラ毎に実装する
  - OpenMP OffloadingはOpenMP 4.0 (2013/7) で追加
    - 最新は [OpenMP 5.1 (2020/11)](https://www.openmp.org/press-release/openmp-arb-releases-openmp-5-1/)
  - OpenMPの代表的な実装としてGCC (libgomp), LLVM (libomp), MSVC , NVC++ (旧PGI) がある
    - 実装状況はまちまち
     - [MSVCはまだ OpenMP 3.0 もサポートしてない](https://docs.microsoft.com/ja-jp/cpp/build/reference/openmp-enable-openmp-2-0-support?view=msvc-160)
- GCC, LLVM共にNVIDIA GPUの制御にはCUDA Runtimeを使う
  - コンパイラ (gcc, clang) はデバイスコードをPTXにコンパイルしてオブジェクトファイルに埋め込み、ランタイム (libgomp, libomp) が実行時にそれをCUDA RuntimeでGPU上に展開・実行させる
  - リンカはホストコードのリンクに加えて、デバイスコード (PTX) をリンクする必要がある
      - ここは標準化されている？
  - ldはPTXをリンク出来ないのでptxas (CUDAの一部) を使うか別実装を使う
- GCCはPTXをオブジェクトに埋め込んで [nvptx-tools](https://github.com/MentorEmbedded/nvptx-tools) を使ってリンクする
    - [Offloading Support in GCC](https://gcc.gnu.org/wiki/Offloading) が詳しい
    - デバイスコード用のlibcやlibmはglibcでなく [組み込み向けの newlib](https://ja.wikipedia.org/wiki/Newlib) を使う
- LLVMは調査中（挙動が結構違うので多分中身も結構違う）
