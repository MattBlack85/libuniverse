use crate::date::Date;
use crate::fit_degrees;

#[must_use]
pub fn get_mean_sidereal_time_from_date(date: &Date) -> f64 {
    let jd = date.to_julian_day().get_value();
    let t = (&jd - 2_451_545_f64) / 36525_f64;

    #[cfg(target_feature = "fma")]
    let theta = 0.000_387_933f64.mul_add(
        t * t,
        360.985_647_366_29f64.mul_add(&jd - 2_451_545_f64, 280.460_618_37),
    );

    #[cfg(not(target_feature = "fma"))]
    let theta =
        280.460_618_37 + 360.985_647_366_29 * (&jd - 2_451_545_f64) + (0.000_387_933 * (t * t))
            - ((t * t * t) / 38_710_000_f64);

    fit_degrees(theta)
}

/// Mean obliquity of the ecliptic in degrees.
///
/// Meeus, *Astronomical Algorithms*, 2nd ed., Eq. 22.2, p. 147.
fn mean_obliquity(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    23.439_291_111 - 0.013_004_167 * t - 1.638_9e-7 * t2 + 5.036_1e-7 * t3
}

/// Equation of the equinoxes (Δψ cos ε₀) in degrees, using the 4-term
/// approximation from Meeus, *Astronomical Algorithms*, 2nd ed., p. 88.
///
/// Accurate to ~0.5 arcsecond (~0.03 s of time), which is sufficient for
/// apparent sidereal time. Replaces the full 63-term IAU 1980 nutation series
/// with 4 sin evaluations and 1 cos evaluation.
fn equation_of_equinoxes(t: f64) -> f64 {
    // Ascending node of the Moon's mean orbit (Meeus p. 144, eq. 22.6)
    let omega = (125.044_52 - 1_934.136_261 * t).to_radians();
    // Mean longitude of the Sun
    let l_sun = (280.466_5 + 36_000.769_8 * t).to_radians();
    // Mean longitude of the Moon
    let l_moon = (218.316_5 + 481_267.881_3 * t).to_radians();

    // Principal terms of Δψ in arcseconds
    let delta_psi = -17.20 * omega.sin() - 1.32 * (2.0 * l_sun).sin() - 0.23 * (2.0 * l_moon).sin()
        + 0.21 * (2.0 * omega).sin();

    // Equation of equinoxes: Δψ cos ε₀, from arcseconds to degrees
    delta_psi * mean_obliquity(t).to_radians().cos() / 3_600.0
}

/// Apparent sidereal time for the given date, in degrees.
///
/// Apparent sidereal time equals mean sidereal time plus the equation of the
/// equinoxes (Δψ cos ε₀), computed via the 4-term approximation from
/// Meeus, *Astronomical Algorithms*, 2nd ed., Chapter 12, p. 87–88.
#[must_use]
pub fn get_apparent_sidereal_time_from_date(date: &Date) -> f64 {
    let jd = date.to_julian_day().get_value();
    let t = (jd - 2_451_545_f64) / 36_525_f64;

    let mean_st = get_mean_sidereal_time_from_date(date);

    fit_degrees(mean_st + equation_of_equinoxes(t))
}

#[cfg(test)]
mod test {
    use crate::date::Date;
    use crate::sidereal_time::{
        get_apparent_sidereal_time_from_date, get_mean_sidereal_time_from_date,
    };
    use crate::RightAscension;

    #[test]
    fn test_mean_sidereal_time_1() {
        // Example 12.a p.88 from Meeus book 2nd edition
        let date = Date::new(1987, 4, 10.0);
        let mst = get_mean_sidereal_time_from_date(&date);
        let expected_ra = RightAscension::new(13, 10, 46.3668);

        assert_eq!(RightAscension::from_degrees(mst), expected_ra);
    }

    #[test]
    fn test_apparent_sidereal_time_1() {
        // Example 12.a p.88 from Meeus, Astronomical Algorithms, 2nd ed.
        // 1987 April 10, 0h UT — apparent sidereal time = 13h 10m 46.1351s
        let date = Date::new(1987, 4, 10.0);
        let ast = get_apparent_sidereal_time_from_date(&date);
        // Expected in degrees: (13 + 10/60 + 46.1351/3600) * 15
        let expected = (13.0 + 10.0 / 60.0 + 46.1351 / 3600.0) * 15.0;
        assert!(
            (ast - expected).abs() < 1e-4,
            "apparent sidereal time = {ast:.6}°, expected ≈ {expected:.6}°"
        );
    }

    // #[test]
    // fn test_mean_sidereal_time_2() {
    // 	// Example 12.b p.89 from Meeus book 2nd edition
    // 	let date = Date::new(1987, 4, 10.0);
    // 	let mst = get_mean_sidereal_time_from_date(&date);
    // 	let expected_ra = RightAscension::new(13, 10, 46.3668);

    // 	assert_eq!(RightAscension::from_degrees(mst), expected_ra);
    // }
}
