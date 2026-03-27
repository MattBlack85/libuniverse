//! Nutation — Meeus, *Astronomical Algorithms*, 2nd ed., Chapter 22.
//!
//! Implements the IAU 1980 theory of nutation (63-term series) and returns:
//! - Δψ: nutation in longitude (arcseconds)
//! - Δε: nutation in obliquity (arcseconds)

use crate::julian_day::JulianDay;

/// Nutation in longitude and obliquity for a given instant.
#[derive(Debug, PartialEq)]
pub struct Nutation {
    /// Nutation in longitude Δψ, in arcseconds.
    pub delta_psi: f64,
    /// Nutation in obliquity Δε, in arcseconds.
    pub delta_eps: f64,
}

// Table 22.A — Meeus, Ch. 22, pp. 145–146.
// Columns: nD, nM, nM', nF, nΩ,
//          S (×0.0001"), S' (×0.0001"/T), C (×0.0001"), C' (×0.0001"/T)
#[rustfmt::skip]
#[allow(clippy::type_complexity)]
static TABLE_22A: &[(i8, i8, i8, i8, i8, i32, f64, i32, f64)] = &[
    ( 0,  0,  0,  0,  1, -171996, -174.2,  92025,   8.9),
    (-2,  0,  0,  2,  2,  -13187,   -1.6,   5736,  -3.1),
    ( 0,  0,  0,  2,  2,   -2274,   -0.2,    977,  -0.5),
    ( 0,  0,  0,  0,  2,    2062,    0.2,   -895,   0.5),
    ( 0,  1,  0,  0,  0,    1426,   -3.4,     54,  -0.1),
    ( 0,  0,  1,  0,  0,     712,    0.1,     -7,   0.0),
    (-2,  1,  0,  2,  2,    -517,    1.2,    224,  -0.6),
    ( 0,  0,  0,  2,  1,    -386,   -0.4,    200,   0.0),
    ( 0,  0,  1,  2,  2,    -301,    0.0,    129,  -0.1),
    (-2, -1,  0,  2,  2,     217,   -0.5,    -95,   0.3),
    (-2,  0,  1,  0,  0,    -158,    0.0,      0,   0.0),
    (-2,  0,  0,  2,  1,     129,    0.1,    -70,   0.0),
    ( 0,  0, -1,  2,  2,     123,    0.0,    -53,   0.0),
    ( 2,  0,  0,  0,  0,      63,    0.0,      0,   0.0),
    ( 0,  0,  1,  0,  1,      63,    0.1,    -33,   0.0),
    ( 2,  0, -1,  2,  2,     -59,    0.0,     26,   0.0),
    ( 0,  0, -1,  0,  1,     -58,   -0.1,     32,   0.0),
    ( 0,  0,  1,  2,  1,     -51,    0.0,     27,   0.0),
    (-2,  0,  2,  0,  0,      48,    0.0,      0,   0.0),
    ( 0,  0, -2,  2,  1,      46,    0.0,    -24,   0.0),
    ( 2,  0,  0,  2,  2,     -38,    0.0,     16,   0.0),
    ( 0,  0,  2,  2,  2,     -31,    0.0,     13,   0.0),
    ( 0,  0,  2,  0,  0,      29,    0.0,      0,   0.0),
    (-2,  0,  1,  2,  2,      29,    0.0,    -12,   0.0),
    ( 0,  0,  0,  2,  0,      26,    0.0,      0,   0.0),
    (-2,  0,  0,  2,  0,     -22,    0.0,      0,   0.0),
    ( 0,  0, -1,  2,  1,      21,    0.0,    -10,   0.0),
    ( 0,  2,  0,  0,  0,      17,   -0.1,      0,   0.0),
    ( 2,  0, -1,  0,  1,      16,    0.0,     -8,   0.0),
    (-2,  2,  0,  2,  2,     -16,    0.1,      7,   0.0),
    ( 0,  1,  0,  0,  1,     -15,    0.0,      9,   0.0),
    (-2,  0,  1,  0,  1,     -13,    0.0,      7,   0.0),
    ( 0, -1,  0,  0,  1,     -12,    0.0,      6,   0.0),
    ( 0,  0,  2, -2,  0,      11,    0.0,      0,   0.0),
    ( 2,  0, -1,  2,  1,     -10,    0.0,      5,   0.0),
    ( 2,  0,  1,  2,  2,      -8,    0.0,      3,   0.0),
    ( 0,  1,  0,  2,  2,      -7,    0.0,      3,   0.0),
    (-2,  1,  1,  0,  0,      -7,    0.0,      0,   0.0),
    ( 0, -1,  0,  2,  2,      -7,    0.0,      3,   0.0),
    ( 2,  0,  0,  2,  1,      -6,    0.0,      3,   0.0),
    ( 2,  0,  1,  0,  0,      -6,    0.0,      0,   0.0),
    (-2,  0,  2,  2,  2,       6,    0.0,     -3,   0.0),
    (-2,  0,  1,  2,  1,       6,    0.0,     -3,   0.0),
    ( 2,  0, -2,  0,  1,      -5,    0.0,      3,   0.0),
    ( 2,  0,  0,  0,  1,      -5,    0.0,      3,   0.0),
    ( 0, -1,  1,  0,  0,      -5,    0.0,      0,   0.0),
    (-2, -1,  0,  2,  1,      -5,    0.0,      3,   0.0),
    (-2,  0,  0,  0,  1,      -5,    0.0,      3,   0.0),
    ( 0,  0,  2,  2,  1,      -5,    0.0,      3,   0.0),
    (-2,  0,  2,  0,  1,       4,    0.0,      0,   0.0),
    (-2,  1,  0,  2,  1,       4,    0.0,      0,   0.0),
    ( 0,  0,  1, -2,  0,       4,    0.0,      0,   0.0),
    (-1,  0,  1,  0,  0,      -4,    0.0,      0,   0.0),
    (-2,  1,  0,  0,  0,      -4,    0.0,      0,   0.0),
    ( 1,  0,  0,  0,  0,      -4,    0.0,      0,   0.0),
    ( 0,  0,  1,  2,  0,       3,    0.0,      0,   0.0),
    ( 0,  0, -2,  2,  2,      -3,    0.0,      1,   0.0),
    (-1, -1,  1,  0,  0,      -3,    0.0,      0,   0.0),
    ( 0,  1,  1,  0,  0,      -3,    0.0,      0,   0.0),
    ( 0, -1,  1,  2,  2,      -3,    0.0,      1,   0.0),
    ( 2, -1, -1,  2,  2,      -3,    0.0,      1,   0.0),
    ( 0,  0,  3,  2,  2,      -3,    0.0,      1,   0.0),
    ( 2, -1,  0,  2,  2,      -3,    0.0,      1,   0.0),
];

