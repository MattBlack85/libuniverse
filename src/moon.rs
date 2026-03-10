//! Moon position calculation.
//!
//! Implements the algorithm from Jean Meeus, *Astronomical Algorithms*, 2nd ed.,
//! Chapter 47 "Position of the Moon".

use crate::fit_degrees;
use crate::julian_day::JulianDay;

/// Geocentric position of the Moon.
#[derive(Debug)]
pub struct MoonPosition {
    /// Geocentric ecliptic longitude in degrees.
    pub longitude: f64,
    /// Geocentric ecliptic latitude in degrees.
    pub latitude: f64,
    /// Distance from the centre of the Earth to the centre of the Moon in km.
    pub distance: f64,
    /// Equatorial right ascension in decimal degrees.
    pub ra: f64,
    /// Equatorial declination in decimal degrees.
    pub dec: f64,
}

// Table 47.A — periodic terms for the Moon's longitude (Σl, in units of 0.000001°)
// and distance (Σr, in units of 0.001 km).
// Columns: D, M, M', F, Σl coefficient, Σr coefficient.
// Source: Meeus, *Astronomical Algorithms*, 2nd ed., Table 47.A (pp. 339–340).
static TABLE_47A: &[(i8, i8, i8, i8, i64, i64)] = &[
    (0, 0, 1, 0, 6_288_774, -20_905_355),
    (2, 0, -1, 0, 1_274_027, -3_699_111),
    (2, 0, 0, 0, 658_314, -2_955_968),
    (0, 0, 2, 0, 213_618, -569_925),
    (0, 1, 0, 0, -185_116, 48_888),
    (0, 0, 0, 2, -114_332, -3_149),
    (2, 0, -2, 0, 58_793, 246_158),
    (2, -1, -1, 0, 57_066, -152_138),
    (2, 0, 1, 0, 53_322, -170_733),
    (2, -1, 0, 0, 45_758, -204_586),
    (0, 1, -1, 0, -40_923, -129_620),
    (1, 0, 0, 0, -34_720, 108_743),
    (0, 1, 1, 0, -30_383, 104_755),
    (2, 0, 0, -2, 15_327, 10_321),
    (0, 0, 1, 2, -12_528, 0),
    (0, 0, 1, -2, 10_980, 79_661),
    (4, 0, -1, 0, 10_675, -34_782),
    (0, 0, 3, 0, 10_034, -23_210),
    (4, 0, -2, 0, 8_548, -21_636),
    (2, 1, -1, 0, -7_888, 24_208),
    (2, 1, 0, 0, -6_766, 30_824),
    (1, 0, -1, 0, -5_163, -8_379),
    (1, 1, 0, 0, 4_987, -16_675),
    (2, -1, 1, 0, 4_036, -12_831),
    (2, 0, 2, 0, 3_994, -10_445),
    (4, 0, 0, 0, 3_861, -11_650),
    (2, 0, -3, 0, 3_665, 14_403),
    (0, 1, -2, 0, -2_689, -7_003),
    (2, 0, -1, 2, -2_602, 0),
    (2, -1, -2, 0, 2_390, 10_056),
    (1, 0, 1, 0, -2_348, 6_322),
    (2, -2, 0, 0, 2_236, -9_884),
    (0, 1, 2, 0, -2_120, 5_751),
    (0, 2, 0, 0, -2_069, 0),
    (2, -2, -1, 0, 2_048, -4_950),
    (2, 0, 1, -2, -1_773, 4_130),
    (2, 0, 0, 2, -1_595, 0),
    (4, -1, -1, 0, 1_215, -3_958),
    (0, 0, 2, 2, -1_110, 0),
    (3, 0, -1, 0, -892, 3_258),
    (2, 1, 1, 0, -810, 2_616),
    (4, -1, -2, 0, 759, -1_897),
    (0, 2, -1, 0, -713, -2_117),
    (2, 2, -1, 0, -700, 2_354),
    (2, 1, -2, 0, 691, 0),
    (2, -1, 0, -2, 596, 0),
    (4, 0, 1, 0, 549, -1_423),
    (0, 0, 4, 0, 537, -1_117),
    (4, -1, 0, 0, 520, -1_571),
    (1, 0, -2, 0, -487, -1_739),
    (2, 1, 0, -2, -399, 0),
    (0, 0, 2, -2, -381, -4_421),
    (1, 1, 1, 0, 351, 0),
    (3, 0, -2, 0, -340, 0),
    (4, 0, -3, 0, 330, 0),
    (2, -1, 2, 0, 327, 0),
    (0, 2, 1, 0, -323, 1_165),
    (1, 1, -1, 0, 299, 0),
    (2, 0, 3, 0, 294, 0),
    (2, 0, -1, -2, 0, 8_752),
];

