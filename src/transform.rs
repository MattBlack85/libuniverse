use crate::{Declination, RightAscension};

pub fn ra_to_deg(ra: &RightAscension) -> f64 {
    let mut deg =
        (ra.hours as f64 + ra.minutes as f64 / 60 as f64 + ra.seconds as f64 / 3600 as f64)
            * 15 as f64;

    if deg > 180.0 {
        deg -= 360.0;
    }
    deg
}

pub fn dec_to_deg(dec: &Declination) -> f64 {
    let mut deg =
        dec.degrees as f64 + dec.minutes as f64 / 60 as f64 + dec.seconds as f64 / 3600 as f64;

    if dec.negative {
        deg *= -1.0;
    }

    deg
}

#[cfg(test)]
mod tests {
    use crate::transform::{dec_to_deg, ra_to_deg};
    use crate::{Declination, RightAscension};

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
}
