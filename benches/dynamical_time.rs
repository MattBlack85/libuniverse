use criterion::{black_box, criterion_group, criterion_main, Criterion};

use runiverse::date::Date;
use runiverse::dynamical_time::delta_t;

fn find_delta_t(c: &mut Criterion) {
    let date = black_box(Date::new(2022, 11, 8.0));
    c.bench_function(
        "Calculate time difference between dynamical and universal time",
        |b| b.iter(|| delta_t(&date)),
    );
}

criterion_group!(benches, find_delta_t);
criterion_main!(benches);
