use serde::Deserialize;
use std::fmt;
use std::fmt::Formatter;
use std::fs;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    motor_one_pin: MotorPin,
    motor_two_pin: MotorPin,
}

impl Config {
    pub fn read(path: &str) -> Config {
        let file_content: String = fs::read_to_string(path).unwrap();
        toml::from_str(file_content.as_str()).unwrap()
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Motor One Pin -> {}\n\
            Motor Two Pin -> {}",
            self.motor_one_pin, self.motor_two_pin
        )
    }
}

#[derive(Deserialize, Debug, PartialEq)]
struct MotorPin {
    enable: u8,
    input_one: u8,
    input_two: u8,
}

impl fmt::Display for MotorPin {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Enable {}, Input One {}, Input Two {}",
            self.enable, self.input_one, self.input_two
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    fn generate_test_config() -> Config {
        let motor_one_pin: MotorPin = MotorPin {
            enable: 1,
            input_one: 2,
            input_two: 3,
        };
        let motor_two_pin: MotorPin = MotorPin {
            enable: 4,
            input_one: 5,
            input_two: 6,
        };
        Config {
            motor_one_pin,
            motor_two_pin,
        }
    }

    #[test]
    fn read_config_should_return_config_from_file() -> () {
        let config_path: &str = "config-test.toml";
        let str_config: &str = "\
[motor_one_pin]
enable = 1
input_one = 2
input_two = 3

[motor_two_pin]
enable = 4
input_one = 5
input_two = 6
        ";
        let mut file = File::create(config_path).unwrap();
        file.write_all(str_config.as_bytes()).unwrap();
        assert_eq!(Config::read(config_path), generate_test_config());
        fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn fmt_should_return_pretty_config_string() -> () {
        let config: Config = generate_test_config();
        assert_eq!(
            config.to_string(),
            "\
Motor One Pin -> Enable 1, Input One 2, Input Two 3
Motor Two Pin -> Enable 4, Input One 5, Input Two 6"
        );
    }
}
