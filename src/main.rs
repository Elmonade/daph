use crate::config::Config;
use crate::message::map_to_message;
use crate::model::PlayerModel;
use crate::update::update;
use crate::utility::play_new_track;
use crate::view::render;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use player::SinkModel;
use ratatui::DefaultTerminal;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::time::Duration;
mod config;
mod fuzzy_search;
mod message;
mod model;
mod order;
mod player;
mod update;
mod utility;
mod view;

// Total set of commands which player can respond to.
#[derive(Debug)]
enum Message<'a> {
    None,
    Submit,
    Escape,

    AppendKeyword(char),
    RemoveKeyword,

    Delete,
    SeekBack,
    SeekForward,
    PlayPause,
    AppendTrack,

    Up,
    Down,
    Next,
    Previous,
    SwapTo(Option<&'a State>),
}

#[derive(PartialEq, Debug)]
enum State {
    Searching,
    Configuring,
    Adjusting,
    Playing,
}

#[derive(Debug, Clone)]
struct Audio {
    is_playing: bool,
    name: String,
    author: String,
    length: u64,
    path: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();

    // TODO: Handle this unwrap.
    let config_path = home::home_dir().unwrap().join(".config/daph.toml");

    let mut model = PlayerModel::create(Config::new(&config_path));
    let (command_tx, sink_rx) = SinkModel::create();
    model.tx = command_tx;
    model.sink_rx = sink_rx;

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut model);
    let _ = ratatui::try_restore();
    result
}

fn run(mut terminal: DefaultTerminal, model: &mut PlayerModel) -> Result<()> {
    let mut previous_state = model.state;
    loop {
        if let Ok(sink) = model.sink_rx.recv_timeout(Duration::from_millis(33)) {
            // Render
            terminal.draw(|f| render(f, model, &sink))?;

            // Input
            if event::poll(std::time::Duration::from_millis(16))?
                && let Event::Key(key) = event::read()?
            {
                if model.state != &State::Adjusting {
                    previous_state = model.state;
                }
                let message = map_to_message(key, model);
                update(&message, model);
            }

            // Auto-Queue
            if sink.current_track_finished
                && let Some(mut index) = model.current_track_index
            {
                model.tracks[index].is_playing = false;
                index = (index + 1) % model.number_of_tracks;
                play_new_track(index, model);
            }

            /*
            Assume two threads are perfectly in sync(probably impossible).
            In total, one iteration should take 49ms when no button is pressed.
            2s / 49ms = ~41
            */
            if model.state == &State::Adjusting {
                model.iteration_count += 1;
                if model.iteration_count % 41 == 0 {
                    model.state = previous_state;
                    model.iteration_count = 0;
                }
            }
        }
    }
}
