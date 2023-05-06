use crate::config::MotorPin;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

pub struct Direction {
    motor_one_enable_pin: OutputPin,
    motor_one_input_one_pin: OutputPin,
    motor_one_input_two_pin: OutputPin,
    motor_two_enable_pin: OutputPin,
    motor_two_input_one_pin: OutputPin,
    motor_two_input_two_pin: OutputPin,
}

impl Direction {
    pub fn build(
        motor_one_pin: MotorPin,
        motor_two_pin: MotorPin,
    ) -> Result<Direction, Box<dyn Error>> {
        Ok(Direction {
            motor_one_enable_pin: Gpio::new()?.get(motor_one_pin.enable)?.into_output(),
            motor_one_input_one_pin: Gpio::new()?.get(motor_one_pin.input_one)?.into_output(),
            motor_one_input_two_pin: Gpio::new()?.get(motor_one_pin.input_two)?.into_output(),
            motor_two_enable_pin: Gpio::new()?.get(motor_two_pin.enable)?.into_output(),
            motor_two_input_one_pin: Gpio::new()?.get(motor_two_pin.input_one)?.into_output(),
            motor_two_input_two_pin: Gpio::new()?.get(motor_two_pin.input_two)?.into_output(),
        })
    }

    pub fn init(&mut self) -> () {
        self.motor_one_enable_pin.set_high();
        self.motor_two_enable_pin.set_high();
        self.stop();
    }

    pub fn forward(&mut self) -> () {
        self.motor_one_input_one_pin.set_high();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_high();
        self.motor_two_input_two_pin.set_low();
    }

    pub fn backward(&mut self) -> () {
        self.motor_one_input_one_pin.set_low();
        self.motor_one_input_two_pin.set_high();
        self.motor_two_input_one_pin.set_low();
        self.motor_two_input_two_pin.set_high();
    }

    pub fn right(&mut self) -> () {
        self.motor_one_input_one_pin.set_high();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_low();
        self.motor_two_input_two_pin.set_low();
    }

    pub fn left(&mut self) -> () {
        self.motor_one_input_one_pin.set_low();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_high();
        self.motor_two_input_two_pin.set_low();
    }

    pub fn stop(&mut self) -> () {
        self.motor_one_input_one_pin.set_low();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_low();
        self.motor_two_input_two_pin.set_low();
    }
}

impl Drop for Direction {
    fn drop(&mut self) -> () {
        self.stop();
        self.motor_one_enable_pin.set_low();
        self.motor_two_enable_pin.set_low();
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rppal::gpio::{Error, Pin};

    fn generate_test_direction() -> Direction {
        let motor_one_pin: MotorPin = MotorPin {
            enable: 5,
            input_one: 6,
            input_two: 13,
        };
        let motor_two_pin: MotorPin = MotorPin {
            enable: 10,
            input_one: 9,
            input_two: 11,
        };
        Direction::build(motor_one_pin, motor_two_pin).unwrap()
    }

    #[test]
    fn build_should_create_direction() -> () {
        let direction_buffer: Direction = generate_test_direction();
        let motor_one_enable_pin: Result<Pin, Error> = Gpio::new().unwrap().get(5);
        let motor_one_input_one_pin: Result<Pin, Error> = Gpio::new().unwrap().get(6);
        let motor_one_input_two_pin: Result<Pin, Error> = Gpio::new().unwrap().get(13);
        let motor_two_enable_pin: Result<Pin, Error> = Gpio::new().unwrap().get(10);
        let motor_two_input_one_pin: Result<Pin, Error> = Gpio::new().unwrap().get(9);
        let motor_two_input_two_pin: Result<Pin, Error> = Gpio::new().unwrap().get(11);
        assert!(motor_one_enable_pin.is_err());
        assert!(motor_one_input_one_pin.is_err());
        assert!(motor_one_input_two_pin.is_err());
        assert!(motor_two_enable_pin.is_err());
        assert!(motor_two_input_one_pin.is_err());
        assert!(motor_two_input_two_pin.is_err());
    }
}
