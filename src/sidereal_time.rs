use crate::date::Date;
use crate::fit_degrees;

#[must_use]
pub fn get_mean_sidereal_time_from_date(date: &Date) -> f64 {
    let jd = date.to_julian_day().get_value();
    let t = (&jd - 2_451_545_f64) / 36525_f64;
    let theta =
        280.460_618_37 + 360.985_647_366_29 * (&jd - 2_451_545_f64) + (0.000_387_933 * (t * t))
            - ((t * t * t) / 38_710_000_f64);
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
