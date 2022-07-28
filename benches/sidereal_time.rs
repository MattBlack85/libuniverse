use criterion::{black_box, criterion_group, criterion_main, Criterion};

use runiverse::date::Date;
use runiverse::sidereal_time::get_mean_sidereal_time_from_date;

fn calc_mean_sidereal_time(c: &mut Criterion) {
    let date = black_box(Date::new(2022, 11, 8.0));
    c.bench_function("Calculate the mean sidereal time for a given date", |b| {
        b.iter(|| get_mean_sidereal_time_from_date(&date))
    });
}

criterion_group!(benches, calc_mean_sidereal_time);
criterion_main!(benches);
