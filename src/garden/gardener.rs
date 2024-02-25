use esp32c3_hal::{
    adc::{AdcConfig, AdcPin, Attenuation, ADC, ADC1},
    analog::AvailableAnalog,
    gpio::{Analog, GpioPin, IO},
    prelude::*,
};

use super::garden_types::{GardenError, Humidity, WET_0_PERCENT, WET_100_PERCENT};

pub struct Gardener<'a> {
    pub adc1: ADC<'a, ADC1>,
    pub data_pin: AdcPin<GpioPin<Analog, 2>, ADC1>,
}

impl<'a> Gardener<'a> {
    #[allow(dead_code)]
    pub fn setup(io: IO, analog: AvailableAnalog) -> Gardener<'a> {
        let mut adc1_config = AdcConfig::new();

        let pin = adc1_config.enable_pin(io.pins.gpio2.into_analog(), Attenuation::Attenuation11dB);

        let adc1 = ADC::<ADC1>::adc(analog.adc1, adc1_config).unwrap();

        Gardener {
            adc1,
            data_pin: pin,
        }
    }

    #[allow(dead_code)]
    fn get_percent(humidity_value: Humidity) -> f32 {
        ((WET_0_PERCENT - u16::from(humidity_value)) as f32
            / (WET_0_PERCENT - WET_100_PERCENT) as f32)
            * 100.0
    }

    /// Return the humidity percentage after a single read
    #[allow(dead_code)]
    pub fn read_humidity(&mut self) -> Result<f32, GardenError> {
        let humidity_value = nb::block!(self.adc1.read(&mut self.data_pin))
            .map_err(|_| GardenError::ReadingFailed)?;
        let humidity_value = Humidity::new(humidity_value)?;

        Ok(Gardener::get_percent(humidity_value))
    }
}
