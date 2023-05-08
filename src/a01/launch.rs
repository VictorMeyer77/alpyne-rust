use crate::config::Config;
use crate::direction::Direction;
use crate::sense::ultrasonic::UltrasonicSensor;
use rand::Rng;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub fn launch(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut direction: Direction = Direction::build(&config.motor_one_pin, &config.motor_two_pin)?;
    let mut ultrasonic_sensor: UltrasonicSensor = UltrasonicSensor::build(&config.ultrasonic_pin)?;
    let mut history_dist: Vec<u16> = vec![];

    direction.init();
    a01_loop(&mut direction, &mut ultrasonic_sensor, &mut history_dist);

    Ok(())
}

fn a01_loop(
    direction: &mut Direction,
    ultrasonic_sensor: &mut UltrasonicSensor,
    history_dist: &mut Vec<u16>,
) -> () {
    let history_max_size: usize = 4;
    let current_dist: u16 = ultrasonic_sensor.get_distance() as u16;
    history_add(history_dist, history_max_size, current_dist);
    println!("{}", current_dist);
    if is_blocked(&history_dist, history_max_size) {
        direction.backward();
        thread::sleep(Duration::from_millis(1500));
    } else if current_dist < 50 {
        let dir: u8 = rand::thread_rng().gen_range(0..=1);
        if dir == 0 {
            direction.left();
        } else {
            direction.right();
        }
        thread::sleep(Duration::from_millis(1000));
    } else {
        direction.forward();
        thread::sleep(Duration::from_millis(500));
    }
    a01_loop(direction, ultrasonic_sensor, history_dist);
}

fn is_blocked(history_dist: &Vec<u16>, history_max_size: usize) -> bool {
    let low_dist: usize = history_dist.iter().filter(|x| **x < 10).count();
    let high_dist: usize = history_dist.iter().filter(|x| **x > 1000).count();
    let history_max = history_dist.iter().max();
    let history_min = history_dist.iter().min();
    let is_blocked_a: bool = history_max.is_some()
        && history_min.is_some()
        && (history_max.unwrap() - history_min.unwrap() < 5);
    let is_blocked_b: bool = low_dist > 0 && high_dist > 0;
    history_dist.len() == history_max_size && (is_blocked_a || is_blocked_b)
}

fn history_add(history_dist: &mut Vec<u16>, history_max_size: usize, value: u16) -> () {
    if history_dist.len() < history_max_size {
        history_dist.push(value);
    } else {
        history_dist.remove(0);
        history_dist.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_blocked_should_return_true_when_history_distance_is_same() -> () {
        let blocked_history_a: Vec<u16> = vec![1, 1, 1, 1, 1];
        let blocked_history_b: Vec<u16> = vec![1, 2, 1, 3, 1];
        let blocked_history_c: Vec<u16> = vec![5, 1200, 6, 1200, 8];
        let normal_history: Vec<u16> = vec![10, 21, 11, 24, 35];
        assert!(is_blocked(&blocked_history_a, 5));
        assert!(is_blocked(&blocked_history_b, 5));
        assert!(is_blocked(&blocked_history_c, 5));
        assert!(!is_blocked(&normal_history, 5));
    }

    #[test]
    fn history_add_should_append_value_in_history() -> () {
        let mut partial_history: Vec<u16> = vec![1, 2, 3];
        let mut full_history: Vec<u16> = vec![1, 2, 3, 4, 5];
        history_add(&mut partial_history, 5, 15);
        history_add(&mut full_history, 5, 15);
        assert_eq!(partial_history, vec![1, 2, 3, 15]);
        assert_eq!(full_history, vec![2, 3, 4, 5, 15]);
    }
}
