use alpyne_rust::config;

fn main() {
    println!("{}", config::read_config("config.toml"));
}
