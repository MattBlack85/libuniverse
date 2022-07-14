use std::fmt::{Display, Formatter, Result};

pub mod julian_day;
pub mod transform;

pub struct Date {
    pub year: i16,
    pub month: u8,
    pub day: f64,
}

impl Date {
    pub fn new(year: i16, month: u8, day: f64) -> Self {
        Self { year, month, day }
    }
}

/// Representation of right ascension coordinates (or RA shortly)
/// in hours, minutes and seconds.
pub struct RightAscension {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: f64,
}

impl RightAscension {
    pub fn new(h: u8, m: u8, s: f64) -> Self {
        Self {
            hours: h,
            minutes: m,
            seconds: s,
        }
    }
}

impl Display for RightAscension {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}h {}m {}s", self.hours, self.minutes, self.seconds)
    }
}

/// Representation of a position in degrees, minutes and seconds.
pub struct DegMinSec {
    pub negative: bool,
    pub degrees: i16,
    pub minutes: u16,
    pub seconds: f64,
}

impl DegMinSec {
    pub fn new(d: i16, m: u16, s: f64) -> Self {
        let deg;
        let neg;

        if d < 0 {
            neg = true;
            deg = d * -1;
        } else {
            neg = false;
            deg = d;
        };

        Self {
            negative: neg,
            degrees: deg,
            minutes: m,
            seconds: s,
        }
    }
}

impl Display for DegMinSec {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}° {}' {:.2}''",
            self.degrees, self.minutes, self.seconds
        )
    }
}

pub type Declination = DegMinSec;

struct EqPosition {
    ra: RightAscension,
    dec: Declination,
}

pub struct LongLatPosition {
    pub long: DegMinSec,
    pub lat: DegMinSec,
}

#[cfg(test)]
mod test {
    use crate::Declination;

    #[test]
    fn test_dec_display() {
        let dec = Declination::new(34, 21, 33.0);
        assert_eq!(format!("{}", dec), "34° 21' 33.00''")
    }
}
