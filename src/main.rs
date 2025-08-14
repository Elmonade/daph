use crate::button_handler::handle_config;
use crate::button_handler::handle_playback;
use crate::button_handler::handle_search;
use crate::state::Configure;
use crate::state::PlayerState;
use crate::utility::play_new_track;
use crate::view::render;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use playback::SinkState;
use ratatui::DefaultTerminal;
use serde::Deserialize;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::time::Duration;
mod button_handler;
mod fuzzy_search;
mod order;
mod playback;
mod state;
mod utility;
mod view;

#[derive(Deserialize)]
struct Config {
    path: PathBuf,
    seek_distance: usize,
}

#[derive(Debug, Clone)]
struct Audio {
    is_playing: bool,
    name: String,
    author: String,
    length: u64,
    path: PathBuf,
}

#[derive(Debug)]
pub(crate) enum Command {
    PlayPause(PathBuf),
    New(PathBuf),
    Forward(usize, usize),
    Backward(usize),
    Volume(f32),
    _Next(PathBuf, i32),
    _Previous(PathBuf, i32),
    _Append(PathBuf, i32),
}

enum Action {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    env_logger::init();

    let mut state = if let Some(path) = home::home_dir() {
        let config_path = path.join(".config").join("daph.toml");
        if config_path.exists() {
            PlayerState::configured(config_path)
        } else {
            PlayerState::default()
        }
    } else {
        PlayerState::default()
    };

    state.table_state.select_first();
    state.table_state.select_first_column();
    state.list_state.select_first();

    let (command_tx, sink_rx) = playback::setup();
    state.tx = command_tx;
    state.sink_rx = sink_rx;

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    let _ = ratatui::try_restore();
    result
}

fn run(mut terminal: DefaultTerminal, state: &mut PlayerState) -> Result<()> {
    loop {
        if let Ok(sink) = state.sink_rx.recv_timeout(Duration::from_millis(33)) {
            // Render
            terminal.draw(|f| render(f, state, &sink))?;

            // Input
            if event::poll(std::time::Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if state.is_searching {
                        match handle_search(key, state) {
                            Action::Escape => state.is_searching = false,
                            Action::Submit => {}
                            Action::None => {}
                        }
                    } else if state.is_configuring {
                        match handle_config(key, state) {
                            Action::Escape => state.is_configuring = false,
                            Action::Submit => {}
                            Action::None => {}
                        }
                    } else {
                        match handle_playback(key, state) {
                            Action::Escape => break,
                            Action::Submit => {}
                            Action::None => {}
                        }
                    }
                }
            }

            // Auto-Queue
            if sink.current_track_finished {
                if let Some(mut index) = state.current_track_index {
                    state.tracks[index].is_playing = false;
                    index = (index + 1) % state.number_of_tracks;
                    play_new_track(index, state);
                }
            }

            // If we assume two threads are perfectly in sync(probably impossible),
            // in total, one iteration should take 49ms when no button is pressed.
            // 2s / 49ms = ~41
            state.iteration_count += 1;
            if state.iteration_count % 41 == 0 {
                state.is_adjusting = false;
                state.iteration_count = 0;
            }
        }
    }
    Ok(())
}
