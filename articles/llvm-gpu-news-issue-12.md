---
title: "LLVM GPU News #12"
emoji: "🐉"
type: "tech"
topics: ["LLVM", "GPU"]
published: false
---

この記事は[LLVM GPU News #12](https://llvm-gpu-news.github.io/2021/05/21/issue-12.html)の和訳になります。
<!-- Welcome to LLVM GPU News, a bi-weekly newsletter on all the GPU things under the LLVM umbrella. This issue covers the period from April 30 to May 20 2021. -->
LLVM GPU Newsは隔週で発行されるLLVM傘下のGPUに関する事をまとめたニュース記事です。この巻では2021/4/30から5/20までの内容を扱います。
<!-- We welcome your feedback and suggestions. Let us know if we missed anything interesting, or want us to bring attention to your (sub)project, revisions under review, or proposals. Please see the bottom of the page for details on how to submit suggestions and contribute. -->
フィードバックや提案を歓迎しています。もし我々が何か面白いことを見逃していたり、あなたのプロジェクトやレビュー中のパッチあるいは提案について我々に知ってほしいときはぜひ教えてください。提案や貢献についてはこのページの末尾を見てください（訳注：原文とスタイルが異なるので最後にリンクを追加しました）。

## 業界ニュース・カンファレンストーク
<!-- Industry News and Conference Talks -->

<!-- *  The X.Org Developers' Conference 2021 is now [accepting submissions and is open for registration](https://lists.freedesktop.org/archives/wayland-devel/2021-May/041828.html). The conference will happen virtually on September 15-17. There is no registration fee. -->
* X.Org開発者会議2021が現在
*  Portable Computing Language (PoCL) v1.7, a portable open-source OpenCL implementation, [has been released](https://lists.llvm.org/pipermail/llvm-dev/2021-May/150654.html). The new release features Clang/LLVM 12.0 support and can execute SPIR-V binaries on CPUs. The project is looking for people interested in taking the roles of ARM and RISC-V CPU maintainers.
*  ROCm 4.2 [has been released](https://github.com/RadeonOpenCompute/ROCm/blob/f7b3a38d4988d41247ded9d4fdb3a405e90cc089/AMD_ROCm_Release_Notes_v4.2.pdf). The new HIP enhancements include target platform macros for AMD and Nvidia, platform-specific include directories, and extended support for Stream Memory Operations that enable direct synchronization between network nodes and GPU.
*  Nvidia proposed a new Vulkan extension to allow application to import CUDA binaries ([cubin ELF files](https://docs.nvidia.com/cuda/cuda-binary-utilities/index.html#cuda-binary)) and execute them.

## Links
このセクションは和訳時に追加されたものです：

- [subscribe via RSS](https://llvm-gpu-news.github.io/feed.xml)
- [llvm-gpu-news/llvm-gpu-news.github.io - GitHub](https://github.com/llvm-gpu-news/llvm-gpu-news.github.io)
  - [Opening an issue](https://github.com/llvm-gpu-news/llvm-gpu-news.github.io/issues/new)
  - [Submiting a Pull Request](https://github.com/llvm-gpu-news/llvm-gpu-news.github.io/tree/main/docs/_posts)
