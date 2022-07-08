pub mod coordinates;
pub mod julian_day;

pub struct Date {
    pub year: i16,
    pub month: u8,
    pub day: f64,
}

impl Date {
    pub fn new(year: i16, month: u8, day: f64) -> Self {
        Self { year, month, day }
    }
}
