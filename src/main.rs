use std::path::PathBuf;
use std::process::exit;
use std::result::Result::Ok;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;
use std::usize;

use ratatui::DefaultTerminal;
use ratatui::widgets::TableState;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEvent};

use crate::fuzzy_search::search;
use crate::utility::load_audio;
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
    musics: Vec<Audio>,
    is_searching: bool,
    is_adjusting: bool,
    keyword: String,
    current_track_index: Option<usize>,
    table_state: TableState,
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
        match load_audio() {
            Ok((musics, number_of_tracks)) => PlayerState {
                _sink_state: None,
                current_track_index: None,
                table_state: TableState::default(),
                musics,
                matched_tracks: Vec::new(),
                is_searching: false,
                is_adjusting: false,
                keyword: String::new(),
                tx,
                sink_rx,
                number_of_tracks,
                iteration_count: 0,
                volume: 1.0,
            },
            Err(_) => {
                eprintln!("No audio file found. Please try different path.");
                exit(1);
            }
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

fn main() -> Result<()> {
    env_logger::init();
    let mut state = PlayerState::default();
    state.table_state.select_first();
    state.table_state.select_first_column();

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

        // TODO: Update render
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
                state.musics[index].is_playing = false;
                index = (index + 1) % state.number_of_tracks;

                state.musics[index].is_playing = true;
                state.current_track_index = Some(index);
                let path = state.musics[index].path.clone();
                state.tx.send(Command::New(path)).unwrap_or(());
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(15));

        state.iteration_count += 1;
        if state.iteration_count % 20 == 0 {
            state.is_adjusting = false;
            state.iteration_count = 0;
        }
    }
    Ok(())
}

fn handle_search(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Char(c) => {
            state.keyword.push(c);
            state.matched_tracks = search(&state.musics, &state.keyword);
        }
        event::KeyCode::Backspace => {
            state.keyword.pop();
            state.matched_tracks = search(&state.musics, &state.keyword);
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

// TODO: Code duplication. Various buttons have a quite similar logic.
fn handle_button(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Esc => return Action::Escape,
        event::KeyCode::Char(char) => match char {
            ' ' => {
                state
                    .tx
                    .send(Command::PlayPause(PathBuf::new()))
                    .unwrap_or(());
            }
            ':' => {
                if let Some(selected_index) = state.table_state.selected() {
                    let mut index = selected_index;
                    if selected_index > state.number_of_tracks {
                        index = state.number_of_tracks - 1;
                    }
                    match state.current_track_index {
                        Some(current_index) => {
                            if index == current_index {
                                state.musics[index].is_playing = !state.musics[index].is_playing;

                                state
                                    .tx
                                    .send(Command::PlayPause(PathBuf::new()))
                                    .unwrap_or(());
                            } else {
                                state.musics[index].is_playing = true;
                                state.musics[current_index].is_playing = false;
                                state.current_track_index = Some(index);

                                let path = state.musics[index].path.clone();
                                state.tx.send(Command::New(path)).unwrap_or(());
                            }
                        }
                        None => {
                            //TODO: Refactor - Number of duplication of following steps.
                            state.musics[index].is_playing = true;
                            state.current_track_index = Some(index);

                            let path = state.musics[index].path.clone();
                            state.tx.send(Command::New(path)).unwrap_or(());
                        }
                    }
                }
            }
            '/' => {
                state.is_searching = true;
            }
            'D' => {
                if let Some(index) = state.table_state.selected() {
                    state.musics.remove(index);
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
                    state.musics[index].is_playing = false;
                    index = (index + state.number_of_tracks - 1) % state.number_of_tracks;

                    state.current_track_index = Some(index);
                    state.musics[index].is_playing = true;

                    let path = state.musics[index].path.clone();
                    state.tx.send(Command::New(path)).unwrap_or(());
                }
            }
            'n' => {
                if let Some(mut index) = state.current_track_index {
                    state.musics[index].is_playing = false;
                    index = (index + 1) % state.number_of_tracks;

                    state.musics[index].is_playing = true;
                    state.current_track_index = Some(index);
                    let path = state.musics[index].path.clone();
                    state.tx.send(Command::New(path)).unwrap_or(());
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
                    let length = state.musics[index].length;
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
