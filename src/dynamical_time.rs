use crate::date::Date;

pub fn delta_t(date: &Date) -> f64 {
    // We define the decimal year "y" as follows:

    let decimal_year: f64 = date.year as f64 + (date.month as f64 - 0.5) / 12_f64;

    // This gives "y" for the middle of the month, which is accurate enough given the precision in
    // the known values of ΔT. The following polynomial expressions can be used calculate the value
    // of ΔT (in seconds) over the time period covered by of the Five Millennium Canon of Solar Eclipses:
    // -1999 to +3000.

    let res = match decimal_year as i16 {
        // Before the year -500, calculate:
        -1999..=-499 => {
            let u = (decimal_year - 1820_f64) / 100_f64;
            -20_f64 + 32_f64 * (u * u)
        }
        -500..=499 => {
            // Between years -500 and +500, we use the data from Table 1, except that for the year
            // -500 we changed the value 17190 to 17203.7 in order to avoid a discontinuity with the
            // previous formula at that epoch. The value for ΔT is given by a polynomial of
            // the 6th degree, which reproduces the values in Table 1 with an error not larger than 4 seconds.
            let u = decimal_year / 100_f64;
            10583.6 - 1014.41 * u + 33.78311 * (u * u)
                - 5.952053 * (u * u * u)
                - 0.1798452 * (u * u * u * u)
                + 0.022174192 * (u * u * u * u * u)
                + 0.0090316521 * (u * u * u * u * u * u)
        }
        500..=1599 => {
            // Between years +500 and +1600, we again use the data from Table 1 to derive a polynomial of the 6th degree.
            let u = (decimal_year - 1000_f64) / 100_f64;
            1574.2 - 556.01 * u + 71.23472 * (u * u) + 0.319781 * (u * u * u)
                - 0.8503463 * (u * u * u * u)
                - 0.005050998 * (u * u * u * u * u)
                + 0.0083572073 * (u * u * u * u * u * u)
        }
        1600..=1699 => {
            // Between years +1600 and +1700, calculate:
            let t = decimal_year - 1600_f64;
            120_f64 - 0.9808 * t - 0.01532 * (t * t) + (t * t * t) / 7129_f64
        }
        1700..=1799 => {
            // Between years +1700 and +1800, calculate:
            let t = decimal_year - 1700_f64;
            8.83 + 0.1603 * t - 0.0059285 * (t * t) + 0.00013336 * (t * t * t)
                - (t * t * t * t) / 1174000_f64
        }
        1800..=1859 => {
            // Between years +1800 and +1860, calculate:
            let t = decimal_year - 1800_f64;
            13.72 - 0.332447 * t + 0.0068612 * (t * t) + 0.0041116 * (t * t * t)
                - 0.00037436 * (t * t * t * t)
                + 0.0000121272 * (t * t * t * t * t)
                - 0.0000001699 * (t * t * t * t * t * t)
                + 0.000000000875 * (t * t * t * t * t * t * t)
        }
        1860..=1899 => {
            let t = decimal_year - 1860_f64;
            7.62 + 0.5737 * t - 0.251754 * (t * t) + 0.01680668 * (t * t * t)
                - 0.0004473624 * (t * t * t * t)
                + (t * t * t * t * t) / 233174_f64
        }
        1900..=1919 => {
            let t = decimal_year - 1900_f64;
            -2.79 + 1.494119 * t - 0.0598939 * (t * t) + 0.0061966 * (t * t * t)
                - 0.000197 * (t * t * t * t)
        }
        1920..=1940 => {
            let t = decimal_year - 1920_f64;
            21.20 + 0.84493 * t - 0.076100 * (t * t) + 0.0020936 * (t * t * t)
        }
        1941..=1960 => {
            let t = decimal_year - 1950_f64;
            29.07 + 0.407 * t - (t * t) / 233_f64 + (t * t * t) / 2547_f64
        }
        1961..=1985 => {
            let t = decimal_year - 1975_f64;
            45.45 + 1.067 * t - (t * t) / 260_f64 - (t * t * t) / 718_f64
        }
        1986..=2004 => {
            let t = decimal_year - 2000_f64;
            63.86 + 0.3345 * t - 0.060374 * (t * t)
                + 0.0017275 * (t * t * t)
                + 0.000651814 * (t * t * t * t)
                + 0.00002373599 * (t * t * t * t * t)
        }
        2005..=2049 => {
            let t = decimal_year - 2000_f64;
            62.92 + 0.32217 * t + 0.005589 * (t * t)
        }
        2050..=2099 => {
            let t = (decimal_year - 1820_f64) / 100_f64;
            -20_f64 + 32_f64 * (t * t) - 0.5628 * (2150_f64 - decimal_year)
        }
        2150..=2999 => {
            let u = (decimal_year - 1820_f64) / 100_f64;
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
