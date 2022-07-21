use crate::julian_day::{JulianDay, get_julian_day};

pub struct Date {
    pub year: i16,
    pub month: u8,
    pub day: f64,
}

impl Date {
    pub fn new(year: i16, month: u8, day: f64) -> Self {
        Self { year, month, day }
    }

    pub fn to_julian_day(&self) -> JulianDay {
        let jd = get_julian_day(self);
        JulianDay::new(jd)
    }

    /// Returns the days interval between two dates
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
    pub fn week_day(&self) -> u8 {
        let jd = self.to_julian_day();
        ((jd.get_value() + 1.5_f64) as i32 % 7) as u8
    }

    pub fn is_leap(&self) -> bool {
        false
    }

    pub fn year_day(&self) -> u16 {
        // todo implement is_leap()
        let k = {
            if self.is_leap() {
                1
            } else {
                2
            }
        };

        ((275 as u16 * self.month as u16) as f64 / 9.0_f64) as u16
            - k * ((self.month + 9) as f64 / 12.0_f64) as u16
            + (self.day as u16)
            - 30
    }
}
