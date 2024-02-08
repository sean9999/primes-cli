use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use criterion::BenchmarkId;
use criterion::Throughput;

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 10", |b| b.iter(|| fibonacci(black_box(10))));
}

fn beneath_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("beneath_range");

    group.sampling_mode(criterion::SamplingMode::Flat);
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(120));

    let sizes: [usize; 13] = [
        10, 100, 1000, 10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000, 100000,
    ];

    for size in sizes {
        group.throughput(Throughput::Bytes(size.try_into().unwrap()));

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &s| {
            b.iter(|| primes::beneath(s));
        });
    }
    group.finish();
}

criterion_group!{
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(60)).sample_size(10);
  targets = criterion_benchmark, beneath_range
}

criterion_main!(benches);