// Table 47.B — periodic terms for the Moon's latitude (Σb, in units of 0.000001°).
// Columns: D, M, M', F, Σb coefficient.
// Source: Meeus, *Astronomical Algorithms*, 2nd ed., Table 47.B (pp. 341–342).
static TABLE_47B: &[(i8, i8, i8, i8, i64)] = &[
    (0, 0, 0, 1, 5_128_122),
    (0, 0, 1, 1, 280_602),
    (0, 0, 1, -1, 277_693),
    (2, 0, 0, -1, 173_237),
    (2, 0, -1, 1, 55_413),
    (2, 0, -1, -1, 46_271),
    (2, 0, 0, 1, 32_573),
    (0, 0, 2, 1, 17_198),
    (2, 0, 1, -1, 9_266),
    (0, 0, 2, -1, 8_822),
    (2, -1, 0, -1, 8_216),
    (2, 0, -2, -1, 4_324),
    (2, 0, 1, 1, 4_200),
    (2, 1, 0, -1, -3_359),
    (2, -1, -1, 1, 2_463),
    (2, -1, 0, 1, 2_211),
    (2, -1, -1, -1, 2_065),
    (0, 1, -1, -1, -1_870),
    (4, 0, -1, -1, 1_828),
    (0, 1, 0, 1, -1_794),
    (0, 0, 0, 3, -1_749),
    (0, 1, -1, 1, -1_565),
    (1, 0, 0, 1, -1_491),
    (0, 1, 1, 1, -1_475),
    (0, 1, 1, -1, -1_410),
    (0, 1, 0, -1, -1_344),
    (1, 0, 0, -1, -1_335),
    (0, 0, 3, 1, 1_107),
    (4, 0, 0, -1, 1_021),
    (4, 0, -1, 1, 833),
    (0, 0, 1, -3, 777),
    (4, 0, -2, 1, 671),
    (2, 0, 0, -3, 607),
    (2, 0, 2, -1, 596),
    (2, -1, 1, -1, 491),
    (2, 0, -2, 1, -451),
    (0, 0, 3, -1, 439),
    (2, 0, 2, 1, 422),
    (2, 0, -3, -1, 421),
    (2, 1, -1, 1, -366),
    (2, 1, 0, 1, -351),
    (4, 0, 0, 1, 331),
    (2, -1, 1, 1, 315),
    (2, -2, 0, -1, 302),
    (0, 0, 1, 3, -283),
    (2, 1, 1, -1, -229),
    (1, 1, 0, -1, 223),
    (1, 1, 0, 1, 223),
    (0, 1, -2, -1, -220),
    (2, 1, -1, -1, -220),
    (1, 0, 1, 1, -185),
    (2, -1, -2, -1, 181),
    (0, 1, 2, 1, -177),
    (4, 0, -2, -1, 176),
    (4, -1, -1, -1, 166),
    (1, 0, 1, -1, -164),
    (4, 0, 1, -1, 132),
    (1, 0, -1, -1, -119),
    (4, -1, 0, -1, 115),
    (2, -2, 0, 1, 107),
];

