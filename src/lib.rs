extern crate derive_more;
extern crate num_traits;

use derive_more::*;
use num_traits::*;
use std::iter::Sum;

#[derive(Add, Div, Mul, Sub, PartialEq, PartialOrd, Debug)]
pub struct Celsius<Value: Num>(Value);

#[derive(Add, Div, Mul, Sub, PartialEq, PartialOrd, Debug)]
pub struct Kelvin<Value: Num>(Value);

#[derive(Add, Div, Mul, Sub, PartialEq, PartialOrd, Debug)]
pub struct Fahrenheit<Value: Num>(Value);

impl Into<Fahrenheit<f64>> for Celsius<f64> {
    fn into(self) -> Fahrenheit<f64> {
        Fahrenheit(((self.0 * 1.8 + 32.0) * 1e6).round() / 1e6)
    }
}

pub const ABSOLUTE_ZERO_TEMPERATURE: Celsius<f64> = Celsius(-273.15);

pub fn mean_temperature(values: Vec<Celsius<f64>>) -> Celsius<f64> {
    let len = values.len();
    if len == 0 { Celsius(0.0) } else {
        let sum: Celsius<f64> = values.into_iter().sum();
        sum / (len as f64)
    }
}

impl<Value: Num> Sum for Celsius<Value> {
    fn sum<I: Iterator<Item=Celsius<Value>>>(iter: I) -> Self {
        iter.fold(Celsius(Value::zero()), |sum, num| sum + num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn temperature_construct_and_extract() {
        let water_temperature = Celsius(42.0);
        let water_temperature_value = water_temperature.0;

        assert_eq!(
            42.0,
            water_temperature_value
        );
    }

    #[test]
    fn temperature_conversion() {
        assert_eq!(
            Fahrenheit(-459.67),
            ABSOLUTE_ZERO_TEMPERATURE.into()
        );
    }

    #[test]
    fn temperature_compare() {
        assert!(
            Celsius(2) > Celsius(1)
        );
    }

    #[test]
    fn mean_temp() {
        assert_eq!(
            Celsius(11.5),
            mean_temperature(vec![Celsius(11.3), Celsius(12.3), Celsius(10.9)])
        );
    }
}
