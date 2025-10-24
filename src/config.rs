use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const SEEK_DISTANCE: usize = 10;
const VOLUME_STEP: f32 = 0.1;

#[derive(Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    pub path: PathBuf,
    pub seek_distance: usize,
    pub volume_step: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: {
                match home::home_dir() {
                    Some(path) => path.join("Music"),
                    None => PathBuf::from("/home"), // Grasping for anything out there.
                }
            },
            seek_distance: SEEK_DISTANCE,
            volume_step: VOLUME_STEP,
        }
    }
}

impl Config {
    pub(crate) fn new(path: &PathBuf) -> Config {
        match fs::read_to_string(path) {
            Ok(path) => match toml::from_str(&path) {
                Ok(config) => config,
                Err(err) => {
                    println!("{}", err);
                    std::process::exit(1);
                }
            },
            Err(_) => {
                eprintln!("Unable to read the configuration file. Using default values.");
                Config::default()
            }
        }
    }
}