/// Compute the geocentric position of the Moon for the given Julian Day.
///
/// Returns a [`MoonPosition`] containing the geocentric ecliptic longitude and
/// latitude (in degrees), the Earth–Moon distance (in km), and the equatorial
/// right ascension and declination (in decimal degrees).
///
/// Algorithm: Meeus, *Astronomical Algorithms*, 2nd ed., Chapter 47,
/// equations 47.1–47.5 and Tables 47.A–47.B.
#[must_use]
pub fn get_moon_position(jd: &JulianDay) -> MoonPosition {
    let pi = std::f64::consts::PI;
    let to_rad = pi / 180.0;

    // Julian centuries from J2000.0 (Meeus eq. 47.1)
    let t = (jd.get_value() - 2_451_545.0) / 36_525.0;
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;

    // Moon's mean longitude L' (degrees), Meeus eq. 47.1
    let lp = fit_degrees(
        218.316_447_7 + 481_267.881_234_21 * t - 0.001_578_6 * t2 + t3 / 538_841.0
            - t4 / 65_194_000.0,
    );

    // Moon's mean elongation D (degrees), Meeus eq. 47.2
    let d = fit_degrees(
        297.850_192_1 + 445_267.111_403_4 * t - 0.001_881_9 * t2 + t3 / 545_868.0
            - t4 / 113_065_000.0,
    );

    // Sun's mean anomaly M (degrees), Meeus eq. 47.3
    let m =
        fit_degrees(357.529_109_2 + 35_999.050_290_9 * t - 0.000_153_6 * t2 + t3 / 24_490_000.0);

    // Moon's mean anomaly M' (degrees), Meeus eq. 47.4
    let mp = fit_degrees(
        134.963_396_4 + 477_198.867_505_5 * t + 0.008_741_4 * t2 + t3 / 69_699.0
            - t4 / 14_712_000.0,
    );

    // Moon's argument of latitude F (degrees), Meeus eq. 47.5
    let f = fit_degrees(
        93.272_095_0 + 483_202.017_523_3 * t - 0.003_653_9 * t2 - t3 / 3_526_000.0
            + t4 / 863_310_000.0,
    );

    // Perturbation arguments from Venus and Jupiter (Meeus p. 338)
    let a1 = fit_degrees(119.75 + 131.849 * t);
    let a2 = fit_degrees(53.09 + 479_264.290 * t);
    let a3 = fit_degrees(313.45 + 481_266.484 * t);

    // Eccentricity correction for Sun's mean anomaly M (Meeus eq. 47.6)
    let e = 1.0 - 0.002_516 * t - 0.000_007_4 * t2;
    let e2 = e * e;

    let d_rad = d * to_rad;
    let m_rad = m * to_rad;
    let mp_rad = mp * to_rad;
    let f_rad = f * to_rad;
    let lp_rad = lp * to_rad;

    // Sum periodic terms for longitude (Σl) and distance (Σr) — Table 47.A
    let mut sum_l: f64 = 0.0;
    let mut sum_r: f64 = 0.0;

    for &(nd, nm, nmp, nf, coeff_l, coeff_r) in TABLE_47A {
        let arg = f64::from(nd) * d_rad
            + f64::from(nm) * m_rad
            + f64::from(nmp) * mp_rad
            + f64::from(nf) * f_rad;

        // Apply eccentricity correction when Sun's anomaly is involved
        let e_factor = match nm.abs() {
            1 => e,
            2 => e2,
            _ => 1.0,
        };

        sum_l += e_factor * (coeff_l as f64) * arg.sin();
        sum_r += e_factor * (coeff_r as f64) * arg.cos();
    }

    // Sum periodic terms for latitude (Σb) — Table 47.B
    let mut sum_b: f64 = 0.0;

    for &(nd, nm, nmp, nf, coeff_b) in TABLE_47B {
        let arg = f64::from(nd) * d_rad
            + f64::from(nm) * m_rad
            + f64::from(nmp) * mp_rad
            + f64::from(nf) * f_rad;

        let e_factor = match nm.abs() {
            1 => e,
            2 => e2,
            _ => 1.0,
        };

        sum_b += e_factor * (coeff_b as f64) * arg.sin();
    }

    // Additional corrections for Venus, Jupiter, and flattening of Earth
    // (Meeus p. 338)
    sum_l += 3_958.0 * (a1 * to_rad).sin()
        + 1_962.0 * (lp_rad - f_rad).sin()
        + 318.0 * (a2 * to_rad).sin();

    sum_b += -2_235.0 * lp_rad.sin()
        + 382.0 * (a3 * to_rad).sin()
        + 175.0 * ((a1 - f) * to_rad).sin()
        + 175.0 * ((a1 + f) * to_rad).sin()
        + 127.0 * (lp_rad - mp_rad).sin()
        - 115.0 * (lp_rad + mp_rad).sin();

    // Geocentric ecliptic longitude λ (degrees)
    let lambda = fit_degrees(lp + sum_l / 1_000_000.0);

    // Geocentric ecliptic latitude β (degrees)
    let beta = sum_b / 1_000_000.0;

    // Distance Earth–Moon Δ (km)
    let delta = 385_000.56 + sum_r / 1_000.0;

    // Convert geocentric ecliptic to equatorial coordinates.
    //
    // Mean obliquity of the ecliptic ε (Meeus Ch. 22, eq. 22.2, low-precision form):
    //   ε = 23°26'21.448" − 46.8150"·T − 0.00059"·T² + 0.001813"·T³
    let epsilon = (23.0 + 26.0 / 60.0 + 21.448 / 3_600.0
        - (46.815_0 / 3_600.0) * t
        - (0.000_59 / 3_600.0) * t2
        + (0.001_813 / 3_600.0) * t3)
        * to_rad;

    let lambda_rad = lambda * to_rad;
    let beta_rad = beta * to_rad;

    // Meeus eq. 13.3 and 13.4 (ecliptic → equatorial)
    let ra_rad =
        (lambda_rad.sin() * epsilon.cos() - beta_rad.tan() * epsilon.sin()).atan2(lambda_rad.cos());
    let dec_rad =
        (beta_rad.sin() * epsilon.cos() + beta_rad.cos() * epsilon.sin() * lambda_rad.sin()).asin();

    let ra = fit_degrees(ra_rad * 180.0 / pi);
    let dec = dec_rad * 180.0 / pi;

    MoonPosition {
        longitude: lambda,
        latitude: beta,
        distance: delta,
        ra,
        dec,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::julian_day::JulianDay;

    #[test]
    fn test_moon_position_meeus_example_47a() {
        // Meeus, Astronomical Algorithms, 2nd ed., Chapter 47, Example 47.a
        // Date: 1992 April 12, 0h TD  →  JDE = 2448724.5
        //
        // Expected results (Meeus p. 342):
        //   λ = 133.167265°,  β = −3.229126°,  Δ = 368409.7 km
        let jd = JulianDay::new(2_448_724.5);
        let pos = get_moon_position(&jd);

        assert!(
            (pos.longitude - 133.167_265).abs() < 0.01,
            "longitude: {}",
            pos.longitude
        );
        assert!(
            (pos.latitude - (-3.229_126)).abs() < 0.001,
            "latitude: {}",
            pos.latitude
        );
        assert!(
            (pos.distance - 368_409.7).abs() < 1.0,
            "distance: {}",
            pos.distance
        );
    }

    #[test]
    fn test_moon_equatorial_coords_meeus_example_47a() {
        // Meeus, Astronomical Algorithms, 2nd ed., Chapter 47, Example 47.a
        // The equatorial coordinates derived from λ=133.167265°, β=−3.229126°
        // with ε ≈ 23.440636° give (Meeus p. 342):
        //   α ≈ 134.688° (≈ 8h 58m 45s),  δ ≈ 13.768°
        let jd = JulianDay::new(2_448_724.5);
        let pos = get_moon_position(&jd);

        assert!(
            (pos.ra - 134.688).abs() < 0.01,
            "right ascension: {}",
            pos.ra
        );
        assert!((pos.dec - 13.768).abs() < 0.01, "declination: {}", pos.dec);
    }
}
