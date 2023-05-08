use crate::config::Config;
use crate::direction::Direction;
use crate::sense::ultrasonic::UltrasonicSensor;
use std::error::Error;

pub fn launch(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut direction: Direction = Direction::build(&config.motor_one_pin, &config.motor_two_pin)?;
    let mut ultrasonic_sensor: UltrasonicSensor = UltrasonicSensor::build(&config.ultrasonic_pin)?;
    let mut history_dist: Vec<u16> = vec![];

    let current_dist: u16 = ultrasonic_sensor.get_distance() as u16;
    history_add(&mut history_dist, 5, current_dist);
    println!("{}", current_dist);
    if current_dist < 50 {
        let dir: u8 = rand::thread_rng().gen_range(0..2);
        if dir == 0 {
            direction.left();
        } else {
            direction.right();
        }
    } else if is_blocked(&history_dist) {
        direction.backward();
    } else {
        direction.forward();
    }

    Ok(())
}

fn is_blocked(history_dist: &Vec<u16>) -> bool {
    let mut history_dist_buffer: Vec<u16> = history_dist.clone();
    history_dist_buffer.dedup();
    history_dist_buffer.len() == 1
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
        let blocked_history: Vec<u16> = vec![1, 1, 1, 1, 1];
        let normal_history: Vec<u16> = vec![1, 2, 3, 4, 5];
        assert!(is_blocked(&blocked_history));
        assert!(!is_blocked(&normal_history));
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
