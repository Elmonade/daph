use crate::Audio;
use crate::Command;
use crate::Config;
use crate::order::Order;
use crate::playback::SinkState;
use crate::utility::load_audio;
use ratatui::widgets::ListState;
use ratatui::widgets::TableState;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};

const DEFAULT_SEEK_DISTANCE: usize = 5;

pub(crate) struct PlayerState {
    pub tracks: Vec<Audio>,
    pub is_searching: bool,
    pub is_adjusting: bool,
    pub is_configuring: bool,
    pub keyword: String,
    pub current_track_index: Option<usize>,
    pub table_state: TableState,
    pub list_state: ListState,
    pub tx: Sender<Command>,
    pub sink_rx: Receiver<SinkState>,
    pub number_of_tracks: usize,
    pub _sink_state: Option<SinkState>,
    pub matched_tracks: Vec<Audio>,
    pub iteration_count: usize,
    pub volume: f32,
    pub playback_order: Order,
    pub seek_distance: usize,
}

impl PlayerState {
    fn init(track_path: PathBuf, seek_distance: usize) -> Self {
        let (tx, _rx) = mpsc::channel::<Command>();
        let (_tx, sink_rx) = mpsc::channel::<SinkState>();
        let (number_of_tracks, tracks) = load_audio(track_path);
        PlayerState {
            tracks,
            number_of_tracks,
            is_searching: false,
            is_adjusting: false,
            is_configuring: false,
            keyword: String::new(),
            current_track_index: None,
            table_state: TableState::default(),
            list_state: ListState::default(),
            tx,
            sink_rx,
            _sink_state: None,
            matched_tracks: Vec::new(),
            iteration_count: 0,
            volume: 1.0,
            playback_order: Order::Artist,
            seek_distance,
        }
    }

    fn load_config(path: &PathBuf) -> Config {
        let file = fs::read(path)
            .expect("Could not read the config file.")
            .iter()
            .map(|c| *c as char)
            .collect::<String>();
        toml::from_str(&file).expect("Not properly formatted")
    }
}

// PlayerState has config file.
impl Configure for PlayerState {
    fn configured(path: PathBuf) -> PlayerState {
        let config = PlayerState::load_config(&path);
        PlayerState::init(config.path, config.seek_distance)
    }
}

pub(crate) trait Configure {
    fn configured(path: PathBuf) -> PlayerState;
}

// In case no config file is found use the default settings.
// TODO: Handle the case where you don't find any music in the default path.
impl Default for PlayerState {
    fn default() -> Self {
        match home::home_dir() {
            Some(path) => PlayerState::init(path.join("Music"), DEFAULT_SEEK_DISTANCE),
            None => PlayerState::init(PathBuf::from("/home"), DEFAULT_SEEK_DISTANCE)
        }
    }
}
