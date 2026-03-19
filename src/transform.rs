use crate::{DegMinSec, RightAscension};

// 15° per hour; 1° = 1/240 hour expressed in degrees → 15/3600
const DEG_PER_HOUR: f64 = 15.0;
const INV_240: f64 = 15.0 / 3600.0; // = 1/240

#[must_use]
pub fn ra_to_deg(ra: &RightAscension) -> f64 {
    // Accumulate total arcseconds then scale once: avoids two divisions (/60, /3600).
    let mut deg = (f64::from(ra.hours) * 3600.0
        + f64::from(ra.minutes) * 60.0
        + ra.seconds)
        * INV_240;

    if deg > 180.0 {
        deg -= 360.0;
    }
    deg
}

#[must_use]
pub fn deg_to_ra(deg: f64) -> RightAscension {
    let hours = (deg / DEG_PER_HOUR) as u8;
    // Cache the degree-equivalent of the whole hours to avoid recomputing it.
    let remainder = deg - f64::from(hours) * DEG_PER_HOUR;
    let minutes = (remainder * 4.0) as u8;
    // Replace /4 with *0.25 (exact representation) to avoid a division.
    let secs = (remainder - f64::from(minutes) * 0.25) * 240.0;

    RightAscension::new(hours, minutes, secs)
}

#[must_use]
pub fn dec_to_deg(dec: &DegMinSec) -> f64 {
    // Accumulate total arcseconds then scale once: avoids two divisions (/60, /3600).
    let degrees = (f64::from(dec.degrees) * 3600.0
        + f64::from(dec.minutes) * 60.0
        + dec.seconds)
        * (1.0 / 3600.0);

    if dec.negative {
        -degrees
    } else {
        degrees
    }
}

/// Utility to go easily from a decimal degree to a Degree-minutes
#[must_use]
pub fn deg_to_dms(degrees: f64) -> DegMinSec {
    let mut n_deg: i16 = degrees as i16;
    let mut n_minutes: f64 = 60_f64 * (degrees.abs() - f64::from(n_deg.abs()));
    let mut n_secs: f64 = 60.0 * (n_minutes - f64::from(n_minutes as u8));

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
    use crate::transform::{dec_to_deg, deg_to_dms, deg_to_ra, ra_to_deg};
    use crate::{Declination, DegMinSec, RightAscension};

    #[test]
    fn test_ra_2h_30m_45s() {
        let ra = RightAscension::new(2, 30, 45.0);
        assert_eq!(ra_to_deg(&ra), 37.6875);
    }

    #[test]
    fn test_deg_to_ra() {
        let ra = RightAscension::new(2, 30, 45.0);
        assert_eq!(deg_to_ra(37.6875), ra);
    }

    #[test]
    fn test_ra_23h_54m_21s() {
        let ra = RightAscension::new(23, 54, 21.0);
        assert_eq!(ra_to_deg(&ra), -1.412_500_000_000_022_7);
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
