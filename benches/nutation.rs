use criterion::{Criterion, black_box, criterion_group, criterion_main};

use runiverse::julian_day::JulianDay;
use runiverse::nutation::{get_delta_psi, get_delta_psi_scalar};

fn bench_delta_psi_avx2(c: &mut Criterion) {
    let jd = black_box(JulianDay::new(2_446_895.5));
    c.bench_function("get_delta_psi (avx2+fma dispatch)", |b| {
        b.iter(|| get_delta_psi(&jd))
    });
}

fn bench_delta_psi_scalar(c: &mut Criterion) {
    let jd = black_box(JulianDay::new(2_446_895.5));
    c.bench_function("get_delta_psi_scalar (no avx2)", |b| {
        b.iter(|| get_delta_psi_scalar(&jd))
    });
}

criterion_group!(benches, bench_delta_psi_avx2, bench_delta_psi_scalar);
criterion_main!(benches);
