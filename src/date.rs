use std::fmt::{Display, Formatter, Result};

use crate::julian_day::{get_julian_day, JulianDay};
use crate::HoursMinSec;

#[derive(PartialEq)]
pub struct Date {
    pub year: i16,
    pub month: u8,
    pub day: f64,
    pub hms: HoursMinSec,
}

impl Date {
    #[must_use]
    pub fn new(year: i16, month: u8, day: f64) -> Self {
        Self {
            year,
            month,
            day,
            hms: HoursMinSec {
                hours: 0,
                minutes: 0,
                seconds: 0.0,
            },
        }
    }

    /// Create a `Date` object from values: year, month, day, seconds, minutes, seconds.
    #[must_use]
    pub fn from_full_date(
        year: i16,
        month: u8,
        day: f64,
        hours: u8,
        minutes: u8,
        seconds: f64,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hms: HoursMinSec {
                hours,
                minutes,
                seconds,
            },
        }
    }

    #[must_use]
    pub fn to_julian_day(&self) -> JulianDay {
        let jd = get_julian_day(self);
        JulianDay::new(jd)
    }

    /// Returns the days interval between two dates
    #[must_use]
    pub fn interval(&self, other: &Self) -> f64 {
        let self_jd = self.to_julian_day();
        let other_jd = other.to_julian_day();

        if self_jd > other_jd {
            self_jd.get_value() - other_jd.get_value()
        } else {
            other_jd.get_value() - self_jd.get_value()
        }
    }

    /// Returns the day of the week of a calendar date (1 is Monday, 7 is Sunday)
    #[must_use]
    pub fn week_day(&self) -> u8 {
        let jd = self.to_julian_day();
        ((jd.get_value() + 1.5_f64) as i32 % 7) as u8
    }

    #[must_use]
    pub fn is_leap(&self) -> bool {
        false
    }

    #[must_use]
    pub fn year_day(&self) -> u16 {
        // todo implement is_leap()
        let k = {
            if self.is_leap() {
                1
            } else {
                2
            }
        };

        (f64::from(275_u16 * u16::from(self.month)) / 9.0_f64) as u16
            - k * (f64::from(self.month + 9) / 12.0_f64) as u16
            + (self.day as u16)
            - 30
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}-{}-{} {}:{}:{:.6}Z",
            self.year,
            self.month,
            self.day as u8,
            self.hms.hours,
            self.hms.minutes,
            self.hms.seconds
        )
    }
}

#[cfg(test)]
mod test {
    use crate::date::Date;

    #[test]
    fn test_date_to_julian_date() {
        let date = Date::new(1900, 1, 1.0);
        assert_eq!(date.to_julian_day().get_value(), 2_415_020.5);
    }

    #[test]
    fn test_date_intervals() {
        // From Meeus book "astronomical algorithms" p.64 example 7.d
        let date_1 = Date::new(1910, 4, 20.0);
        let date_2 = Date::new(1986, 2, 9.0);
        assert_eq!(date_1.interval(&date_2), 27689.0);
    }

    #[test]
    fn test_day_of_the_week_meeus() {
        // From Meeus book "astronomical algorithms" p.65 example 7.e
        let date = Date::new(1954, 6, 30.0);
        assert_eq!(date.week_day(), 3);
    }

    #[test]
    fn test_day_of_the_week_online_calculator() {
        let date = Date::new(478, 3, 11.0);
        assert_eq!(date.week_day(), 6);
    }

    #[test]
    fn test_day_of_the_year_meeus_book() {
        let date = Date::new(1978, 11, 14.0);
        assert_eq!(date.year_day(), 318);
    }

    #[test]
    fn test_date_with_hours_format() {
        let date = Date::from_full_date(2000, 6, 17.0, 8, 34, 57.0);
        assert_eq!(format!("{}", date), "2000-6-17 8:34:57.000000Z");
    }
}
