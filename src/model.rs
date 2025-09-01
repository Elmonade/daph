use crate::Audio;
use crate::Command;
use crate::Config;
use crate::State;
use crate::order::Order;
use crate::player::SinkModel;
use crate::utility::load_audio;
use ratatui::widgets::ListState;
use ratatui::widgets::TableState;
use std::sync::mpsc::{self, Receiver, Sender};

pub(crate) struct PlayerModel<'a> {
    pub state: &'a State,

    pub tracks: Vec<Audio>,
    pub number_of_tracks: usize,
    pub current_track_index: Option<usize>,

    pub table_state: TableState,
    pub list_state: ListState,

    pub tx: Sender<Command>,
    pub sink_rx: Receiver<SinkModel>,

    pub keyword: String,
    pub matched_tracks: Vec<Audio>,

    pub iteration_count: usize,
    pub volume: f32,
    pub playback_order: Order,
    pub seek_distance: usize,
    pub volume_step: f32,
}

impl PlayerModel<'_> {
    pub(crate) fn create(config: Config) -> Self {
        let (tx, _rx) = mpsc::channel::<Command>();
        let (_tx, sink_rx) = mpsc::channel::<SinkModel>();
        let (number_of_tracks, tracks) = load_audio(config.path);

        // Will not create a model with no tracks.
        if number_of_tracks == 0 {
            println!("Can't find the audio files. ");
            println!(
                "You may:
        1. Update the configuation file with path to your audio file.
        2. Create Music directory in your home directory."
            );

            std::process::exit(1);
        }

        PlayerModel {
            tracks,
            number_of_tracks,
            state: &State::Playing,
            keyword: String::new(),
            current_track_index: None,
            table_state: TableState::default().with_selected(Some(0)),
            list_state: ListState::default().with_selected(Some(0)),
            tx,
            sink_rx,
            matched_tracks: Vec::new(),
            iteration_count: 0,
            volume: 1.0,
            playback_order: Order::Artist,
            seek_distance: config.seek_distance,
            volume_step: config.volume_step,
        }
    }
}
