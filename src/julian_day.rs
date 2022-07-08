use crate::Date;

pub fn get_julian_day(date: &Date) -> f64 {
    let year;
    let month;

    match date.month {
        3..=12 => {
            year = date.year;
            month = date.month;
        }
        1..=2 => {
            year = date.year + 1;
            month = date.month + 12;
        }
        _ => panic!("Error"),
    }

    let b: i16;

    if year > 1582 || (year == 1582 && (month > 10 || (month == 10 && date.day >= 4.0))) {
        // Gregorian calendar
        b = 2 - (year / 100) + ((year / 100) / 4);
    } else {
        // Julian calendar
        b = 0;
    }

    let left_side = (365.25 as f64 * (year + 4716) as f64) as i64;
    let right_side = (30.6 as f64 * (month + 1) as f64) as i64;

    left_side as f64 + right_side as f64 + date.day + b as f64 - 1524.5
}

#[cfg(test)]
mod test {
    use crate::julian_day::get_julian_day;
    use crate::Date;

    #[test]
    fn test_sputnik_launch_date_to_julian_date() {
        let date = Date::new(1957, 10, 4.81);
        assert_eq!(2436116.31, get_julian_day(&date));
    }

    #[test]
    fn test_jd_0() {
        let date = Date::new(-4712, 1, 1.5);
        assert_eq!(0.0, get_julian_day(&date));
    }

    #[test]
    fn test_1999_1_1() {
        let date = Date::new(1999, 1, 1.0);
        assert_eq!(2451179.5, get_julian_day(&date));
    }

    #[test]
    fn test_1600_12_31() {
        let date = Date::new(1600, 12, 31.0);
        assert_eq!(2305812.5, get_julian_day(&date));
    }

    #[test]
    fn test_minus_1000_7_12_dot_5() {
        let date = Date::new(-1000, 7, 12.5);
        assert_eq!(1356001.0, get_julian_day(&date));
    }
}
