use criterion::{black_box, criterion_group, criterion_main, Criterion};

use universe::julian_day::get_julian_day;
use universe::Date;

fn get_julian_day_benchmark(c: &mut Criterion) {
    let date = black_box(Date::new(-1001, 8, 17.9));
    c.bench_function("Get JD from date", |b| b.iter(|| get_julian_day(&date)));
}

criterion_group!(benches, get_julian_day_benchmark);
criterion_main!(benches);
