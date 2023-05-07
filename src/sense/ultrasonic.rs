use crate::config::UltrasonicPin;
use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

pub struct UltrasonicSensor {
    trigger_pin: OutputPin,
    echo_pin: InputPin,
}

impl UltrasonicSensor {
    pub fn build(ultrasonic_pin: UltrasonicPin) -> Result<UltrasonicSensor, Box<dyn Error>> {
        Ok(UltrasonicSensor {
            trigger_pin: Gpio::new()?.get(ultrasonic_pin.trigger)?.into_output(),
            echo_pin: Gpio::new()?.get(ultrasonic_pin.echo)?.into_input(),
        })
    }

    pub fn get_distance(&mut self) -> f32 {
        self.trigger_pin.set_high();
        thread::sleep(Duration::from_millis(1));
        self.trigger_pin.set_low();
        let mut start_time: Instant = Instant::now();
        let mut duration: Duration = start_time.elapsed();
        while self.echo_pin.is_low() {
            start_time = Instant::now();
        }
        while self.echo_pin.is_high() {
            duration = start_time.elapsed();
        }
        duration.as_secs_f32() * 34300.0 / 2.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::sense::ultrasonic;
    use rppal::gpio::{Error, Pin};

    fn generate_test_ultrasonic_sensor() -> UltrasonicSensor {
        let ultrasonic_pin: UltrasonicPin = UltrasonicPin {
            trigger: 17,
            echo: 27,
        };
        UltrasonicSensor::build(ultrasonic_pin).unwrap()
    }

    #[test]
    fn build_should_init_pins() -> () {
        let _ultrasonic_sensor: UltrasonicSensor = generate_test_ultrasonic_sensor();
        let trigger_pin: Result<Pin, Error> = Gpio::new().unwrap().get(17);
        let echo_pin: Result<Pin, Error> = Gpio::new().unwrap().get(27);
        assert!(trigger_pin.is_err());
        assert!(echo_pin.is_err());
    }

    #[test]
    fn get_distance_should_return_distance_cm() -> () {
        let mut ultrasonic_sensor: UltrasonicSensor = generate_test_ultrasonic_sensor();
        let distance_a: f32 = ultrasonic_sensor.get_distance();
        thread::sleep(Duration::from_millis(1000));
        let distance_b: f32 = ultrasonic_sensor.get_distance();
        assert!((distance_a - distance_b) * (distance_a - distance_b) < 0.00001);
        assert!(distance_a < 100.0);
    }
}
