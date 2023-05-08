use alpyne_rust::a01::launch::launch;
use alpyne_rust::config;
use alpyne_rust::config::Config;
use alpyne_rust::direction::Direction;
use alpyne_rust::sense::ultrasonic::UltrasonicSensor;
use std::thread;
use std::time::Duration;

fn main() {
    let config: Config = Config::read("config.toml");
    let mut direction: Direction =
        Direction::build(&config.motor_one_pin, &config.motor_two_pin).unwrap();
    direction.init();

    direction.forward();
    thread::sleep(Duration::from_millis(2000));

    direction.backward();
    thread::sleep(Duration::from_millis(2000));

    direction.right();
    thread::sleep(Duration::from_millis(1000));

    direction.left();
    thread::sleep(Duration::from_millis(2000));

    direction.stop(); /*
                      let mut t = UltrasonicSensor::build(&config.ultrasonic_pin).unwrap();
                      println!("{}", t.get_distance());
                      thread::sleep(Duration::from_millis(2000));
                      println!("{}", t.get_distance());
                      thread::sleep(Duration::from_millis(2000));
                      println!("{}", t.get_distance());
                      thread::sleep(Duration::from_millis(2000));
                      println!("{}", t.get_distance());
                      thread::sleep(Duration::from_millis(2000));*/

    //launch(&config).unwrap();
}