/// Compute nutation in longitude (Δψ) and obliquity (Δε) for the given Julian Day.
///
/// Uses the 63-term IAU 1980 series from Meeus,
/// *Astronomical Algorithms*, 2nd ed., Chapter 22, Table 22.A, pp. 145–146.
///
/// Both components of the returned [`Nutation`] are in **arcseconds**.
#[must_use]
pub fn get_nutation(jd: &JulianDay) -> Nutation {
    // Julian centuries from J2000.0 — eq. 22.1
    let t = (jd.get_value() - 2_451_545.0) / 36_525.0;
    let t2 = t * t;
    let t3 = t2 * t;

    // Fundamental arguments in degrees — eqs. 22.2–22.6
    let d = 297.850_36 + 445_267.111_480 * t - 0.001_914_2 * t2 + t3 / 189_474.0;
    let m = 357.527_72 + 35_999.050_340 * t - 0.000_160_3 * t2 - t3 / 300_000.0;
    let mp = 134.962_98 + 477_198.867_398 * t + 0.008_697_2 * t2 + t3 / 56_250.0;
    let f = 93.271_91 + 483_202.017_538 * t - 0.003_682_5 * t2 + t3 / 327_270.0;
    let omega = 125.044_52 - 1_934.136_261 * t + 0.002_070_8 * t2 + t3 / 450_000.0;

    let d = d.to_radians();
    let m = m.to_radians();
    let mp = mp.to_radians();
    let f = f.to_radians();
    let omega = omega.to_radians();

    let mut sum_psi = 0.0_f64;
    let mut sum_eps = 0.0_f64;

    for &(nd, nm, nmp, nf, nomega, s, sp, c, cp) in TABLE_22A {
        let arg = f64::from(nd) * d
            + f64::from(nm) * m
            + f64::from(nmp) * mp
            + f64::from(nf) * f
            + f64::from(nomega) * omega;
        let (sin_arg, cos_arg) = arg.sin_cos();
        sum_psi += (f64::from(s) + sp * t) * sin_arg;
        sum_eps += (f64::from(c) + cp * t) * cos_arg;
    }

    // Convert from 0.0001 arcsecond units to arcseconds
    Nutation {
        delta_psi: sum_psi / 10_000.0,
        delta_eps: sum_eps / 10_000.0,
    }
}

