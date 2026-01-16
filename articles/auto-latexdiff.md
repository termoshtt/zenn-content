---
title: "ブランチを作ったら自動でlatexdiffしてPDFを生成してほしい (GitLab CI編)"
emoji: "🦀"
type: "tech"
topics: ["latex", "devops", "latexdiff", "gitlab"]
published: true
---

この記事ではGitLab CIを使ってLaTeX文章を管理する方法について述べていきます。Gitの使い方については既知とします。

latexdiff
----------
latexdiffはLaTeX文章の差分をLaTeX文章としてタイプセットしてくれるツールです。

GitLab CI入門
--------------
GitLab CIはGitLabに付随したCI(継続的インテグレーション)ツールです。要は`git push`したら色んな処理を自動的に実行してくれるやつです。類似のサービスとしてGitHub Actionsがありますが、こちらを使う場合は別の記事になる予定です。

GitLab CIでは起動してほしい処理を `.gitlab-ci.yml` ファイルに記述していきます。
