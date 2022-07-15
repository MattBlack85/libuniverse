use crate::{DegMinSec, RightAscension};

pub fn ra_to_deg(ra: &RightAscension) -> f64 {
    let mut deg =
        (ra.hours as f64 + ra.minutes as f64 / 60 as f64 + ra.seconds as f64 / 3600 as f64)
            * 15 as f64;

    if deg > 180.0 {
        deg -= 360.0;
    }
    deg
}

pub fn dec_to_deg(dec: &DegMinSec) -> f64 {
    let mut deg =
        dec.degrees as f64 + dec.minutes as f64 / 60 as f64 + dec.seconds as f64 / 3600 as f64;

    if dec.negative {
        deg *= -1.0;
    }

    deg
}

/// Utility to go easily from a decimal degree to a Degree-minutes
pub fn deg_to_dms(degrees: f64) -> DegMinSec {
    let mut n_deg: i16 = degrees as i16;
    let mut n_minutes: f64 = 60.0 * (degrees.abs() - n_deg.abs() as f64);
    let mut n_secs: f64 = 60.0 * (n_minutes - (n_minutes as u8) as f64);

    if n_secs > 59.0 {
        n_secs = 0.0;
        n_minutes += 1.0;
    }

    if n_minutes > 59.0 {
        n_minutes = 0.0;
        n_deg += 1;
    }

    DegMinSec::new(n_deg, n_minutes as u8, n_secs)
}

#[cfg(test)]
mod tests {
    use crate::transform::{dec_to_deg, deg_to_dms, ra_to_deg};
    use crate::{Declination, DegMinSec, RightAscension};

    #[test]
    fn test_ra_2h_30m_45s() {
        let ra = RightAscension::new(2, 30, 45.0);
        assert_eq!(ra_to_deg(&ra), 37.6875);
    }

    #[test]
    fn test_ra_23h_54m_21s() {
        let ra = RightAscension::new(23, 54, 21.0);
        assert_eq!(ra_to_deg(&ra), -1.4125000000000227);
    }

    #[test]
    fn test_dec_57_11_12() {
        let dec = Declination::new(57, 11, 12.0);
        assert_eq!(math::round::half_down(dec_to_deg(&dec), 4), 57.1867);
    }

    #[test]
    fn test_dec_min_81_7_59() {
        let dec = Declination::new(-81, 7, 59.0);
        assert_eq!(math::round::half_down(dec_to_deg(&dec), 4), -81.1331);
    }

    #[test]
    fn test_89_6078d_to89d_36m_28s() {
        let test_dms = DegMinSec::new(89, 36, 28.08);
        assert_eq!(deg_to_dms(89.6078), test_dms);
    }

    #[test]
    fn test_minus_59_1936d_to89d_36m_28s() {
        let test_dms = DegMinSec::new(-59, 11, 36.96);
        assert_eq!(deg_to_dms(-59.1936), test_dms);
    }
}