/// Compute only nutation in longitude (Δψ), in arcseconds.
///
/// Skips the cosine/obliquity (Δε) accumulation entirely — roughly halving
/// trigonometric work versus [`get_nutation`]. Callers that only need Δψ
/// (e.g. apparent sidereal time) should prefer this function.
///
/// On `x86_64` with `avx2` + `fma` target features the non-trig arithmetic
/// is batched four rows at a time using 256-bit FMA instructions.
/// All other targets fall back to an FMA-optimised scalar loop.
///
/// Meeus, *Astronomical Algorithms*, 2nd ed., Chapter 22, Table 22.A.
#[must_use]
pub fn get_delta_psi(jd: &JulianDay) -> f64 {
    let t = (jd.get_value() - 2_451_545.0) / 36_525.0;
    let t2 = t * t;
    let t3 = t2 * t;

    let d = (297.850_36 + 445_267.111_480 * t - 0.001_914_2 * t2 + t3 / 189_474.0).to_radians();
    let m = (357.527_72 + 35_999.050_340 * t - 0.000_160_3 * t2 - t3 / 300_000.0).to_radians();
    let mp = (134.962_98 + 477_198.867_398 * t + 0.008_697_2 * t2 + t3 / 56_250.0).to_radians();
    let f = (93.271_91 + 483_202.017_538 * t - 0.003_682_5 * t2 + t3 / 327_270.0).to_radians();
    let omega = (125.044_52 - 1_934.136_261 * t + 0.002_070_8 * t2 + t3 / 450_000.0).to_radians();

    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx2",
        target_feature = "fma"
    ))]
    {
        // SAFETY: both `avx2` and `fma` are verified at compile time by the
        // enclosing `cfg` — no runtime CPU detection needed.
        return unsafe { delta_psi_avx2_fma(t, d, m, mp, f, omega) };
    }

    delta_psi_scalar(t, d, m, mp, f, omega)
}

/// Scalar (FMA-optimised) inner loop for [`get_delta_psi`].
fn delta_psi_scalar(t: f64, d: f64, m: f64, mp: f64, f_arg: f64, omega: f64) -> f64 {
    let mut sum = 0.0_f64;
    for &(nd, nm, nmp, nf, nomega, s, sp, _, _) in TABLE_22A {
        #[cfg(target_feature = "fma")]
        let arg = f64::from(nd).mul_add(
            d,
            f64::from(nm).mul_add(
                m,
                f64::from(nmp).mul_add(mp, f64::from(nf).mul_add(f_arg, f64::from(nomega) * omega)),
            ),
        );
        #[cfg(not(target_feature = "fma"))]
        let arg = f64::from(nd) * d
            + f64::from(nm) * m
            + f64::from(nmp) * mp
            + f64::from(nf) * f_arg
            + f64::from(nomega) * omega;

        #[cfg(target_feature = "fma")]
        {
            sum = sp.mul_add(t, f64::from(s)).mul_add(arg.sin(), sum);
        }
        #[cfg(not(target_feature = "fma"))]
        {
            sum += (f64::from(s) + sp * t) * arg.sin();
        }
    }
    sum / 10_000.0
}

