use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyEvent};
use lofty::tag::Accessor;
use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{
    Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding,
};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    symbols,
    widgets::{Paragraph, Widget},
};

use lofty::file::{TaggedFile, TaggedFileExt};
use lofty::read_from_path;

#[derive(Debug, Default)]
struct PlayerState {
    musics: Vec<Audio>,
    list_state: ListState,
    is_searching: bool,
    keyword: String,
    is_playing: bool,
    current_track_index: usize,
    current_track: Audio,
}

#[derive(Debug, Default)]
struct Audio {
    is_playing: bool,
    name: String,
    author: String,
    length: u16,
}

enum Action {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    let mut state = PlayerState::default();
    state.is_playing = false;
    state.current_track_index = 0;

    load_audio(&mut state);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::try_restore(); // Exit raw mode
    result
}

fn load_audio(player_state: &mut PlayerState) -> Result<TaggedFile>{
    //TODO: Initialize audio files from the given path.

    let path = "~/Media/audio/Enji - Ulaan/Enji - Ulaan - 02 Taivshral.mp3";
    let tagged_file = read_from_path(path)?;

    // Get the primary tag (ID3v2 in this case)
    let id3v2 = tagged_file.primary_tag();

    // If the primary tag doesn't exist, or the tag types
    // don't matter, the first tag can be retrieved
    let unknown_first_tag = tagged_file.first_tag();
    match id3v2 {
        Some(tag) => {
            if let Some(title) = tag.title() {
                println!("{}", title);
            }
        }
        None => println!("No tag found."),
    }

    //println!("{}", unknown_first_tag.unwrap());
    //println!("{}", id3v2.title);

    player_state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Hello from the other side")),
        author: (String::from("Adele")),
        length: 180,
    });
    player_state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Hail to the king")),
        author: (String::from("Adele")),
        length: 180,
    });
    player_state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Lemon Tree")),
        author: (String::from("Adele")),
        length: 180,
    });
    Ok(tagged_file)
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

fn handle_button(key: KeyEvent, player_state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Esc => return Action::Escape,
        event::KeyCode::Char(char) => match char {
            'p' => {
                if let Some(index) = player_state.list_state.selected() {
                    if index == player_state.current_track_index {
                        player_state.musics[index].is_playing =
                            !player_state.musics[index].is_playing;
                        player_state.is_playing = !player_state.is_playing;
                    } else {
                        player_state.musics[index].is_playing = true;
                        player_state.musics[player_state.current_track_index].is_playing = false;
                        player_state.current_track_index = index;
                        player_state.is_playing = true;
                    }
                }
            }
            '/' => {
                player_state.is_searching = true;
            }
            'D' => {
                if let Some(index) = player_state.list_state.selected() {
                    player_state.musics.remove(index);
                }
            }
            'j' => {
                player_state.list_state.select_next();
            }
            'k' => {
                player_state.list_state.select_previous();
            }
            _ => {}
        },
        _ => {}
    }
    Action::None
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

    //TODO: Implement Table inside music_list_area
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

    let items: Vec<ListItem> = player_state
        .musics
        .iter()
        .map(|item| {
            let style = match item.is_playing {
                true => Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
                _ => Style::default(),
            };

            ListItem::new(item.name.as_str()).style(style)
        })
        .collect();

    let list = List::new(items)
        .highlight_symbol("-")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_widget(left_top_block, left_top);
    frame.render_widget(left_bottom_block, left_bottom);
    frame.render_stateful_widget(list, music_list_area, &mut player_state.list_state);

    if player_state.is_playing {
        Paragraph::new("Current music begin played.").render(player_area, frame.buffer_mut());
    } else {
        Paragraph::new("No music is currently playing").render(player_area, frame.buffer_mut());
    }
}
