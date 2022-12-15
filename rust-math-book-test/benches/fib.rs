use criterion::{black_box, criterion_group, criterion_main, Criterion};

// フィボナッチ数を求める
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// ラムダ関数の形でベンチマークを登録する
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

// ベンチマークグループを定義する
criterion_group!(benches, criterion_benchmark);

// main関数を用意
criterion_main!(benches);
