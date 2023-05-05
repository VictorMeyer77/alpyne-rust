use alpyne_rust::config::Config;
use alpyne_rust::direction::Direction;
use std::thread;
use std::time::Duration;

fn main() {
    //println!("{}", Config::read("config.toml"));
    let config: Config = Config::read("config.toml");
    let mut direction = Direction::build(config.motor_one_pin, config.motor_two_pin).unwrap();

    direction.forward();
    thread::sleep(Duration::from_millis(2000));
    direction.backward();
    thread::sleep(Duration::from_millis(2000));
    direction.right();
    thread::sleep(Duration::from_millis(1000));
    direction.left();
    thread::sleep(Duration::from_millis(2000));
    direction.stop();
}
