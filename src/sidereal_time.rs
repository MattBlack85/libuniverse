use crate::date::Date;
use crate::fit_degrees;
use crate::nutation;

#[must_use]
pub fn get_mean_sidereal_time_from_date(date: &Date) -> f64 {
    let jd = date.to_julian_day().get_value();
    let d = jd - 2_451_545_f64;
    let t = d / 36525.0;
    let t2 = t * t;
    let t3 = t2 * t;

    let theta = 280.460_618_37 + 360.985_647_366_29 * d + 0.000_387_933 * t2
        - (t3 * (1.0 / 38_710_000_f64));

    fit_degrees(theta)
}

<<<<<<< claude/implement-sidereal-time-YGcjd
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
=======
pub fn get_local_sidereal_time(date: &Date, longitude_deg: f64) -> f64 {
    let gmst = get_mean_sidereal_time_from_date(date);
    fit_degrees(gmst + longitude_deg)
>>>>>>> main
}

#[cfg(test)]
mod test {
    use crate::RightAscension;
    use crate::date::Date;
<<<<<<< claude/implement-sidereal-time-YGcjd
    use crate::sidereal_time::{
        get_apparent_sidereal_time_from_date, get_mean_sidereal_time_from_date,
    };
    use crate::RightAscension;
=======
    use crate::fit_degrees;
    use crate::sidereal_time::get_mean_sidereal_time_from_date;

    use super::get_local_sidereal_time;
>>>>>>> main

    #[test]
    fn test_mean_sidereal_time_1() {
        // Example 12.a p.88 from Meeus book 2nd edition
        let date = Date::new(1987, 4, 10.0);
        let mst = get_mean_sidereal_time_from_date(&date);
        let expected_ra = RightAscension::new(13, 10, 46.3668);

        assert_eq!(RightAscension::from_degrees(mst), expected_ra);
    }

    #[test]
<<<<<<< claude/implement-sidereal-time-YGcjd
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
=======
    fn test_mean_sidereal_time_2() {
        // Example 12.b p.89 from Meeus book 2nd edition
        let date = Date::new(1987, 4, 10.0);
        let mst = get_mean_sidereal_time_from_date(&date);
        let expected_ra = RightAscension::new(13, 10, 46.3668);

        assert_eq!(RightAscension::from_degrees(mst), expected_ra);
    }

    #[test]
    fn test_lst_j2000_longitude_0() {
        let date = Date::new(2000, 1, 1.5);
        let lst = get_local_sidereal_time(&date, 0.0);
        assert!((lst - 280.46061837).abs() < 1e-6);
    }

    #[test]
    fn test_lst_j2000_longitude_15() {
        let date = Date::new(2000, 1, 1.5);
        let lst = get_local_sidereal_time(&date, 15.0);
        assert!((lst - 295.46061837).abs() < 1e-6);
    }

    #[test]
    fn test_lst_j2000_1_2_longitude_0() {
        let date = Date::new(2000, 1, 2.0);
        let lst = get_local_sidereal_time(&date, 0.0);
        assert!((lst - 100.95344).abs() < 1e-5);
    }

    #[test]
    fn test_lst_2024_1_1_longitude_20() {
        let date = Date::new(2024, 1, 1.0);
        let lst = get_local_sidereal_time(&date, 20.0);
        println!("LST: {}", lst);
        assert!((lst - 120.1526).abs() < 1e-4);
    }

    #[test]
    fn test_lst_equals_gmst_at_greenwich() {
        let date = Date::from_full_date(2000, 1, 1.0, 12, 0, 0.0);
        let gmst = get_mean_sidereal_time_from_date(&date);
        let lst = get_local_sidereal_time(&date, 0.0);

        assert!((gmst - lst).abs() < 1e-10);
    }
>>>>>>> main

    #[test]
    fn test_lst_longitude_offset() {
        let date = Date::from_full_date(2000, 1, 1.0, 12, 0, 0.0);

        let lst0 = get_local_sidereal_time(&date, 0.0);
        let lst15 = get_local_sidereal_time(&date, 15.0);

        let diff = fit_degrees(lst15 - lst0);

        assert!((diff - 15.0).abs() < 1e-8);
    }

    #[test]
    fn test_lst_wraparound() {
        let date = Date::from_full_date(2000, 1, 1.0, 12, 0, 0.0);

        let lst = get_local_sidereal_time(&date, 200.0);
        assert!(lst >= 0.0 && lst < 360.0);
    }
}
