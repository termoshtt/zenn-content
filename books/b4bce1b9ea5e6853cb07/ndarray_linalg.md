---
title: 線形代数 (ndarray-linalg crate)
---

[ndarray-linalg](https://github.com/rust-ndarray/ndarray-linalg) を使います

```toml
[dependencies]
ndarray = "0.13"
ndarray-linalg = { version = "0.12", features = ["openblas"] }
```

`features` には BLAS/LAPACKのバックエンドが指定できて、[Intel (R) MKL](https://software.intel.com/content/www/us/en/develop/tools/math-kernel-library.html) を使う場合は 

```toml
ndarray-linalg = { version = "0.12", features = ["intel-mkl"] }
```

のように使います

線型方程式
--------------

```rust
use ndarray::*;
use ndarray_linalg::*;

// Solve `Ax=b`
fn solve() -> Result<(), error::LinalgError> {
    let a: Array2<f64> = random((3, 3));
    let b: Array1<f64> = random(3);
    let _x = a.solve(&b)?;
    Ok(())
}

// Solve `Ax=b` for many b with fixed A
fn factorize() -> Result<(), error::LinalgError> {
    let a: Array2<f64> = random((3, 3));
    let f = a.factorize_into()?; // LU factorize A (A is consumed)
    for _ in 0..10 {
        let b: Array1<f64> = random(3);
        let _x = f.solve_into(b)?; // solve Ax=b using factorized L, U
    }
    Ok(())
}

fn main() {
    solve().unwrap();
    factorize().unwrap();
}
```

`a.solve(&b)` でLU分解で線型方程式を解きます。分解した結果を再利用したい場合は `a.factorize()` を使います。いずれも `*_into()` 版があり、これは `a` を消費する点が異なります。

固有値
---------

```rust
use ndarray::*;
use ndarray_linalg::*;

fn main() {
    let a = arr2(&[[2.0, 1.0, 2.0], [-2.0, 2.0, 1.0], [1.0, 2.0, -2.0]]);
    let (e, vecs) = a.clone().eig().unwrap();
    println!("eigenvalues = \n{:?}", e);
    println!("V = \n{:?}", vecs);
    let a_c: Array2<c64> = a.map(|f| c64::new(*f, 0.0));
    let av = a_c.dot(&vecs);
    println!("AV = \n{:?}", av);
}
```

対称行列の場合は `eigh` を使います

```rust
use ndarray::*;
use ndarray_linalg::*;

fn main() {
    let a = arr2(&[[3.0, 1.0, 1.0], [1.0, 3.0, 1.0], [1.0, 1.0, 3.0]]);
    let (e, vecs) = a.clone().eigh(UPLO::Upper).unwrap();
    println!("eigenvalues = \n{:?}", e);
    println!("V = \n{:?}", vecs);
    let av = a.dot(&vecs);
    println!("AV = \n{:?}", av);
}
```


