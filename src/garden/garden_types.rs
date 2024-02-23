use thiserror_no_std::Error;

pub const WET_100_PERCENT: u16 = 3100;
pub const WET_0_PERCENT: u16 = 4095;

#[derive(Debug)]
pub struct Humidity(u16);

impl Humidity {
    pub fn new(value: u16) -> Result<Humidity, GardenError> {
        if value < WET_100_PERCENT || value > WET_0_PERCENT {
            return Err(GardenError::InvalidValue(value))
        }

        Ok(Humidity(value))
    }
}

impl From<Humidity> for u16 {
    fn from(value: Humidity) -> Self {
        value.0
    }
}

/// This error type is strictly related to sensor interaction
#[derive(Error, Debug)]
pub enum GardenError {
    #[error("Failed to read value")]
    ReadingFailed,

    #[error("Reading: {0} is outside the normal reading range: 3100 -> 4095")]
    InvalidValue(u16),
}