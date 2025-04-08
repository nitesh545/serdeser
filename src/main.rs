use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    settings: Settings,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    server: String,
    name: String,
    port: i32,
}

// load file
fn load_config(path: &str) -> Config {
    let value = fs::read_to_string(path).expect("could not read from file");
    let cfg: Config = toml::from_str(&value).expect("could not serialize in load_config");
    cfg
}

// update values
fn update_config(value: Config) -> Config {
    Config {
        settings: Settings {
            server: String::from("127.0.0.2"),
            ..value.settings
        },
    }
}

// save file
fn save_cofig(path: &str, value: Config) {
    let contents = toml::to_string_pretty(&value).expect("could not serialize in save config");
    fs::write(path, contents).expect("could not write to file");
}

fn main() {
    let value = load_config("src/config.toml");
    let updated_value = update_config(value);
    save_cofig("src/config.toml", updated_value);
    println!("Value updated successfully");
}
