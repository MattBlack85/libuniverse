use crate::RightAscension;

pub fn ra_to_deg(ra: &RightAscension) -> f64 {
    let mut deg =
        (ra.hours as f64 + ra.minutes as f64 / 60 as f64 + ra.seconds as f64 / 3600 as f64)
            * 15 as f64;

    if deg > 180.0 {
        deg -= 360.0;
    }
    deg
}

#[cfg(test)]
mod tests {
    use crate::transform::ra_to_deg;
    use crate::RightAscension;

    #[test]
    fn test_2h_30m_45s() {
        let ra = RightAscension::new(2, 30, 45.0);
        assert_eq!(ra_to_deg(&ra), 37.6875);
    }

    #[test]
    fn test_23h_54m_21s() {
        let ra = RightAscension::new(23, 54, 21.0);
        assert_eq!(ra_to_deg(&ra), -1.4125000000000227);
    }
}
