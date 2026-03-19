use crate::date::Date;
use crate::fit_degrees;

#[must_use]
pub fn get_mean_sidereal_time_from_date(date: &Date) -> f64 {
    let jd = date.to_julian_day().get_value();
    let d = jd - 2_451_545_f64;
    let t = d / 36525_f64;
    let t2 = t * t;

    // Formula: θ = 280.460618_37 + 360.985647366_29·D + 0.000387933·T² − T³/38710000
    // where D = JD − 2451545 and T = D/36525.
    // (Meeus, Astronomical Algorithms, 2nd ed., eq. 12.3)
    //
    // The T³ correction term is very small (~10⁻¹² per century³) but included for
    // correctness.  The dominant terms are the constant and the D-proportional term.
    //
    // On FMA-capable targets (x86_64 +fma, AArch64 base ISA) we use fused
    // multiply-add to reduce rounding error and instruction count.
    #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
    let theta = {
        // Factor the T² and T³ terms: t2*(0.000387933 - t/38710000)
        // = t2 * ((-1/38710000).mul_add(t, 0.000387933))
        let corr = (-1_f64 / 38_710_000_f64).mul_add(t, 0.000_387_933);
        // Combine: 280.460618_37 + 360.985647366_29*d + corr*t2
        t2.mul_add(corr, 360.985_647_366_29_f64.mul_add(d, 280.460_618_37))
    };

    #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
    let theta =
        280.460_618_37 + 360.985_647_366_29 * d + 0.000_387_933 * t2 - t2 * t / 38_710_000_f64;

    fit_degrees(theta)
}

#[cfg(test)]
mod test {
    use crate::date::Date;
    use crate::sidereal_time::get_mean_sidereal_time_from_date;
    use crate::RightAscension;

    #[test]
    fn test_mean_sidereal_time_1() {
        // Example 12.a p.88 from Meeus book 2nd edition
        let date = Date::new(1987, 4, 10.0);
        let mst = get_mean_sidereal_time_from_date(&date);
        let expected_ra = RightAscension::new(13, 10, 46.3668);

        assert_eq!(RightAscension::from_degrees(mst), expected_ra);
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
