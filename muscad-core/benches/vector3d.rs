use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cc::vector3d;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3d.ops");

    let v1 = [1.0, 2.0, 3.0];
    let v2 = [1.0, 2.0, 3.0];
    let mut v3 = vector3d::zero();

    group.bench_function("assign", |b| {
        b.iter(|| {
            for _ in 0..10_000_000 {
                vector3d::assign(black_box(&mut v3), black_box(&v2));
            }
        })
    });

    group.bench_function("add", |b| {
        b.iter(|| {
            for _ in 0..100_000_000 {
                vector3d::add(black_box(&v1), black_box(&v2));
            }
        })
    });

    group.finish();
}

/*
fn criterion_benchmark(c: &mut Criterion) {
    let pts = [
        [1.52, 0.46, 1.69],
        [1.52, 1.46, 1.69],
        [0.52, 1.46, 1.69],
        [0.52, 0.46, 1.69],
        [1.52, 0.46, 0.69],
        [1.52, 1.46, 0.69],
        [0.52, 0.46, 0.69],
        [0.52, 1.46, 0.69],
    ];

    let mut group = c.benchmark_group("sample-size-example");
    group.sample_size(1000);
    group.bench_function("my-function", |b| {
        b.iter(|| {
            let pl = cc::plane::Plane::from_planar_points(black_box(&pts));
        })
    });
    group.finish();
}
*/

/*
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add", |b| {
        b.iter(|| vector3d::add(black_box(&[1.0, 1.0, 1.0]), black_box(&[1.0, 1.0, 1.0])))
    });

    c.bench_function("add_mut", |b| {
        b.iter(|| vector3d::add_mut(black_box(&mut [1.0, 1.0, 1.0]), black_box(&[1.0, 1.0, 1.0])))
    });
}
*/

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
