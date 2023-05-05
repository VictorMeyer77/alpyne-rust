/*use alpyne_rust::config::Config;
use alpyne_rust::direction::Direction;
use std::thread;
use std::time::Duration;

fn main() {
    //println!("{}", Config::read("config.toml"));
    let config: Config = Config::read("config.toml");
    let mut direction = Direction::build(config.motor_one_pin, config.motor_two_pin).unwrap();
    direction.stop();
    println!("{:?}", direction.pwn_one.pulse_width());

    direction.forward();
    thread::sleep(Duration::from_millis(2000));
    println!("{:?}", direction.pwn_one.frequency());
    direction.backward();
    thread::sleep(Duration::from_millis(2000));
    println!("{:?}", direction.pwn_one.frequency());
    direction.right();
    thread::sleep(Duration::from_millis(1000));
    direction.left();
    println!("{:?}", direction.pwn_one.frequency());
    thread::sleep(Duration::from_millis(2000));
    direction.stop();
}*/

// pwm_servo.rs - Rotates a servo using hardware PWM.
//
// Calibrate your servo beforehand, and change the values listed below to fall
// within your servo's safe limits to prevent potential damage. Don't power the
// servo directly from the Pi's GPIO header. Current spikes during power-up and
// stalls could otherwise damage your Pi, or cause your Pi to spontaneously
// reboot, corrupting your microSD card. If you're powering the servo using a
// separate power supply, remember to connect the grounds of the Pi and the
// power supply together.
//
// Interrupting the process by pressing Ctrl-C causes the application to exit
// immediately without disabling the PWM channel. Check out the
// gpio_blinkled_signals.rs example to learn how to properly handle incoming
// signals to prevent an abnormal termination.

use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;

use alpyne_rust::config::Config;
use rppal::pwm::{Channel, Polarity, Pwm};

// Servo configuration. Change these values based on your servo's verified safe
// minimum and maximum values.
//
// Period: 20 ms (50 Hz). Pulse width: min. 1200 µs, neutral 1500 µs, max. 1800 µs.

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::read("config.toml");
    let motor_one_pin = config.motor_one_pin;
    let mut motor_one_enable_pin = Gpio::new()?.get(motor_one_pin.enable)?.into_output();
    let mut motor_one_input_one_pin = Gpio::new()?.get(motor_one_pin.input_one)?.into_output();
    let mut motor_one_input_two_pin = Gpio::new()?.get(motor_one_pin.input_two)?.into_output();
    let mut pwn_one = Pwm::new(Channel::Pwm0)?;
    pwn_one.set_frequency(100.0, 100.0);
    motor_one_enable_pin.set_low();
    motor_one_input_one_pin.set_low();
    motor_one_input_two_pin.set_low();
    motor_one_input_one_pin.set_high();
    thread::sleep(Duration::from_millis(9000));
    Ok(())
}
