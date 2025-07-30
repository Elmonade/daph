use std::path::PathBuf;
use std::result::Result::Ok;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use std::usize;

use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;
use ratatui::widgets::TableState;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEvent};

use crate::fuzzy_search::search;
use crate::utility::load_audio;
use crate::utility::play_new_track;
use crate::view::render;
use playback::SinkState;

mod fuzzy_search;
mod playback;
mod utility;
mod view;

const PATH: &str = "/home/jello/Media/audio";
const SEEK_DISTANCE: usize = 5;
const VOLUME_STEP: f32 = 0.1;

struct PlayerState {
    tracks: Vec<Audio>,
    is_searching: bool,
    is_adjusting: bool,
    is_configuring: bool,
    keyword: String,
    current_track_index: Option<usize>,
    table_state: TableState,
    list_state: ListState,
    tx: Sender<Command>,
    sink_rx: Receiver<SinkState>,
    number_of_tracks: usize,
    _sink_state: Option<SinkState>,
    matched_tracks: Vec<Audio>,
    iteration_count: usize,
    volume: f32,
}

impl Default for PlayerState {
    fn default() -> Self {
        let (tx, _rx) = mpsc::channel::<Command>();
        let (_tx, sink_rx) = mpsc::channel::<SinkState>();
        let (number_of_tracks, tracks) = load_audio();
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
        }
    }
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

enum Order {
    Shuffle,
    Album,
    Artist,
    Track,
}

impl Iterator for Order {
    type Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let mut state = PlayerState::default();
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

// TODO: Use list to show possible options
fn handle_config(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Tab => state.is_configuring = !state.is_configuring,
        event::KeyCode::Char(char) => match char {
            'j' => {
                if let Some(selected_index) = state.list_state.selected() {
                    if selected_index < 5 {
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
