use crate::fuzzy_search::search;
use crate::state::PlayerState;
use crate::state::Configure;
use crate::utility::order_by;
use crate::utility::play_new_track;
use crate::view::render;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEvent};
use ratatui::DefaultTerminal;
use serde::Deserialize;
use std::env::home_dir;
use std::fmt::Display;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::time::Duration;
mod fuzzy_search;
mod playback;
mod state;
mod utility;
mod view;

// TODO: Read the following variables from the config file.
// Could include the previous state of the player too.
// e.g. playback order, volume, colorscheme...
const SEEK_DISTANCE: usize = 5;
const VOLUME_STEP: f32 = 0.1;

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

// TODO: Anything involving Order is just horrible code. Refactor.
enum Order {
    Shuffle,
    Album,
    Artist,
    Track,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Shuffle => write!(f, "Shuffle"),
            Order::Album => write!(f, "Album"),
            Order::Artist => write!(f, "Artist"),
            Order::Track => write!(f, "Track"),
        }
    }
}

impl Iterator for Order {
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Order::Shuffle => Some(Order::Album),
            Order::Album => Some(Order::Artist),
            Order::Artist => Some(Order::Track),
            Order::Track => Some(Order::Shuffle),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let config_path = home_dir().unwrap().join(".config").join("daph.toml");

    let mut state = if config_path.exists() {
        PlayerState::configured(config_path)
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
    let mut is_playing = false;
    let mut current_track_finished = false;
    let mut position = Duration::new(0, 0);
    loop {
        if let Ok(sink) = state.sink_rx.try_recv() {
            is_playing = sink.is_playing;
            current_track_finished = sink.current_track_finished;
            position = sink.position;
            state.volume = sink.volume;
        }

        // TODO: Update render. The state.volume is redundant.
        // Render
        terminal.draw(|f| render(f, state, is_playing, position, state.volume))?;

        // Input - Non-blocking poll. Raw Event will block this thread.
        // Wait up to 50 ms.
        if event::poll(std::time::Duration::from_millis(50))? {
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
                    match handle_button(key, state) {
                        Action::Escape => break,
                        Action::Submit => {}
                        Action::None => {}
                    }
                }
            }
        }

        // Auto-Queue
        if current_track_finished {
            if let Some(mut index) = state.current_track_index {
                state.tracks[index].is_playing = false;
                index = (index + 1) % state.number_of_tracks;
                play_new_track(index, state);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(15));

        // Clear volume control window after 20*(15..65)msec
        state.iteration_count += 1;
        if state.iteration_count % 20 == 0 {
            state.is_adjusting = false;
            state.iteration_count = 0; // Could be used with other windows with different interval.
        }
    }
    Ok(())
}

fn handle_config(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Tab => state.is_configuring = !state.is_configuring,
        event::KeyCode::Char(char) => match char {
            'j' => {
                if let Some(selected_index) = state.list_state.selected() {
                    if selected_index < 3 {
                        state.list_state.select_next();
                    }
                }
            }
            'k' => {
                state.list_state.select_previous();
            }
            _ => {}
        },
        event::KeyCode::Esc => {
            return Action::Escape;
        }
        event::KeyCode::Enter => {
            if let Some(index) = state.list_state.selected() {
                match index {
                    0 => {
                        order_by(&Order::Shuffle, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Shuffle;
                    }
                    1 => {
                        order_by(&Order::Album, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Album;
                    }
                    2 => {
                        order_by(&Order::Artist, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Artist;
                    }

                    3 => {
                        order_by(&Order::Track, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Track;
                    }
                    _ => {
                        order_by(&Order::Shuffle, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Shuffle;
                    }
                }
            }
            return Action::Submit;
        }
        _ => {}
    };
    Action::None
}

fn handle_search(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Char(c) => {
            state.keyword.push(c);
            state.matched_tracks = search(&state.tracks, &state.keyword);
        }
        event::KeyCode::Backspace => {
            state.keyword.pop();
            state.matched_tracks = search(&state.tracks, &state.keyword);
        }
        event::KeyCode::Esc => {
            return Action::Escape;
        }
        event::KeyCode::Enter => {
            return Action::Submit;
        }
        _ => {}
    };
    Action::None
}

fn handle_button(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Tab => state.is_configuring = !state.is_configuring,
        event::KeyCode::Esc => return Action::Escape,
        event::KeyCode::Char(char) => match char {
            ' ' => {
                state
                    .tx
                    .send(Command::PlayPause(PathBuf::new()))
                    .unwrap_or(());
            }
            ':' => {
                if let Some(index) = state.table_state.selected() {
                    match state.current_track_index {
                        Some(current_index) => {
                            if index == current_index {
                                state
                                    .tx
                                    .send(Command::PlayPause(PathBuf::new()))
                                    .unwrap_or(());
                            } else {
                                state.tracks[current_index].is_playing = false;
                                play_new_track(index, state);
                            }
                        }
                        None => {
                            play_new_track(index, state);
                        }
                    }
                }
            }
            '/' => {
                state.is_searching = true;
            }
            'D' => {
                if let Some(index) = state.table_state.selected() {
                    state.tracks.remove(index);
                }
            }
            'j' => {
                if let Some(selected_index) = state.table_state.selected() {
                    if selected_index < state.number_of_tracks - 1 {
                        state.table_state.select_next();
                    }
                }
            }
            'k' => {
                state.table_state.select_previous();
            }
            'p' => {
                if let Some(mut index) = state.current_track_index {
                    state.tracks[index].is_playing = false;
                    index = (index + state.number_of_tracks - 1) % state.number_of_tracks;
                    play_new_track(index, state);
                }
            }
            'n' => {
                if let Some(mut index) = state.current_track_index {
                    state.tracks[index].is_playing = false;
                    index = (index + 1) % state.number_of_tracks;
                    play_new_track(index, state);
                }
            }
            '<' => {
                state
                    .tx
                    .send(Command::Backward(SEEK_DISTANCE))
                    .unwrap_or(());
            }
            '>' => match state.current_track_index {
                Some(index) => {
                    let length = state.tracks[index].length;
                    state
                        .tx
                        .send(Command::Forward(SEEK_DISTANCE, length as usize))
                        .unwrap_or(());
                }
                _ => (),
            },
            'K' => {
                state.is_adjusting = true;
                state.iteration_count = 0;
                if state.volume < 2.0 {
                    state.tx.send(Command::Volume(VOLUME_STEP)).unwrap_or(());
                }
            }
            'J' => {
                state.is_adjusting = true;
                state.iteration_count = 0;
                if state.volume > 0.0 {
                    state.tx.send(Command::Volume(-VOLUME_STEP)).unwrap_or(());
                }
            }
            _ => {}
        },
        _ => {}
    }
    Action::None
}
