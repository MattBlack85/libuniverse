use criterion::{Criterion, criterion_group, criterion_main};

use runiverse::fit_degrees;

fn fit_small_negative_angle(c: &mut Criterion) {
    c.bench_function("Fit a small negative angle into 0-360 range", |b| {
        b.iter(|| fit_degrees(-0.000000001))
    });
}

criterion_group!(benches, fit_small_negative_angle,);
criterion_main!(benches);
