use alpyne_rust::config::Config;

fn main() {
    println!("{}", Config::read("config.toml"));
}
