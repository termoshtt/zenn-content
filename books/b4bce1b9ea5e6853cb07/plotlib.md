---
title: グラフをプロットする(plotlib crate)
---

[plotlib](https://github.com/milliams/plotlib)というPure RustのSVGを生成するライブラリがあります。

```rust
use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::Point;
use plotlib::view::ContinuousView;

fn main() {
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];

    let s = Scatter::from_slice(&data).style(
        // プロット点の色を指定
        Style::new().colour("#35C788"),
    );

    let v = ContinuousView::new()
        .add(&s)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");
    // SVGとして保存する
    Page::single(&v).save("scatter.svg").unwrap();
}
```

![image.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/30426/85f0fed6-b37f-01e0-8906-529b22015860.png)

SVGはImageMagickでPNGに変換できます：

```sh
convert scatter.{svg,png}
```

