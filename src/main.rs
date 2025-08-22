use crate::fuzzy_search::search;
use crate::order::Order;
use crate::state::Configure;
use crate::state::PlayerState;
use crate::utility::order_by;
use crate::utility::play_new_track;
use crate::view::render;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEvent};
use playback::SinkState;
use ratatui::DefaultTerminal;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::time::Duration;
mod fuzzy_search;
mod order;
mod playback;
mod state;
mod utility;
mod view;

// TODO: This shoud be inside state.rs
const VOLUME_STEP: f32 = 0.1;

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
            if event::poll(std::time::Duration::from_millis(16))?
                && let Event::Key(key) = event::read()?
            {
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

            // Auto-Queue
            if sink.current_track_finished
                && let Some(mut index) = state.current_track_index
            {
                state.tracks[index].is_playing = false;
                index = (index + 1) % state.number_of_tracks;
                play_new_track(index, state);
            }

            // If we assume two threads are perfectly in sync(probably impossible),
            // in total, one iteration should take 46ms when no button is pressed.
            // 2s / 49 = ~41
            state.iteration_count += 1;
            if state.iteration_count % 41 == 0 {
                state.is_adjusting = false;
                state.iteration_count = 0;
            }
        }
    }
    Ok(())
}

fn handle_config(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Tab => state.is_configuring = !state.is_configuring,
        event::KeyCode::Char(char) => match char {
            'j' => {
                if let Some(selected_index) = state.list_state.selected()
                    && selected_index < 3
                {
                    state.list_state.select_next();
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
                        match order_by(&Order::Shuffle, &state.playback_order, &mut state.tracks) {
                            Some(index) => {
                                state.current_track_index = Some(index);

                                state.playback_order = Order::Shuffle;
                            }

                            None => (),
                        }
                    }
                    1 => match order_by(&Order::Album, &state.playback_order, &mut state.tracks) {
                        Some(index) => {
                            state.current_track_index = Some(index);
                            state.playback_order = Order::Album;
                        }
                        None => (),
                    },
                    2 => match order_by(&Order::Artist, &state.playback_order, &mut state.tracks) {
                        Some(index) => {
                            state.current_track_index = Some(index);
                            state.playback_order = Order::Artist;
                        }
                        None => (),
                    },

                    3 => match order_by(&Order::Track, &state.playback_order, &mut state.tracks) {
                        Some(index) => {
                            state.current_track_index = Some(index);
                            state.playback_order = Order::Track;
                        }
                        None => (),
                    },
                    _ => {
                        match order_by(&Order::Shuffle, &state.playback_order, &mut state.tracks) {
                            Some(index) => {
                                state.current_track_index = Some(index);

                                state.playback_order = Order::Shuffle;
                            }

                            None => (),
                        }
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
                if let Some(selected_index) = state.table_state.selected()
                    && selected_index < state.number_of_tracks - 1
                {
                    state.table_state.select_next();
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
                    .send(Command::Backward(state.seek_distance))
                    .unwrap_or(());
            }
            '>' => {
                if let Some(index) = state.current_track_index {
                    let length = state.tracks[index].length;
                    state
                        .tx
                        .send(Command::Forward(state.seek_distance, length as usize))
                        .unwrap_or(());
                }
            }
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
