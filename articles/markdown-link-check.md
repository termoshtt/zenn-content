---
title: "GitHub ActionでMarkdownのリンクを検査する"
emoji: "🔗"
type: "tech"
topics: ["github", "githubactions"]
published: true
---

Markdown link check
--------------------

https://github.com/tcort/markdown-link-check

これはMarkdownのテキストからLinkを抽出して、それが200 OKを返すかチェックしてくれます。

```
npm install --save-dev markdown-link-check
```

でプロジェクト毎にインストールして使うか、システムにインストールするときは`-g`でインストールします。

```
markdown-link-check ./README.md
```

のように引数で取ったファイルを検査してくれます：

```
$ markdown-link-check README.md

FILE: README.md
[✓] https://github.com/stepcode/stepcode
[✖] https://crates.io/crates/espr
[✓] https://img.shields.io/crates/v/espr.svg
[✓] https://docs.rs/espr
[✓] https://docs.rs/espr/badge.svg

...

32 links checked.

ERROR: 3 dead links found!
[✖] https://crates.io/crates/espr → Status: 404
[✖] https://crates.io/crates/ruststep → Status: 404
[✖] https://crates.io/crates/ruststep-derive → Status: 404
```

これは[ruststep/README.md](https://github.com/ricosjp/ruststep/blob/master/README.md)に対する結果になっています。Rustのレジストリcrates.ioは[HTTPヘッダを指定しないと404を返す](https://github.com/rust-lang/crates.io/issues/788)ので`markdown-link-check`の設定ファイルを書く必要があります：

```json
{
  "httpHeaders": [
    {
      "urls": ["https://crates.io/crates"],
      "headers": {
        "Accept": "text/html"
      }
    }
  ]
}
```

設定の詳細は[config-file-format](https://github.com/tcort/markdown-link-check#config-file-format)を見てください。例えば特定のURLだけ除く事も出来ます。

GitHub Actionsの設定
---------------------

https://github.com/gaurav-nelson/github-action-markdown-link-check

これをGitHub Actionsとして提供してくれるのがこれです。これを使うには以下のYAMLを `.github/workflows` 以下に置きます：

```yaml:.github/workflows/doc.yaml
name: doc

on:
  push:
    branches:
      - master
  pull_request: {}

jobs:
  markdown-link-check:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@master
    - uses: gaurav-nelson/github-action-markdown-link-check@v1
      with:
        config-file: 'markdown-link-check.json'
```

`with:`に指定できるオプションは[Custom variables](https://github.com/gaurav-nelson/github-action-markdown-link-check#custom-variables)に詳細があります。`config-file`はmarkdown-link-checkの為の設定ファイル()で、デフォルトで`mlc_config.json`ですがこれをリポジトリに置くと何のファイルか分からないので別名を指定しています。

GitHub Actionsは`.github/workflows`以下に存在しているYAMLファイル毎にWorkflowを作ります。ワークフロー単位で`on`が指定できるので、例えば`master`でのみ実行して欲しい時は次のようにできます：

```yaml
on:
  push:
    branches:
      - master
```
