use alpyne_rust::a01::launch::launch;
use alpyne_rust::config::Config;

fn main() {
    let config: Config = Config::read("config.toml");
    launch(&config).unwrap();
}