/// AVX2 + FMA inner loop for [`get_delta_psi`]: batches argument computation
/// and amplitude accumulation four rows at a time using 256-bit SIMD.
/// Sin is still evaluated per-lane (no vectorised trig in `std`).
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx2",
    target_feature = "fma"
))]
unsafe fn delta_psi_avx2_fma(t: f64, d: f64, m: f64, mp: f64, f_arg: f64, omega: f64) -> f64 {
    use std::arch::x86_64::*;

    let vd = _mm256_set1_pd(d);
    let vm = _mm256_set1_pd(m);
    let vmp = _mm256_set1_pd(mp);
    let vf = _mm256_set1_pd(f_arg);
    let vo = _mm256_set1_pd(omega);
    let vt = _mm256_set1_pd(t);
    let mut vsum = _mm256_setzero_pd();

    let n = TABLE_22A.len(); // 63
    let chunks = n / 4; // 15 full groups

    for i in 0..chunks {
        let b = i * 4;
        let r = &TABLE_22A[b..b + 4];

        // Pack integer multipliers for 4 rows into f64x4 lanes.
        // _mm256_set_pd fills lanes [3,2,1,0] from its args left-to-right.
        let vnd = _mm256_set_pd(
            f64::from(r[3].0),
            f64::from(r[2].0),
            f64::from(r[1].0),
            f64::from(r[0].0),
        );
        let vnm = _mm256_set_pd(
            f64::from(r[3].1),
            f64::from(r[2].1),
            f64::from(r[1].1),
            f64::from(r[0].1),
        );
        let vnmp = _mm256_set_pd(
            f64::from(r[3].2),
            f64::from(r[2].2),
            f64::from(r[1].2),
            f64::from(r[0].2),
        );
        let vnf = _mm256_set_pd(
            f64::from(r[3].3),
            f64::from(r[2].3),
            f64::from(r[1].3),
            f64::from(r[0].3),
        );
        let vno = _mm256_set_pd(
            f64::from(r[3].4),
            f64::from(r[2].4),
            f64::from(r[1].4),
            f64::from(r[0].4),
        );

        // arg[i] = nd*d + nm*m + nmp*mp + nf*f + nomega*omega  (FMA chain)
        let varg = _mm256_fmadd_pd(
            vnd,
            vd,
            _mm256_fmadd_pd(
                vnm,
                vm,
                _mm256_fmadd_pd(vnmp, vmp, _mm256_fmadd_pd(vnf, vf, _mm256_mul_pd(vno, vo))),
            ),
        );

        // Extract the 4 args and evaluate sin individually (no std SIMD trig).
        let mut args = [0.0f64; 4];
        _mm256_storeu_pd(args.as_mut_ptr(), varg);
        let vsins = _mm256_set_pd(args[3].sin(), args[2].sin(), args[1].sin(), args[0].sin());

        // amplitude[i] = s + sp * t  (FMA)
        let vs = _mm256_set_pd(
            f64::from(r[3].5),
            f64::from(r[2].5),
            f64::from(r[1].5),
            f64::from(r[0].5),
        );
        let vsp = _mm256_set_pd(r[3].6, r[2].6, r[1].6, r[0].6);
        let vamp = _mm256_fmadd_pd(vsp, vt, vs);

        // sum += amplitude * sin(arg)  (FMA)
        vsum = _mm256_fmadd_pd(vamp, vsins, vsum);
    }

    // Horizontal reduction of the 4 lanes.
    let mut lanes = [0.0f64; 4];
    _mm256_storeu_pd(lanes.as_mut_ptr(), vsum);
    let mut sum = lanes[0] + lanes[1] + lanes[2] + lanes[3];

    // Scalar FMA tail for the 3 remainder rows (63 % 4 == 3).
    for i in (chunks * 4)..n {
        let (nd, nm, nmp, nf, nomega, s, sp, _, _) = TABLE_22A[i];
        let arg = f64::from(nd).mul_add(
            d,
            f64::from(nm).mul_add(
                m,
                f64::from(nmp).mul_add(mp, f64::from(nf).mul_add(f_arg, f64::from(nomega) * omega)),
            ),
        );
        sum = sp.mul_add(t, f64::from(s)).mul_add(arg.sin(), sum);
    }

    sum / 10_000.0
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::julian_day::JulianDay;

    #[test]
    fn test_nutation_1987_apr_10() {
        // Meeus, Astronomical Algorithms, 2nd ed., Example 22.a, p. 148
        // JDE = 2446895.5  (1987 April 10, 0h TT)
        // Expected: Δψ = −3.788", Δε = +9.443"
        let jd = JulianDay::new(2_446_895.5);
        let nut = get_nutation(&jd);
        // Meeus rounds intermediate values in the worked example; the full
        // 63-term sum produces −3.7905" and +9.4432", both within 0.005" of
        // the printed result — well within the precision of this approximation.
        assert!(
            (nut.delta_psi - (-3.788)).abs() < 0.005,
            "delta_psi = {:.4}, expected ≈ -3.788",
            nut.delta_psi
        );
        assert!(
            (nut.delta_eps - 9.443).abs() < 0.005,
            "delta_eps = {:.4}, expected ≈ 9.443",
            nut.delta_eps
        );
    }
}
