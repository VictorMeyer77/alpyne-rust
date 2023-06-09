use crate::config::MotorPin;
use rppal::gpio::{Gpio, OutputPin};
use std::error::Error;

pub enum Direction {
    Forward,
    Backward,
    Right,
    Left,
    Stop,
}

pub struct Motor {
    motor_one_enable_pin: OutputPin,
    motor_one_input_one_pin: OutputPin,
    motor_one_input_two_pin: OutputPin,
    motor_two_enable_pin: OutputPin,
    motor_two_input_one_pin: OutputPin,
    motor_two_input_two_pin: OutputPin,
}

impl Motor {
    pub fn build(
        motor_one_pin: &MotorPin,
        motor_two_pin: &MotorPin,
    ) -> Result<Motor, Box<dyn Error>> {
        Ok(Motor {
            motor_one_enable_pin: Gpio::new()?.get(motor_one_pin.enable)?.into_output(),
            motor_one_input_one_pin: Gpio::new()?.get(motor_one_pin.input_one)?.into_output(),
            motor_one_input_two_pin: Gpio::new()?.get(motor_one_pin.input_two)?.into_output(),
            motor_two_enable_pin: Gpio::new()?.get(motor_two_pin.enable)?.into_output(),
            motor_two_input_one_pin: Gpio::new()?.get(motor_two_pin.input_one)?.into_output(),
            motor_two_input_two_pin: Gpio::new()?.get(motor_two_pin.input_two)?.into_output(),
        })
    }

    pub fn drive(&mut self, direction: Direction) -> () {
        match direction {
            Direction::Forward => self.forward(),
            Direction::Backward => self.backward(),
            Direction::Right => self.right(),
            Direction::Left => self.left(),
            Direction::Stop => self.stop(),
        }
    }

    pub fn init(&mut self) -> () {
        self.motor_one_enable_pin.set_high();
        self.motor_two_enable_pin.set_high();
        self.stop();
    }

    fn forward(&mut self) -> () {
        self.motor_one_input_one_pin.set_high();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_high();
        self.motor_two_input_two_pin.set_low();
    }

    fn backward(&mut self) -> () {
        self.motor_one_input_one_pin.set_low();
        self.motor_one_input_two_pin.set_high();
        self.motor_two_input_one_pin.set_low();
        self.motor_two_input_two_pin.set_high();
    }

    fn right(&mut self) -> () {
        self.motor_one_input_one_pin.set_high();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_low();
        self.motor_two_input_two_pin.set_low();
    }

    fn left(&mut self) -> () {
        self.motor_one_input_one_pin.set_low();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_high();
        self.motor_two_input_two_pin.set_low();
    }

    fn stop(&mut self) -> () {
        self.motor_one_input_one_pin.set_low();
        self.motor_one_input_two_pin.set_low();
        self.motor_two_input_one_pin.set_low();
        self.motor_two_input_two_pin.set_low();
    }
}

impl Drop for Motor {
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

    fn generate_test_motor() -> Motor {
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
        Motor::build(&motor_one_pin, &motor_two_pin).unwrap()
    }

    #[test]
    fn build_should_init_output_pins() -> () {
        let _motor: Motor = generate_test_motor();
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

    #[test]
    fn init_should_set_pin_states() -> () {
        let mut motor: Motor = generate_test_motor();
        motor.init();
        assert!(motor.motor_one_enable_pin.is_set_high());
        assert!(motor.motor_one_input_one_pin.is_set_low());
        assert!(motor.motor_one_input_two_pin.is_set_low());
        assert!(motor.motor_two_enable_pin.is_set_high());
        assert!(motor.motor_two_input_one_pin.is_set_low());
        assert!(motor.motor_two_input_two_pin.is_set_low());
    }

    #[test]
    fn forward_should_set_pin_states() -> () {
        let mut motor: Motor = generate_test_motor();
        motor.drive(Direction::Forward);
        assert!(motor.motor_one_enable_pin.is_set_low());
        assert!(motor.motor_one_input_one_pin.is_set_high());
        assert!(motor.motor_one_input_two_pin.is_set_low());
        assert!(motor.motor_two_enable_pin.is_set_low());
        assert!(motor.motor_two_input_one_pin.is_set_high());
        assert!(motor.motor_two_input_two_pin.is_set_low());
    }

    #[test]
    fn backward_should_set_pin_states() -> () {
        let mut motor: Motor = generate_test_motor();
        motor.drive(Direction::Backward);
        assert!(motor.motor_one_enable_pin.is_set_low());
        assert!(motor.motor_one_input_one_pin.is_set_low());
        assert!(motor.motor_one_input_two_pin.is_set_high());
        assert!(motor.motor_two_enable_pin.is_set_low());
        assert!(motor.motor_two_input_one_pin.is_set_low());
        assert!(motor.motor_two_input_two_pin.is_set_high());
    }

    #[test]
    fn right_should_set_pin_states() -> () {
        let mut motor: Motor = generate_test_motor();
        motor.drive(Direction::Right);
        assert!(motor.motor_one_enable_pin.is_set_low());
        assert!(motor.motor_one_input_one_pin.is_set_high());
        assert!(motor.motor_one_input_two_pin.is_set_low());
        assert!(motor.motor_two_enable_pin.is_set_low());
        assert!(motor.motor_two_input_one_pin.is_set_low());
        assert!(motor.motor_two_input_two_pin.is_set_low());
    }

    #[test]
    fn left_should_set_pin_states() -> () {
        let mut motor: Motor = generate_test_motor();
        motor.drive(Direction::Left);
        assert!(motor.motor_one_enable_pin.is_set_low());
        assert!(motor.motor_one_input_one_pin.is_set_low());
        assert!(motor.motor_one_input_two_pin.is_set_low());
        assert!(motor.motor_two_enable_pin.is_set_low());
        assert!(motor.motor_two_input_one_pin.is_set_high());
        assert!(motor.motor_two_input_two_pin.is_set_low());
    }

    #[test]
    fn drop_should_disable_pin_states() -> () {
        {
            let _motor: Motor = generate_test_motor();
        }
        let motor_one_enable_pin: Result<Pin, Error> = Gpio::new().unwrap().get(5);
        let motor_one_input_one_pin: Result<Pin, Error> = Gpio::new().unwrap().get(6);
        let motor_one_input_two_pin: Result<Pin, Error> = Gpio::new().unwrap().get(13);
        let motor_two_enable_pin: Result<Pin, Error> = Gpio::new().unwrap().get(10);
        let motor_two_input_one_pin: Result<Pin, Error> = Gpio::new().unwrap().get(9);
        let motor_two_input_two_pin: Result<Pin, Error> = Gpio::new().unwrap().get(11);
        assert!(motor_one_enable_pin.is_ok());
        assert!(motor_one_input_one_pin.is_ok());
        assert!(motor_one_input_two_pin.is_ok());
        assert!(motor_two_enable_pin.is_ok());
        assert!(motor_two_input_one_pin.is_ok());
        assert!(motor_two_input_two_pin.is_ok());
    }
}
