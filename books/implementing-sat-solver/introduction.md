---
title: "はじめに"
---

この本は私が数理最適化への応用のためにSATソルバーの技術を学んだ際のノートです。この本を読めばSATソルバーを十分に理解できる、という構成にはなっておらず、適切に参考文献を併用する必要があります。この本の目的は理解すべき事項を整理して知識の全体像の概略をつかみ、適切な参考文献を見つける手助けをすることです。

参考文献
========

- [The Art of Computer Programming Volume 4A Combinatorial Algorithms Part1 日本語版](https://www.amazon.co.jp/dp/4048930559/)
  - Donald E.Knuth (著), 有澤 誠 (監修), 和田 英一 (監修), 筧 一彦 (翻訳), 小出 洋 (翻訳) 
  - 4A, 4Bを合わせて組み合わせアルゴリズムの解説がされている。4AではSATまでは行かず、ブール代数の基礎からそれらをどうやって計算機で扱うかが主に扱われている。Binary Decision Diagram (BDD)やZero-suppressed Binary Decision Diagram (ZDD) に関する解説も含まれている。
- [The Art of Computer Programming Volume 4B Combinatorial Algorithms Part2 日本語版](https://www.amazon.co.jp/dp/4048931148)
  - Donald E.Knuth (著), 和田 英一 (監修), 岩崎 英哉 (翻訳), 田村 直之 (翻訳), 寺田 実 (翻訳) 
  - 4Bではバックトラックによる方法とSATソルバーに関する詳しい議論がある。