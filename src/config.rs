use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const SEEK_DISTANCE: usize = 10;
const VOLUME_STEP: f32 = 0.1;

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(default = "Config::default_path")]
    pub path: PathBuf,
    #[serde(default = "Config::default_seek_distance")]
    pub seek_distance: usize,
    #[serde(default = "Config::default_volume_step")]
    pub volume_step: f32,
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
                Config {
                    path: Self::default_path(),
                    seek_distance: Self::default_seek_distance(),
                    volume_step: Self::default_volume_step(),
                }
            }
        }
    }

    fn default_path() -> PathBuf {
        match home::home_dir() {
            Some(path) => path.join("Music"),
            None => PathBuf::from("/home"), // Grasping for anything out there.
        }
    }
    fn default_seek_distance() -> usize {
        SEEK_DISTANCE
    }
    fn default_volume_step() -> f32 {
       VOLUME_STEP 
    }
}

