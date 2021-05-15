---
title: 既存のC++やCUDAコードと連携する(cc crate)
---

Rustには[cc](https://github.com/alexcrichton/cc-rs)というcrateがあって、`build.rs`内でC/C++のコードをコンパイルしてリンクするのに便利なのですが、これがCUDAもコンパイルできるらしいので試してみた記録（を発掘したので和訳）したものです。

発掘現場：https://github.com/termoshtt/link_cuda_kernel


CUDAサンプルにある`vector_add.cu`を`kernel.cu`としてコピーしてきます。

```cuda:kernel.cu
/** CUDA Kernel Device code */
__global__ void vectorAdd(const float *A, const float *B, float *C, int numElements) {
    int i = blockDim.x * blockIdx.x + threadIdx.x;
    if (i < numElements)
    {
        C[i] = A[i] + B[i];
    }
}

extern "C" {  // CUDAはデフォルトでC++のマングリングを採用するのでそれを抑制する

/** この関数を静的ライブラリとして提供する */
int vectorAdd_main (void) {
    /* 長いので手元のCUDAサンプルを読んでね */
}

} // extern C
```

これを`libvector_add.a` のように静的ライブラリにして`build.rs`でビルドしましょう。

```rust:build.rs
extern crate cc;

fn main() {
    cc::Build::new()
        .cuda(true)               // CUDAをコンパイルします
        .flag("-cudart=shared")   // 以下3行はnvccの引数
        .flag("-gencode")
        .flag("arch=compute_61,code=sm_61")
        .file("kernel.cu")
        .compile("libvector_add.a");  // 静的ライブラリにコンパイルします

    /* Link CUDA Runtime (libcudart.so) */

    // Add link directory
    // - This path depends on where you install CUDA (i.e. depends on your Linux distribution)
    // - This should be set by `$LIBRARY_PATH`
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");

    /* Optional: Link CUDA Driver API (libcuda.so) */

    // println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64/stub");
    // println!("cargo:rustc-link-lib=cuda");
}
```

CUDAにはGPGPU向けに公開されているCUDA Runtime (libcudart.so)に加えてCUDA Driver API (libcuda.so)があるので必要に応じてリンクフラグを追加します。このようにしてリンクした`vectorAdd_main`は普通にCのAPIとして呼び出せます。

```rust
#[link(name = "vector_add", kind = "static")]
extern "C" {
    fn vectorAdd_main();
}

fn main() {
    unsafe {
        vectorAdd_main();
    }
}
```

既存のC++製のCUDAコードの資産を使いつつRust使いたいみたいな状況があればお使いください。

