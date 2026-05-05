use std::fmt::{Display, Formatter, Result};

use regex::Regex;

pub mod date;
pub mod dynamical_time;
pub mod julian_day;
pub mod moon;
pub mod nutation;
pub mod sidereal_time;
pub mod transform;

/// Rework a big angle so it can fit in the standard range 0-360
pub fn fit_degrees(orig_angle: f64) -> f64 {
    let mut a = orig_angle % 360.0;
    if a < 0.0 {
        a += 360.0;
    }
    a
}

/// Representation of right ascension coordinates (or RA shortly)
/// in hours, minutes and seconds.
#[derive(Debug)]
pub struct HoursMinSec {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: f64,
}

impl HoursMinSec {
    #[must_use]
    pub fn new(h: u8, m: u8, s: f64) -> Self {
        Self {
            hours: h,
            minutes: m,
            seconds: s,
        }
    }

    #[must_use]
    pub fn from_degrees(deg: f64) -> Self {
        transform::deg_to_ra(deg)
    }

    /// Tries to parse a RA position from a string
    /// DRAGONS AHEAD!
    /// Using this method implies you possibly already knows that the format
    /// is accepted, this method panics if the string doesn't match the regex.
    #[must_use]
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

impl Display for HoursMinSec {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}h {}m {}s", self.hours, self.minutes, self.seconds)
    }
}

impl PartialEq for HoursMinSec {
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
    #[must_use]
    pub fn new(d: i16, m: u8, s: f64) -> Self {
        let degrees;
        let neg;

        if d < 0 {
            neg = true;
            degrees = -d;
        } else {
            neg = false;
            degrees = d;
        };

        Self {
            negative: neg,
            degrees,
            minutes: m,
            seconds: s,
        }
    }

    #[must_use]
    pub fn from_degrees(deg: f64) -> Self {
        transform::deg_to_dms(deg)
    }

    /// Tries to parse a DMS position from a string
    /// DRAGONS AHEAD!
    /// Using this method implies you possibly already knows that the format
    /// is accepted, this method panics if the string doesn't match the regex.
    #[must_use]
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
pub type RightAscension = HoursMinSec;

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
    use crate::{Declination, EqPosition, RightAscension, fit_degrees};

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
        let dec2 = Declination::new(-28, 9, 44.08);
        assert_eq!(dec1, dec2);
    }

    #[test]
    fn test_eq_pos() {
        let ra = RightAscension::new(23, 44, 01.0);
        let dec = Declination::new(-28, 9, 44.08);
        let eq_pos = EqPosition::from_string("23 44 01", "-28 09 44.08");
        assert_eq!(eq_pos.ra, ra);
        assert_eq!(eq_pos.dec, dec);
        assert_eq!(format!("{}", eq_pos), "ra:23h 44m 1s dec:-28° 9' 44.08''");
    }

    #[test]
    fn test_small_negative_angle() {
        let a = fit_degrees(-0.0000000001);
        assert!((a - 360.0).abs() < 0.00000001f64);
    }

    #[test]
    fn test_just_bigger_than_360_angle() {
        let a = fit_degrees(360.0000000001);
        assert!((a - 0.0).abs() < 0.00001f64);
    }

    #[test]
    fn test_angle_normalization_invariant() {
        let inputs = [360.0000000001, 720.0, -0.0000000001, -360.0, 1080.5];

        for x in inputs {
            let a = fit_degrees(x);
            assert!(a >= 0.0 && a < 360.0);
        }
    }
}
