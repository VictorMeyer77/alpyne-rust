use alpyne_rust::config::Config;
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
}
