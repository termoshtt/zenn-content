use criterion::*;

// フィボナッチ数を求める
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 複数の入力に対してベンチマークを取る
fn fib_bench_with_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("fib");
    for n in [6, 8, 10, 12, 14, 16, 18, 20] {
        group.bench_with_input(BenchmarkId::new("fib", n), &n, |bench, n| {
            bench.iter(|| {
                fibonacci(black_box(*n));
            })
        });
    }
    group.finish();
}

// ベンチマークグループを定義する
criterion_group!(benches, fib_bench_with_input);

// main関数を用意
criterion_main!(benches);
