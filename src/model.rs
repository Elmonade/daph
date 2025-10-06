use crate::Audio;
use crate::Config;
use crate::State;
use crate::order::Order;
use crate::player::SinkModel;
use crate::update::Command;
use crate::utility::load_audio;
use crate::utility::order_by;
use ratatui::widgets::ListState;
use ratatui::widgets::ScrollbarState;
use ratatui::widgets::TableState;
use std::sync::mpsc::{self, Receiver, Sender};

pub(crate) struct PlayerModel<'a> {
    pub state: &'a State,

    pub tracks: Vec<Audio>,
    pub number_of_tracks: usize,
    pub current_track_index: Option<usize>,

    pub table_state: TableState,
    pub order_list_state: ListState,
    pub search_list_state: ListState,
    pub scrollbar_state: ScrollbarState,

    pub tx: Sender<Command>,
    pub sink_rx: Receiver<SinkModel>,

    pub keyword: String,
    pub matched_tracks: Vec<usize>,

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
        let (number_of_tracks, mut tracks) = load_audio(config.path);

        // Will not create a model without tracks.
        if number_of_tracks == 0 {
            println!("Can't find the audio files. ");
            println!(
                "You may:
        1. Update the configuation file with path to your audio file(s).
        2. Create 'Music' directory in your home directory."
            );

            std::process::exit(1);
        }

        order_by(&Order::Artist, &Order::Shuffle, &mut tracks);

        PlayerModel {
            tracks,
            number_of_tracks,
            state: &State::Playing,
            keyword: String::new(),
            current_track_index: None,
            table_state: TableState::default().with_selected(Some(0)),
            order_list_state: ListState::default().with_selected(Some(0)),
            search_list_state: ListState::default().with_selected(Some(0)),
            tx,
            sink_rx,
            matched_tracks: Vec::new(),
            iteration_count: 0,
            volume: 1.0,
            playback_order: Order::Artist,
            seek_distance: config.seek_distance,
            volume_step: config.volume_step,
            scrollbar_state: ScrollbarState::new(number_of_tracks).position(0),
        }
    }
}
