use criterion::{black_box, criterion_group, criterion_main, Criterion};

use runiverse::transform::{deg_to_dms, deg_to_ra, ra_to_deg};
use runiverse::RightAscension;

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

fn deg_to_negative_dms(c: &mut Criterion) {
    let deg = black_box(43.0194);
    c.bench_function("Transform negative angle degrees to dd:mm:ss", |b| {
        b.iter(|| deg_to_dms(deg))
    });
}

fn deg_to_positive_dms(c: &mut Criterion) {
    let deg = black_box(78.1038);
    c.bench_function("Transform positive angle degrees to dd:mm:ss", |b| {
        b.iter(|| deg_to_dms(deg))
    });
}

fn deg_to_hms(c: &mut Criterion) {
    let deg = black_box(78.1038);
    c.bench_function("Transform an angle to a `HoursMinSec` type", |b| {
        b.iter(|| deg_to_ra(deg))
    });
}

criterion_group!(
    benches,
    ra_to_deg_positive_angle,
    ra_to_deg_negative_angle,
    deg_to_negative_dms,
    deg_to_positive_dms,
    deg_to_hms,
);
criterion_main!(benches);
