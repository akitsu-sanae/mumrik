use serde_derive::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::sync::Mutex;
use util;

pub const FILENAME: &str = "mumrik-conf.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub build: BuildConfig,
}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::load());
}

impl Config {
    fn default() -> Config {
        Config {
            build: BuildConfig::default(),
        }
    }

    fn load() -> Self {
        match config_path() {
            Some(config_path) => {
                let mut config = String::new();
                let mut f = BufReader::new(File::open(config_path).unwrap());
                f.read_to_string(&mut config).unwrap();
                toml::from_str(config.as_str()).unwrap_or_else(|err| {
                    eprintln!("{}: {}", util::alert("invalid config file"), err);
                    std::process::exit(-1);
                })
            }
            None => Config::default(),
        }
    }
}

pub fn config_path() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().unwrap();
    while {
        let mut config_path = dir.clone();
        config_path.push(FILENAME);
        if config_path.as_path().is_file() {
            return Some(config_path);
        }
        dir.pop()
    } {}
    None
}

#[derive(Debug, Deserialize)]
pub struct BuildConfig {
    pub src: String,
    pub output: String,
    pub dep: String,
}

impl BuildConfig {
    fn default() -> BuildConfig {
        BuildConfig {
            src: "./main.mm".to_string(),
            output: "./a.out".to_string(),
            dep: ".".to_string(),
        }
    }
}
