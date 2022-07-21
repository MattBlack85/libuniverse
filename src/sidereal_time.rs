use crate::date::Date;

fn get_mean_sidereal_time_from_date(date: &Date) -> f64 {
    let jd = date.to_julian_day().get_value();
    let t = (&jd - 2451545_f64) / 36525_f64;
    let theta = 280.46061837 + 360.98564736629 * (&jd - 2451545_f64) + (0.000387933 * (t * t)) - ((t * t * t) / 38710000_f64);
    dbg!("{}", theta);
    theta
}

#[cfg(test)]
mod test {
    use crate::RightAscension;
    use crate::date::Date;
    use crate::sidereal_time::get_mean_sidereal_time_from_date;

    #[test]
    fn test_mean_sidereal_time() {
	let date = Date::new(1987, 4, 10.0);
	let mst = get_mean_sidereal_time_from_date(&date);
	let expected_ra = RightAscension::new(13, 10, 46.3668);

	assert_eq!(RightAscension::from_degrees(mst), expected_ra);
    }
}