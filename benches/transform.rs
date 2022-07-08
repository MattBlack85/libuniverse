use criterion::{black_box, criterion_group, criterion_main, Criterion};

use universe::transform::ra_to_deg;
use universe::RightAscension;

fn ra_to_deg_positive_angle(c: &mut Criterion) {
    let ra = black_box(RightAscension::new(4, 55, 45.0));
    c.bench_function("Transform RA to degrees (positive angle)", |b| {
        b.iter(|| ra_to_deg(&ra))
    });
}

fn ra_to_deg_negative_angle(c: &mut Criterion) {
    let ra = black_box(RightAscension::new(23, 55, 45.0));
    c.bench_function("Transform RA to degrees (negative angle)", |b| {
        b.iter(|| ra_to_deg(&ra))
    });
}

criterion_group!(benches, ra_to_deg_positive_angle, ra_to_deg_negative_angle);
criterion_main!(benches);
