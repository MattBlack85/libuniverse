use crate::date::Date;

#[must_use]
pub fn delta_t(date: &Date) -> f64 {
    // We define the decimal year "y" as follows:

    let decimal_year: f64 = f64::from(date.year) + (f64::from(date.month) - 0.5) / 12_f64;

    // This gives "y" for the middle of the month, which is accurate enough given the precision in
    // the known values of ΔT. The following polynomial expressions can be used calculate the value
    // of ΔT (in seconds) over the time period covered by of the Five Millennium Canon of Solar Eclipses:
    // -1999 to +3000.

    let res = match decimal_year as i16 {
        // Before the year -500, calculate:
        -1999..=-501 => {
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
                - 5.952_053 * (u * u * u)
                - 0.179_845_2 * (u * u * u * u)
                + 0.022_174_192 * (u * u * u * u * u)
                + 0.009_031_652_1 * (u * u * u * u * u * u)
        }
        500..=1599 => {
            // Between years +500 and +1600, we again use the data from Table 1 to derive a polynomial of the 6th degree.
            let u = (decimal_year - 1000_f64) / 100_f64;
            1574.2 - 556.01 * u + 71.23472 * (u * u) + 0.319_781 * (u * u * u)
                - 0.850_346_3 * (u * u * u * u)
                - 0.005_050_998 * (u * u * u * u * u)
                + 0.008_357_207_3 * (u * u * u * u * u * u)
        }
        1600..=1699 => {
            // Between years +1600 and +1700, calculate:
            let t = decimal_year - 1600_f64;
            120_f64 - 0.9808 * t - 0.01532 * (t * t) + (t * t * t) / 7129_f64
        }
        1700..=1799 => {
            // Between years +1700 and +1800, calculate:
            let t = decimal_year - 1700_f64;
            8.83 + 0.1603 * t - 0.005_928_5 * (t * t) + 0.000_133_36 * (t * t * t)
                - (t * t * t * t) / 1_174_000_f64
        }
        1800..=1859 => {
            // Between years +1800 and +1860, calculate:
            let t = decimal_year - 1800_f64;
            13.72 - 0.332_447 * t + 0.006_861_2 * (t * t) + 0.004_111_6 * (t * t * t)
                - 0.000_374_36 * (t * t * t * t)
                + 0.000_012_127_2 * (t * t * t * t * t)
                - 0.000_000_169_9 * (t * t * t * t * t * t)
                + 0.000_000_000_875 * (t * t * t * t * t * t * t)
        }
        1860..=1899 => {
            let t = decimal_year - 1860_f64;
            7.62 + 0.5737 * t - 0.251_754 * (t * t) + 0.016_806_68 * (t * t * t)
                - 0.000_447_362_4 * (t * t * t * t)
                + (t * t * t * t * t) / 233_174_f64
        }
        1900..=1919 => {
            let t = decimal_year - 1900_f64;
            -2.79 + 1.494_119 * t - 0.059_893_9 * (t * t) + 0.006_196_6 * (t * t * t)
                - 0.000_197 * (t * t * t * t)
        }
        1920..=1940 => {
            let t = decimal_year - 1920_f64;
            21.20 + 0.84493 * t - 0.076_100 * (t * t) + 0.002_093_6 * (t * t * t)
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
            63.86 + 0.3345 * t - 0.060_374 * (t * t)
                + 0.001_727_5 * (t * t * t)
                + 0.000_651_814 * (t * t * t * t)
                + 0.000_023_735_99 * (t * t * t * t * t)
        }
        2005..=2049 => {
            let t = decimal_year - 2000_f64;
            62.92 + 0.32217 * t + 0.005_589 * (t * t)
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
