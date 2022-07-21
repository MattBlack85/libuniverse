use std::fmt::{Display, Formatter, Result};

use regex::Regex;

pub mod date;
pub mod julian_day;
pub mod sidereal_time;
pub mod transform;


/// Representation of right ascension coordinates (or RA shortly)
/// in hours, minutes and seconds.
#[derive(Debug)]
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

    pub fn from_degrees(deg: f64) -> Self {
        transform::deg_to_ra(deg)
    }

    /// Tries to parse a RA position from a string
    /// DRAGONS AHEAD!
    /// Using this method implies you possibly already knows that the format
    /// is accepted, this method panics if the string doesn't match the regex.
    pub fn from_string(s: &str) -> Self {
        let num_spaces_reg = Regex::new(r"^(\d{2}) (\d{2}) (\d{2}(\.\d{1,2})?)$").unwrap();
        let caps = num_spaces_reg.captures(s);

        if let Some(mat) = caps {
            return Self {
                hours: mat.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                minutes: mat.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                seconds: mat.get(3).unwrap().as_str().parse::<f64>().unwrap(),
            };
        }

        panic!("Cannot parse RA string")
    }
}

impl Display for RightAscension {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}h {}m {}s", self.hours, self.minutes, self.seconds)
    }
}

impl PartialEq for RightAscension {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours
            && self.minutes == other.minutes
            && math::round::half_up(self.seconds, 2) == math::round::half_up(other.seconds, 2)
    }
}

impl Eq for RightAscension {}

/// Representation of a position in degrees, minutes and seconds.
#[derive(Debug)]
pub struct DegMinSec {
    pub negative: bool,
    pub degrees: i16,
    pub minutes: u8,
    pub seconds: f64,
}

impl DegMinSec {
    pub fn new(d: i16, m: u8, s: f64) -> Self {
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

    pub fn from_degrees(deg: f64) -> Self {
        transform::deg_to_dms(deg)
    }

    /// Tries to parse a DMS position from a string
    /// DRAGONS AHEAD!
    /// Using this method implies you possibly already knows that the format
    /// is accepted, this method panics if the string doesn't match the regex.
    pub fn from_string(s: &str) -> Self {
        let num_spaces_reg = Regex::new(r"^(-?\d{2}) (\d{2}) (\d{2}(\.\d{1,2})?)$").unwrap();
        let caps = num_spaces_reg.captures(s);

        if let Some(mat) = caps {
            return Self::new(
                mat.get(1).unwrap().as_str().parse::<i16>().unwrap(),
                mat.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                mat.get(3).unwrap().as_str().parse::<f64>().unwrap(),
            );
        }

        panic!("Cannot parse DMS string")
    }
}

impl Display for DegMinSec {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.negative {
            write!(
                f,
                "-{}° {}' {:.2}''",
                self.degrees, self.minutes, self.seconds
            )
        } else {
            write!(
                f,
                "{}° {}' {:.2}''",
                self.degrees, self.minutes, self.seconds
            )
        }
    }
}

impl PartialEq for DegMinSec {
    fn eq(&self, other: &Self) -> bool {
        self.negative == other.negative
            && self.degrees == other.degrees
            && self.minutes == other.minutes
            && math::round::half_up(self.seconds, 2) == math::round::half_up(other.seconds, 2)
    }
}

impl Eq for DegMinSec {}

pub type Declination = DegMinSec;

struct EqPosition {
    ra: RightAscension,
    dec: Declination,
}

impl EqPosition {
    pub fn from_string(ra: &str, dec: &str) -> Self {
        Self {
            ra: RightAscension::from_string(ra),
            dec: Declination::from_string(dec),
        }
    }
}

impl Display for EqPosition {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ra:{} dec:{}", self.ra, self.dec)
    }
}

pub struct LongLatPosition {
    pub long: DegMinSec,
    pub lat: DegMinSec,
}

#[cfg(test)]
mod test {
    use crate::date::Date;
    use crate::{Declination, EqPosition, RightAscension};

    #[test]
    fn test_dec_display() {
        let dec = Declination::new(34, 21, 33.0);
        assert_eq!(format!("{}", dec), "34° 21' 33.00''");
    }

    #[test]
    fn test_dms_created_correctly_from_deg() {
        let dec = Declination::from_degrees(28.8103);
        assert_eq!(format!("{}", dec), "28° 48' 37.08''");
    }

    #[test]
    fn test_parse_ra_from_simple_str() {
        let ra1 = RightAscension::from_string("28 45 78.81");
        let ra2 = RightAscension::new(28, 45, 78.81);
        assert_eq!(ra1, ra2);
    }

    #[test]
    fn test_parse_dms_from_simple_str() {
        let dec1 = Declination::from_string("-28 09 44.08");
        let dec2 = Declination::new(-28, 09, 44.08);
        assert_eq!(dec1, dec2);
    }

    #[test]
    fn test_eq_pos() {
        let ra = RightAscension::new(23, 44, 01.0);
        let dec = Declination::new(-28, 09, 44.08);
        let eq_pos = EqPosition::from_string("23 44 01", "-28 09 44.08");
        assert_eq!(eq_pos.ra, ra);
        assert_eq!(eq_pos.dec, dec);
        assert_eq!(format!("{}", eq_pos), "ra:23h 44m 1s dec:-28° 9' 44.08''");
    }

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
}
