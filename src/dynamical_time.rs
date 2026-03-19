use crate::date::Date;

// Use the FMA-based Horner path on x86_64 (when +fma is enabled via .cargo/config.toml)
// and unconditionally on AArch64, where FMADD is part of the base ISA.
#[allow(unused_macros)]
macro_rules! horner_fma {
    // horner_fma!(x; a_n, a_n-1, ..., a_0)
    // Evaluates the polynomial a_0 + a_1*x + ... + a_n*x^n using Horner's method
    // with fused multiply-add instructions.
    ($x:expr; $first:expr $(, $rest:expr)*) => {{
        let mut acc: f64 = $first;
        $(acc = acc.mul_add($x, $rest);)*
        acc
    }};
}

#[must_use]
pub fn delta_t(date: &Date) -> f64 {
    // We define the decimal year "y" as follows:

    let decimal_year: f64 =
        f64::from(date.year) + (f64::from(date.month) - 0.5) * (1.0 / 12.0);

    // This gives "y" for the middle of the month, which is accurate enough given the precision in
    // the known values of ΔT. The following polynomial expressions can be used calculate the value
    // of ΔT (in seconds) over the time period covered by of the Five Millennium Canon of Solar Eclipses:
    // -1999 to +3000.

    let res = match decimal_year as i16 {
        // Before the year -500, calculate:
        -1999..=-501 => {
            let u = (decimal_year - 1820_f64) * (1.0 / 100.0);
            -20_f64 + 32_f64 * (u * u)
        }
        -500..=499 => {
            // Between years -500 and +500, we use the data from Table 1, except that for the year
            // -500 we changed the value 17190 to 17203.7 in order to avoid a discontinuity with the
            // previous formula at that epoch. The value for ΔT is given by a polynomial of
            // the 6th degree, which reproduces the values in Table 1 with an error not larger than 4 seconds.
            let u = decimal_year * (1.0 / 100.0);

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // Horner's method from highest to lowest degree:
                // 10583.6 - 1014.41*u + 33.78311*u² - 5.952053*u³ - 0.1798452*u⁴ + 0.022174192*u⁵ + 0.0090316521*u⁶
                horner_fma!(u; 0.009_031_652_1, 0.022_174_192, -0.179_845_2, -5.952_053, 33.783_11, -1014.41, 10583.6)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((((((0.009_031_652_1 * u + 0.022_174_192) * u
                    - 0.179_845_2) * u
                    - 5.952_053) * u
                    + 33.783_11) * u
                    - 1014.41) * u
                    + 10583.6)
            }
        }
        500..=1599 => {
            // Between years +500 and +1600, we again use the data from Table 1 to derive a polynomial of the 6th degree.
            let u = (decimal_year - 1000_f64) * (1.0 / 100.0);

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 1574.2 - 556.01*u + 71.23472*u² + 0.319781*u³ - 0.8503463*u⁴ - 0.005050998*u⁵ + 0.0083572073*u⁶
                horner_fma!(u; 0.008_357_207_3, -0.005_050_998, -0.850_346_3, 0.319_781, 71.234_72, -556.01, 1574.2)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((((((0.008_357_207_3 * u - 0.005_050_998) * u
                    - 0.850_346_3) * u
                    + 0.319_781) * u
                    + 71.234_72) * u
                    - 556.01) * u
                    + 1574.2)
            }
        }
        1600..=1699 => {
            // Between years +1600 and +1700, calculate:
            let t = decimal_year - 1600_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 120 - 0.9808*t - 0.01532*t² + t³/7129
                // = 120 + t*(-0.9808 + t*(-0.01532 + t/7129))
                horner_fma!(t; 1.0 / 7129.0, -0.015_32, -0.980_8, 120.0)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((t / 7129_f64 - 0.015_32) * t - 0.980_8) * t + 120.0
            }
        }
        1700..=1799 => {
            // Between years +1700 and +1800, calculate:
            let t = decimal_year - 1700_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 8.83 + 0.1603*t - 0.0059285*t² + 0.00013336*t³ - t⁴/1174000
                // = 8.83 + t*(0.1603 + t*(-0.0059285 + t*(0.00013336 - t/1174000)))
                horner_fma!(t; -1.0 / 1_174_000.0, 0.000_133_36, -0.005_928_5, 0.160_3, 8.83)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                (((-t / 1_174_000_f64 + 0.000_133_36) * t - 0.005_928_5) * t + 0.160_3) * t
                    + 8.83
            }
        }
        1800..=1859 => {
            // Between years +1800 and +1860, calculate:
            let t = decimal_year - 1800_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 13.72 - 0.332447*t + 0.0068612*t² + 0.0041116*t³ - 0.00037436*t⁴
                //       + 0.0000121272*t⁵ - 0.00000016990*t⁶ + 0.000000000875*t⁷
                horner_fma!(
                    t;
                    0.000_000_000_875,
                    -0.000_000_169_9,
                    0.000_012_127_2,
                    -0.000_374_36,
                    0.004_111_6,
                    0.006_861_2,
                    -0.332_447,
                    13.72
                )
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((((((0.000_000_000_875 * t - 0.000_000_169_9) * t
                    + 0.000_012_127_2) * t
                    - 0.000_374_36) * t
                    + 0.004_111_6) * t
                    + 0.006_861_2) * t
                    - 0.332_447) * t
                    + 13.72
            }
        }
        1860..=1899 => {
            let t = decimal_year - 1860_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 7.62 + 0.5737*t - 0.251754*t² + 0.01680668*t³ - 0.0004473624*t⁴ + t⁵/233174
                horner_fma!(
                    t;
                    1.0 / 233_174.0,
                    -0.000_447_362_4,
                    0.016_806_68,
                    -0.251_754,
                    0.573_7,
                    7.62
                )
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((((t / 233_174_f64 - 0.000_447_362_4) * t + 0.016_806_68) * t - 0.251_754) * t
                    + 0.573_7) * t
                    + 7.62
            }
        }
        1900..=1919 => {
            let t = decimal_year - 1900_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // -2.79 + 1.494119*t - 0.0598939*t² + 0.0061966*t³ - 0.000197*t⁴
                horner_fma!(t; -0.000_197, 0.006_196_6, -0.059_893_9, 1.494_119, -2.79)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                (((-0.000_197 * t + 0.006_196_6) * t - 0.059_893_9) * t + 1.494_119) * t - 2.79
            }
        }
        1920..=1940 => {
            let t = decimal_year - 1920_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 21.20 + 0.84493*t - 0.076100*t² + 0.0020936*t³
                horner_fma!(t; 0.002_093_6, -0.076_100, 0.844_93, 21.20)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((0.002_093_6 * t - 0.076_100) * t + 0.844_93) * t + 21.20
            }
        }
        1941..=1960 => {
            let t = decimal_year - 1950_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 29.07 + 0.407*t - t²/233 + t³/2547
                // = 29.07 + t*(0.407 + t*(-1/233 + t/2547))
                horner_fma!(t; 1.0 / 2547.0, -1.0 / 233.0, 0.407, 29.07)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((t / 2547_f64 - 1_f64 / 233_f64) * t + 0.407) * t + 29.07
            }
        }
        1961..=1985 => {
            let t = decimal_year - 1975_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 45.45 + 1.067*t - t²/260 - t³/718
                // = 45.45 + t*(1.067 + t*(-1/260 - t/718))
                horner_fma!(t; -1.0 / 718.0, -1.0 / 260.0, 1.067, 45.45)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((-t / 718_f64 - 1_f64 / 260_f64) * t + 1.067) * t + 45.45
            }
        }
        1986..=2004 => {
            let t = decimal_year - 2000_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 63.86 + 0.3345*t - 0.060374*t² + 0.0017275*t³ + 0.000651814*t⁴ + 0.00002373599*t⁵
                horner_fma!(
                    t;
                    0.000_023_735_99,
                    0.000_651_814,
                    0.001_727_5,
                    -0.060_374,
                    0.334_5,
                    63.86
                )
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                ((((0.000_023_735_99 * t + 0.000_651_814) * t + 0.001_727_5) * t - 0.060_374) * t
                    + 0.334_5) * t
                    + 63.86
            }
        }
        2005..=2049 => {
            let t = decimal_year - 2000_f64;

            #[cfg(any(target_feature = "fma", target_arch = "aarch64"))]
            {
                // 62.92 + 0.32217*t + 0.005589*t²
                horner_fma!(t; 0.005_589, 0.322_17, 62.92)
            }

            #[cfg(not(any(target_feature = "fma", target_arch = "aarch64")))]
            {
                62.92 + 0.322_17 * t + 0.005_589 * (t * t)
            }
        }
        2050..=2099 => {
            let t = (decimal_year - 1820_f64) * (1.0 / 100.0);
            -20_f64 + 32_f64 * (t * t) - 0.5628 * (2150_f64 - decimal_year)
        }
        2150..=2999 => {
            let u = (decimal_year - 1820_f64) * (1.0 / 100.0);
            -20_f64 + 32_f64 * (u * u)
        }
        _ => panic!("Not supported"),
    };

    res
}

#[cfg(test)]
mod test {
    use crate::date::Date;
    use crate::dynamical_time::delta_t;

    #[test]
    fn test_delta_t() {
        // From Meeus book 2nd edition, example 10.a p.78 - however the result is rounded to the nearest
        // integer because the calculation is using the NASA polynomial functions to get an exact delta t.
        let date = Date::from_full_date(1977, 2, 18.0, 3, 37, 40.0);
        let dt = delta_t(&date);
        assert_eq!(48, math::round::half_up(dt, 0) as i64)
    }

    #[test]
    fn test_lunar_eclipse_2022_11_08() {
        let date = Date::from_full_date(2022, 11, 8.0, 11, 00, 22.0);
        let dt = delta_t(&date);
        assert_eq!(73, math::round::half_up(dt, 0) as i64);
    }
}
