use color_eyre::eyre::Result;
use color_eyre::eyre::Error;
use crossterm::event::{self, Event, KeyEvent};
use lofty::tag::Accessor;
use playback::SinkState;

use ratatui::DefaultTerminal;
use crate::view::render;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::read_from_path;
use ratatui::widgets::TableState;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::sync::mpsc::{self, Receiver, Sender};
use walkdir::WalkDir;
mod playback;
mod view;

const PATH: &str = "/home/jello/Media/audio";

struct PlayerState {
    musics: Vec<Audio>,
    is_searching: bool,
    keyword: String,
    current_track_index: Option<usize>,
    table_state: TableState,
    tx: Sender<Command>,
    sink_rx: Receiver<SinkState>,
    number_of_tracks: usize,
    que_len: usize,
    _sink_state: Option<SinkState>,
}
impl Default for PlayerState {
    fn default() -> Self {
        let (tx, _rx) = mpsc::channel::<Command>();
        let (_tx, sink_rx) = mpsc::channel::<SinkState>();

        PlayerState {
            _sink_state: None,
            current_track_index: None,
            table_state: TableState::default(),
            musics: Vec::new(),
            is_searching: false,
            keyword: String::new(),
            tx,
            sink_rx,
            number_of_tracks: 0,
            que_len: 0,
        }
    }
}

#[derive(Debug)]
struct Audio {
    is_playing: bool,
    name: String,
    author: String,
    length: u64,
    path: PathBuf,
}

enum Action {
    None,
    Submit,
    Escape,
}

#[derive(Debug)]
pub(crate) enum Command {
    PlayPause(PathBuf, i32),
    Forward(PathBuf, i32),
    Backward(PathBuf, i32),
    Next(PathBuf, i32),
    Previous(PathBuf, i32),
    New(PathBuf, i32),
    Append(PathBuf, i32),
}

fn main() -> Result<()> {
    env_logger::init();
    let mut state = PlayerState::default();
    state.table_state.select_first();
    state.table_state.select_first_column();
    let (command_tx, sink_rx) = playback::setup();
    state.tx = command_tx;
    state.sink_rx = sink_rx;

    let _ = load_audio(&mut state);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    let _ = ratatui::try_restore(); // Exit raw mode
    result
}

//TODO: Pushing everything to PlayerState struct is kinda iffy.
fn load_audio(player_state: &mut PlayerState) -> Result<bool, Error> {
    for entry in WalkDir::new(PATH) {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            if extension == "mp3" || extension == "flac" || extension == "wav" {
                player_state.number_of_tracks += 1;
                let path = entry.path();
                let tagged_file = match read_from_path(path) {
                    Ok(it) => it,
                    Err(_) => todo!(),
                };

                let tag = match tagged_file.primary_tag() {
                    Some(primary_tag) => primary_tag,
                    None => tagged_file.first_tag().expect("ERROR: No tags"),
                };

                let tag_title = tag.title();
                let title = String::from(tag_title.as_deref().unwrap_or("None"));
                let tag_artist = tag.artist();
                let artist = String::from(tag_artist.as_deref().unwrap_or("None"));
                let properties = tagged_file.properties();
                let seconds = properties.duration().as_secs();

                player_state.musics.push(Audio {
                    is_playing: (false),
                    name: (title),
                    author: (artist),
                    length: seconds,
                    path: path.to_path_buf(),
                });
            }
        }
    }
    Ok(true)
}

fn run(mut terminal: DefaultTerminal, state: &mut PlayerState) -> Result<()> {
    let mut is_playing = false;
    let mut current_track_finished = false;
    loop {
        if let Ok(sink) = state.sink_rx.try_recv() {
            state.que_len = sink.que_len;
            is_playing = sink.is_playing;
            current_track_finished = sink.current_track_finished;
        }

        // Render
        terminal.draw(|f| render(f, state, is_playing))?;

        // Input - Non-blocking poll. Raw Event will block this thread.
        // Wait up to 50 ms.
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if state.is_searching {
                    //TODO: Do we need state?
                    match handle_search(key, state) {
                        Action::Submit => state.is_searching = false,
                        Action::Escape => state.is_searching = false,
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
                state.tx.send(Command::New(path, 10)).unwrap_or(());
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(15));
    }
    Ok(())
}

fn handle_search(key: KeyEvent, player_state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Char(c) => {
            player_state.keyword.push(c);
        }
        event::KeyCode::Backspace => {
            player_state.keyword.pop();
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
        event::KeyCode::Esc => return Action::Escape,
        event::KeyCode::Char(char) => match char {
            ' ' => {
                state
                    .tx
                    .send(Command::PlayPause(PathBuf::new(), 10))
                    .unwrap_or(());
            }
            'p' => {
                // TODO: Use of unwrap is discouraged. Handle the possible error.
                let selected_index = state.table_state.selected().unwrap();
                match state.current_track_index {
                    Some(current_index) => {
                        if selected_index == current_index {
                            state.musics[selected_index].is_playing =
                                !state.musics[selected_index].is_playing;

                            state
                                .tx
                                .send(Command::PlayPause(PathBuf::new(), 10))
                                .unwrap_or(());
                        } else {
                            state.musics[selected_index].is_playing = true;
                            state.musics[current_index].is_playing = false;
                            state.current_track_index = Some(selected_index);

                            let path = state.musics[selected_index].path.clone();
                            state.tx.send(Command::New(path, 10)).unwrap_or(());
                        }
                    }
                    None => {
                        //TODO: Refactor - Number of duplication of following steps.
                        state.musics[selected_index].is_playing = true;
                        state.current_track_index = Some(selected_index);

                        let path = state.musics[selected_index].path.clone();
                        state.tx.send(Command::New(path, 10)).unwrap_or(());
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
                state.table_state.select_next();
            }
            'k' => {
                state.table_state.select_previous();
            }
            '<' => {
                if let Some(mut index) = state.current_track_index {
                    state.musics[index].is_playing = false;
                    index = (index + state.number_of_tracks - 1) % state.number_of_tracks;

                    state.current_track_index = Some(index);
                    state.musics[index].is_playing = true;

                    let path = state.musics[index].path.clone();
                    state.tx.send(Command::New(path, 10)).unwrap_or(());
                }
            }
            '>' => {
                if let Some(mut index) = state.current_track_index {
                    state.musics[index].is_playing = false;
                    index = (index + 1) % state.number_of_tracks;

                    state.musics[index].is_playing = true;
                    state.current_track_index = Some(index);
                    let path = state.musics[index].path.clone();
                    state.tx.send(Command::New(path, 10)).unwrap_or(());
                }
            }
            _ => {}
        },
        _ => {}
    }
    Action::None
}
