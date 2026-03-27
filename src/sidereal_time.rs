use crate::date::Date;
use crate::fit_degrees;
use crate::nutation;

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

/// Apparent sidereal time for the given date, in degrees.
///
/// Apparent sidereal time equals mean sidereal time plus the equation of the
/// equinoxes (Δψ cos ε₀), where Δψ is the full 63-term IAU 1980 nutation in
/// longitude computed via [`nutation::get_delta_psi`] — which skips the unused
/// Δε/cosine accumulation and batches arithmetic with AVX2 FMA where available.
///
/// Using mean obliquity ε₀ instead of true ε introduces < 0.02″ error in the
/// equation of the equinoxes — negligible for sidereal time purposes.
///
/// Meeus, *Astronomical Algorithms*, 2nd ed., Chapter 12, p. 87–88.
#[must_use]
pub fn get_apparent_sidereal_time_from_date(date: &Date) -> f64 {
    let jd = date.to_julian_day();
    let t = (jd.get_value() - 2_451_545_f64) / 36_525_f64;

    let mean_st = get_mean_sidereal_time_from_date(date);
    let delta_psi = nutation::get_delta_psi(&jd);
    let eq_of_equinoxes = delta_psi * mean_obliquity(t).to_radians().cos() / 3_600.0;

    fit_degrees(mean_st + eq_of_equinoxes)
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
