use crate::date::Date;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct JulianDay {
    value: f64,
}

impl JulianDay {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn from_date(date: &Date) -> Self {
        Self {
            value: get_julian_day(date),
        }
    }

    pub fn to_modified_jd(&self) -> f64 {
        self.value - 2_400_000.5_f64
    }

    pub fn to_calendar_date(&self) -> Date {
        let a: i32;

        let jd_plus_half = self.value + 0.5_f64;
        let z: i32 = jd_plus_half as i32;
        let f: f64 = jd_plus_half - z as f64;

        if z < 2_299_161 {
            a = z;
        } else {
            let alfa = ((z as f64 - 1_867_216.25_f64) / 36524.25) as i32;
            a = z + 1 + alfa - (alfa as f64 / 4_f64) as i32;
        }

        let b: i32 = a + 1524;
        let c: i16 = ((f64::from(b) - 122.1_f64) / 365.25) as i16;
        let d: i32 = (365.25_f64 * f64::from(c)) as i32;
        let e: u8 = ((b - d) as f64 / 30.6001_f64) as u8;

        let day: f64 = (b - d - (30.6001_f64 * f64::from(e)) as i32) as f64 + f;

        let month: u8 = {
            if e < 14 {
                e - 1
            } else {
                e - 13
            }
        };

        let year: i16 = {
            if month > 2 {
                c - 4716
            } else {
                c - 4715
            }
        };

        Date::new(year, month, day)
    }
}

pub fn get_julian_day(date: &Date) -> f64 {
    let year;
    let month;

    match date.month {
        3..=12 => {
            year = date.year;
            month = date.month;
        }
        1..=2 => {
            year = date.year - 1;
            month = date.month + 12;
        }
        _ => panic!("Error"),
    }

    let b: i16 =
        if year > 1582 || (year == 1582 && (month > 10 || (month == 10 && date.day >= 4.0))) {
            // Gregorian calendar
            2 - (year / 100) + ((year / 100) / 4)
        } else {
            // Julian calendar
            0
        };

    let left_side = (365.25_f64 * f64::from(year + 4716)) as i64;
    let right_side = (30.6001_f64 * f64::from(month + 1)) as i64;

    left_side as f64 + right_side as f64 + date.day + b as f64 - 1524.5_f64
}

#[cfg(test)]
mod test {
    use crate::date::Date;
    use crate::julian_day::{get_julian_day, JulianDay};

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

    #[test]
    fn test_whole_jd_to_calendar_date() {
        // 2459581 is 2022-01-01 12:00:00 UTC
        let jd = JulianDay::new(2459581.0);
        let date = jd.to_calendar_date();
        assert_eq!(date.year, 2022);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 1.5);
    }

    #[test]
    fn test_fractional_jd_to_calendar_date() {
        // From Meeus book "astronomical algorithms" p. 64 example 7.c
        // 2436116.31 is October 4.81, 1957
        let jd = JulianDay::new(2436116.31);
        let date = jd.to_calendar_date();
        assert_eq!(date.year, 1957);
        assert_eq!(date.month, 10);
        assert_eq!(math::round::half_up(date.day, 2), 4.81);
    }

    #[test]
    fn test_more_jd_to_calendar_date() {
        // From Meeus book "astronomical algorithms" p. 64 excercises
        let jd1 = JulianDay::new(1842713.0);
        let date1 = jd1.to_calendar_date();
        assert_eq!(date1.year, 333);
        assert_eq!(date1.month, 1);
        assert_eq!(math::round::half_up(date1.day, 2), 27.5);

        let jd2 = JulianDay::new(1507900.13);
        let date2 = jd2.to_calendar_date();
        assert_eq!(date2.year, -584);
        assert_eq!(date2.month, 5);
        assert_eq!(math::round::half_up(date2.day, 2), 28.63);
    }

    #[test]
    fn test_jd_to_modified_jd() {
        let jd = JulianDay::new(2436116.31);
        assert_eq!(jd.get_value() - 2_400_000.5, jd.to_modified_jd());
    }

    #[test]
    fn test_jd_equality() {
        assert_eq!(JulianDay::new(2436116.31), JulianDay::new(2436116.31));
        assert_eq!(JulianDay::new(2000.0) > JulianDay::new(1000.0), true);
        assert_eq!(
            JulianDay::new(139_164.0) < JulianDay::new(2_451_911.0),
            true
        );
    }
}
