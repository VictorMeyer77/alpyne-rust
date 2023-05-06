use alpyne_rust::config::Config;
use alpyne_rust::direction::Direction;
use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration; /*

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
                         }
                         */

fn main() -> Result<(), Box<dyn Error>> {
    //println!("{}", Config::read("config.toml"));
    let config: Config = Config::read("config.toml");
    let motor_one_pin = config.motor_one_pin;
    let mut enable = Gpio::new()?.get(motor_one_pin.enable)?.into_output();
    let mut ain_one = Gpio::new()?.get(motor_one_pin.input_one)?.into_output();
    let mut ain_two = Gpio::new()?.get(motor_one_pin.input_two)?.into_output();

    ain_one.set_high();
    ain_two.set_low();

    enable.set_high();

    thread::sleep(Duration::from_millis(2000));

    ain_one.set_low();
    enable.set_low();
    Ok(())
}
