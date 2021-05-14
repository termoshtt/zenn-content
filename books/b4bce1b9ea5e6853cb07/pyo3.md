---
title: PythonからRustの関数を呼び出す (pyo3 crate)
---

[pyo3](https://github.com/PyO3/pyo3) crateを用いるとRustの関数をPythonから呼び出す事が出来ます。ここでは `ndarray-linalg` と共に用いる方法について述べます。

https://github.com/termoshtt/pyo3-linalg-example

```toml
[package]
name = "pyo3-linalg-example"
version = "0.1.0"
edition = "2018"

[lib]
name = "pyo3_linalg_example"
crate-type = ["cdylib"]

[dependencies]
ndarray = "0.13.1"
numpy = "0.9.0"

[dependencies.pyo3]
version = "0.10.1"
features = ["extension-module"]

[dependencies.ndarray-linalg]
version = "0.12.0"
features = ["static"]
```

いくつかポイントがあって

- `crate-type = ["cdylib"]` で共有ライブラリを作ります
- `ndarray-linalg` に `features = ["static"]` をつけます。FFIでLAPACKを呼び出している為、その部分を静的に結合するためです。

```rust
use ndarray::*;
use ndarray_linalg::*;
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};

#[pymodule]
fn pyo3_linalg_example(_py: Python, m: &PyModule) -> PyResult<()> {
    // immutable example
    fn axpy(a: f64, x: ArrayViewD<f64>, y: ArrayViewD<f64>) -> ArrayD<f64> {
        a * &x + &y
    }

    // mutable example (no return)
    fn mult(a: f64, mut x: ArrayViewMutD<f64>) {
        x *= a;
    }

    // wrapper of `axpy`
    #[pyfn(m, "axpy")]
    fn axpy_py(
        py: Python,
        a: f64,
        x: &PyArrayDyn<f64>,
        y: &PyArrayDyn<f64>,
    ) -> Py<PyArrayDyn<f64>> {
        let x = x.as_array();
        let y = y.as_array();
        axpy(a, x, y).into_pyarray(py).to_owned()
    }

    // wrapper of `axpy`
    #[pyfn(m, "det")]
    fn det_py(_py: Python, x: &PyArrayDyn<f64>) -> f64 {
        let x = x.as_array();
        if let &[nx, ny] = x.shape() {
            let x = x.into_shape((nx, ny)).unwrap();
            x.det().unwrap()
        } else {
            panic!("Must be 2D array");
        }
    }

    // wrapper of `mult`
    #[pyfn(m, "mult")]
    fn mult_py(_py: Python, a: f64, x: &PyArrayDyn<f64>) -> PyResult<()> {
        let x = x.as_array_mut();
        mult(a, x);
        Ok(())
    }
    Ok(())
}
```

これをPyO3のパッケージャである [maturin](https://github.com/PyO3/maturin) を用いてパッケージするとwheelが出来上がります

```shell
pip install maturin
maturin build --manylinux=off
```
