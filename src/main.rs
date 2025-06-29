use color_eyre::eyre::{Error, Result};
use crossterm::event::{self, Event, KeyEvent};
use lofty::tag::Accessor;
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Padding, Row, Table, TableState};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    widgets::{Paragraph, Widget},
};

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::read_from_path;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::sync::mpsc::{self, Sender};
use walkdir::WalkDir;
mod playback;

const PATH: &str = "/home/jello/Media/audio";

struct PlayerState {
    musics: Vec<Audio>,
    is_searching: bool,
    keyword: String,
    is_playing: bool,
    current_track_index: Option<usize>,
    table_state: TableState,
    tx: Sender<Command>,
    number_of_tracks: usize,
}
impl Default for PlayerState {
    fn default() -> Self {
        let (tx, _rx) = mpsc::channel();

        PlayerState {
            is_playing: false,
            current_track_index: None,
            table_state: TableState::default(),
            musics: Vec::new(),
            is_searching: false,
            keyword: String::new(),
            tx,
            number_of_tracks: 0,
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
enum Command {
    PlayPause(PathBuf, i32),
    Forward(PathBuf, i32),
    Backward(PathBuf, i32),
    Next(PathBuf, i32),
    Previous(PathBuf, i32),
    New(PathBuf, i32),
}

fn main() -> Result<()> {
    env_logger::init();
    let mut state = PlayerState::default();
    state.table_state.select_first();
    state.table_state.select_first_column();
    state.tx = playback::setup();

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

fn run(mut terminal: DefaultTerminal, player_state: &mut PlayerState) -> Result<()> {
    loop {
        //Rendring
        terminal.draw(|f| render(f, player_state))?;
        //Input
        if let Event::Key(key) = event::read()? {
            if player_state.is_searching {
                //TODO: Do we need player_state?
                match handle_search(key, player_state) {
                    Action::Submit => player_state.is_searching = false,
                    Action::Escape => player_state.is_searching = false,
                    Action::None => {}
                }
            } else {
                match handle_button(key, player_state) {
                    Action::Escape => break,
                    Action::Submit => {}
                    Action::None => {}
                }
            }
        }
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
            'p' => {
                // When paused, changing songs not working. Paused current_index is kept same.
                // TODO: Use of unwrap is discouraged. Handle the possible error.
                let selected_index = state.table_state.selected().unwrap();
                match state.current_track_index {
                    Some(current_index) => {
                        println!("{selected_index} == {current_index}");
                        if selected_index == current_index {
                            state.musics[selected_index].is_playing = !state.musics[selected_index].is_playing;
                            state.is_playing = !state.is_playing;
                            state
                                .tx
                                .send(Command::PlayPause(PathBuf::new(), 10))
                                .unwrap_or(());
                        } else {
                            state.musics[selected_index].is_playing = true;
                            state.musics[current_index].is_playing = false;
                            state.current_track_index = Some(selected_index);
                            state.is_playing = true;

                            let path = state.musics[selected_index].path.clone();
                            state.tx.send(Command::New(path, 10)).unwrap_or(());
                        }
                    }
                    None => {
                        println!("New song - current_index = None");
                        //TODO: Refactor
                        state.musics[selected_index].is_playing = true;
                        state.current_track_index = Some(selected_index);
                        state.is_playing = true;

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
            // TODO: Should it wrap around?
            '<' => {
                if let Some(mut index) = state.current_track_index {
                    if index > 0 {
                        index -= 1;
                    }
                    state.current_track_index = Some(index);

                    state.musics[index + 1].is_playing = false;
                    state.musics[index].is_playing = true;
                    state.is_playing = true;

                    let path = state.musics[index].path.clone();
                    match state.tx.send(Command::New(path, 10)) {
                        Ok(_) => println!("Sent the command"),
                        Err(err) => println!("{}", err),
                    }
                }
            }
            '>' => {
                if let Some(mut index) = state.current_track_index {
                    if index < state.number_of_tracks - 1 {
                        index += 1;
                    }
                    state.current_track_index = Some(index);

                    state.musics[index - 1].is_playing = false;
                    state.musics[index].is_playing = true;
                    state.is_playing = true;

                    let path = state.musics[index].path.clone();
                    match state.tx.send(Command::New(path, 10)) {
                        Ok(_) => println!("Sent the command"),
                        Err(err) => println!("{}", err),
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
    Action::None
}

fn create_table(tracks: &Vec<Audio>) -> Table {
    let header = Row::new(["Song", "Artist", "Duration"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows: Vec<Row> = tracks
        .iter()
        .map(|item| {
            let style = match item.is_playing {
                true => Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
                _ => Style::default(),
            };

            //TODO: Is cloning the only way? Investigate.
            Row::new([
                item.name.clone(),
                item.author.clone(),
                item.length.to_string(),
            ])
            .style(style)
        })
        .collect();

    let footer = Row::new(["Lemon", "Lemon Tree", "000"]);
    let widths = [
        Constraint::Percentage(50),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(footer.italic())
        .column_spacing(1)
        //.style(Color::White)
        //.row_highlight_style(Style::new().on_black().bold())
        .row_highlight_style(Style::new().fg(Color::Green))
        //.column_highlight_style(Color::Gray)
        //.cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("- ");
    table
}

fn render(frame: &mut Frame, player_state: &mut PlayerState) {
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
            .margin(0)
            .areas(frame.area());
    let [left_top, left_bottom] =
        Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
            .margin(0)
            .areas(left);

    if player_state.is_searching {
        //TODO: Dynamic Scaling OR Make it toggleable
        Paragraph::new(player_state.keyword.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::uniform(1))
                    .title("SEARCH"),
            )
            .render(right, frame.buffer_mut());
    }

    let [music_list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(left_top);
    let [player_area] = Layout::horizontal([Constraint::Fill(1)])
        .margin(1)
        .areas(left_bottom);

    let left_top_block = Block::bordered()
        .title("AUDIO")
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow);

    let left_bottom_block = Block::bordered()
        .title("PLAYER")
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow);

    frame.render_widget(left_top_block, left_top);
    frame.render_widget(left_bottom_block, left_bottom);
    let musics = &player_state.musics;
    let table = create_table(musics);
    frame.render_stateful_widget(table, music_list_area, &mut player_state.table_state);

    if player_state.is_playing {
        // TODO: Only a mother can love type shi code.
        // It's not much but it's a honest work.
        // This is about to get even worse. - 06.24.2025

        let mut index = 0;
        if let Some(current_index) = player_state.current_track_index {
            index = current_index;
        }

        // TODO: Use iterator to replace the clone
        let current_track_name = player_state.musics[index].name.clone();
        let current_track_artist = player_state.musics[index].author.clone();
        Paragraph::new(format!("{} - {}", current_track_name, current_track_artist))
            .render(player_area, frame.buffer_mut());
    } else {
        Paragraph::new("No music is currently playing").render(player_area, frame.buffer_mut());
    }
}
