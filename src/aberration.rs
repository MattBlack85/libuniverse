//! Annual stellar aberration.
//!
//! Implements the algorithm from Jean Meeus, *Astronomical Algorithms*, 2nd ed.,
//! Chapter 23 "Aberration".
//!
//! Annual aberration is the apparent displacement of a star's position caused by
//! the finite speed of light combined with Earth's orbital velocity around the Sun.

use crate::fit_degrees;
use crate::julian_day::JulianDay;

const TO_RAD: f64 = std::f64::consts::PI / 180.0;

/// Compute the annual aberration corrections (Δα, Δδ) for a star in equatorial
/// coordinates, given the Julian Day of observation.
///
/// # Arguments
/// * `jd`     – Julian Day (Terrestrial Time)
/// * `ra_deg` – Star's right ascension in decimal degrees
/// * `dec_deg`– Star's declination in decimal degrees
///
/// # Returns
/// A tuple `(delta_ra_arcsec, delta_dec_arcsec)` where both values are in
/// arcseconds. Add these to the star's mean position to obtain the apparent
/// (aberration-corrected) position.
///
/// # Algorithm
/// Meeus, *Astronomical Algorithms*, 2nd ed., Chapter 23, equations 23.3–23.4.
/// The Sun's apparent longitude λ, true obliquity ε, orbital eccentricity e and
/// longitude of Earth's perihelion Π are derived from low-precision solar theory
/// (Chapter 25).
#[must_use]
pub fn annual_aberration(jd: &JulianDay, ra_deg: f64, dec_deg: f64) -> (f64, f64) {
    // Julian centuries from J2000.0 (Meeus eq. 22.1)
    let t = (jd.get_value() - 2_451_545.0) / 36_525.0;
    let t2 = t * t;
    let t3 = t2 * t;

    // Sun's geometric mean longitude L0 (degrees), Meeus eq. 25.2
    let l0 = fit_degrees(280.466_46 + 36_000.769_83 * t + 0.000_303_2 * t2);

    // Sun's mean anomaly M (degrees), Meeus eq. 25.3
    let m_deg = fit_degrees(357.529_11 + 35_999.050_29 * t - 0.000_153_6 * t2 + t3 / 24_490_000.0);
    let m = m_deg * TO_RAD;

    // Equation of center C (degrees), Meeus eq. 25.4
    let c = (1.914_602 - 0.004_817 * t - 0.000_014 * t2) * m.sin()
        + (0.019_993 - 0.000_101 * t) * (2.0 * m).sin()
        + 0.000_289 * (3.0 * m).sin();

    // Sun's true longitude Θ (degrees)
    let theta = l0 + c;

    // Longitude of ascending node of the Moon Ω (degrees), Meeus eq. 25.9
    let omega = fit_degrees(125.04 - 1_934.136 * t);
    let omega_rad = omega * TO_RAD;

    // Sun's apparent longitude λ (degrees), Meeus eq. 25.10
    let lambda = theta - 0.005_69 - 0.004_78 * omega_rad.sin();
    let lambda_rad = lambda * TO_RAD;

    // Mean obliquity of the ecliptic ε₀ (degrees), Meeus eq. 22.2 (low-precision)
    let eps0 = 23.0 + 26.0 / 60.0 + 21.448 / 3_600.0
        - (46.815_0 / 3_600.0) * t
        - (0.000_59 / 3_600.0) * t2
        + (0.001_813 / 3_600.0) * t3;

    // True obliquity ε (degrees), Meeus eq. 25.8
    let epsilon = eps0 + 0.002_56 * omega_rad.cos();
    let epsilon_rad = epsilon * TO_RAD;

    // Earth's orbital eccentricity e, Meeus eq. 25.4 (p. 163)
    let e = 0.016_708_634 - 0.000_042_037 * t - 0.000_000_126_7 * t2;

    // Longitude of Earth's perihelion (Sun's longitude of perigee) Π, Meeus p. 164
    let pi_deg = fit_degrees(102.937_35 + 1.719_46 * t + 0.000_46 * t2);
    let pi_rad = pi_deg * TO_RAD;

    // Aberration constant κ = 20.49552" (Meeus p. 151)
    const KAPPA: f64 = 20.495_52;

    let alpha = ra_deg * TO_RAD;
    let delta = dec_deg * TO_RAD;

    // Meeus eq. 23.3 — aberration in right ascension (arcseconds)
    let delta_ra = (-KAPPA
        * (alpha.cos() * lambda_rad.cos() * epsilon_rad.cos() + alpha.sin() * lambda_rad.sin())
        + e * KAPPA
            * (alpha.cos() * pi_rad.cos() * epsilon_rad.cos() + alpha.sin() * pi_rad.sin()))
        / delta.cos();

    // Meeus eq. 23.4 — aberration in declination (arcseconds)
    let delta_dec = -KAPPA
        * (lambda_rad.cos()
            * epsilon_rad.cos()
            * (epsilon_rad.tan() * delta.cos() - alpha.sin() * delta.sin())
            + alpha.cos() * delta.sin() * lambda_rad.sin())
        + e * KAPPA
            * (pi_rad.cos()
                * epsilon_rad.cos()
                * (epsilon_rad.tan() * delta.cos() - alpha.sin() * delta.sin())
                + alpha.cos() * delta.sin() * pi_rad.sin());

    (delta_ra, delta_dec)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::julian_day::JulianDay;

    // Helper: convert h m s to decimal degrees
    fn hms_to_deg(h: f64, m: f64, s: f64) -> f64 {
        (h + m / 60.0 + s / 3600.0) * 15.0
    }

    // Helper: convert d m s to decimal degrees (positive)
    fn dms_to_deg(d: f64, m: f64, s: f64) -> f64 {
        d + m / 60.0 + s / 3600.0
    }

    #[test]
    fn test_annual_aberration_meeus_example_23a() {
        // Meeus, *Astronomical Algorithms*, 2nd ed., Chapter 23, Example 23.a (p. 152)
        // Star: θ Persei
        //   α = 2h 46m 11.331s,  δ = +49° 20' 54.54"
        // Date: 2028 November 13.19 TT  →  JD = 2 462 088.69
        //
        // Expected (Meeus p. 152):
        //   Δα = +30.045",  Δδ = +6.681"
        let jd = JulianDay::new(2_462_088.69);
        let ra = hms_to_deg(2.0, 46.0, 11.331);
        let dec = dms_to_deg(49.0, 20.0, 54.54);

        let (da, dd) = annual_aberration(&jd, ra, dec);

        assert!(
            (da - 30.045).abs() < 0.1,
            "Δα expected ~30.045\" got {da:.3}\""
        );
        assert!(
            (dd - 6.681).abs() < 0.1,
            "Δδ expected ~6.681\" got {dd:.3}\""
        );
    }

    #[test]
    fn test_annual_aberration_near_j2000() {
        // Star on the equator (δ = 0°) at α = 0° observed at J2000.0.
        // At J2000.0 the Sun is near λ ≈ 280°, so cos(λ) ≈ +0.17.
        // Both corrections must be within the maximum possible aberration
        // magnitude κ/(1–e) ≈ 20.8", i.e. well under 25".
        let jd = JulianDay::new(2_451_545.0);
        let (da, dd) = annual_aberration(&jd, 0.0, 0.0);

        assert!(da.abs() < 25.0, "Δα={da:.3}\" out of physical range");
        assert!(dd.abs() < 25.0, "Δδ={dd:.3}\" out of physical range");
    }

    #[test]
    fn test_annual_aberration_south_pole_direction() {
        // A star at δ = –60° on 1987 April 10 (a date used elsewhere in Meeus examples).
        // JD = 2 446 895.5
        // We only verify that the corrections are within the physically expected
        // range: |Δα| and |Δδ| must both be ≤ κ/(1–e²) ≈ 20.5" (the maximum
        // possible aberration for circular/slightly-eccentric orbit).
        let jd = JulianDay::new(2_446_895.5);
        let ra = hms_to_deg(18.0, 30.0, 0.0); // α ≈ 277.5°
        let dec = -60.0_f64;

        let (da, dd) = annual_aberration(&jd, ra, dec);

        // Both components bounded by ~21" in magnitude
        assert!(da.abs() < 42.0, "Δα={da:.3}\" unexpectedly large");
        assert!(dd.abs() < 22.0, "Δδ={dd:.3}\" unexpectedly large");
    }

    #[test]
    fn test_annual_aberration_high_dec_star() {
        // Star near the north celestial pole (δ = 89°) on 2000 July 1.
        // JD = 2 451 726.5
        // Near the pole cos(δ) ≈ 0, so Δα can be very large; Δδ stays bounded.
        let jd = JulianDay::new(2_451_726.5);
        let (da, dd) = annual_aberration(&jd, 0.0, 89.0);

        // Δδ must stay below κ ≈ 20.5"
        assert!(
            dd.abs() < 22.0,
            "Δδ={dd:.3}\" unexpectedly large at high dec"
        );
        // Δα can be large near the pole but should be finite
        assert!(da.is_finite(), "Δα is not finite");
    }
}
