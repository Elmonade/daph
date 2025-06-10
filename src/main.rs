use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyEvent};
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

#[derive(Debug, Default)]
struct PlayerState {
    musics: Vec<Audio>,
    list_state: ListState,
    is_searching: bool,
    keyword: String,
    is_playing: bool,
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

    state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Hello from the other side")),
        author: (String::from("Adele")),
        length: 180,
    });
    state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Hail to the king")),
        author: (String::from("Adele")),
        length: 180,
    });
    state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Lemon Tree")),
        author: (String::from("Adele")),
        length: 180,
    });

    color_eyre::install()?;
    let terminal = ratatui::init();

    let result = run(terminal, &mut state);

    // Bring back the normal mode. Otherwise terminal is stuck in raw mode.
    ratatui::try_restore();
    result
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
                // Make sure everything is set to not playing
                //TODO: If you press again it should pause.
                //TODO: Horrible way of doing it.

                player_state.is_playing = !player_state.is_playing;

                player_state
                    .musics
                    .iter_mut()
                    .for_each(|audio| audio.is_playing = false);

                // Set the this audio as being played
                if let Some(index) = player_state.list_state.selected() {
                    player_state.musics[index].is_playing = true;

                    // We will never go out of bounds. But we could do it this way - bit
                    // more safe.
                    /*
                    let being_played = player_state.musics.get_mut(index);
                    match being_played {
                        Some(audio) => audio.is_playing = true,
                        None => println!("No music, out of bounds."),
                    }
                    */
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

    let [music_list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(left_top);

    let left_top_block = Block::bordered()
        .title("AUDIO")
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow);

    //Block::new().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT).render(left_top, frame.buffer_mut());

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
                    .add_modifier(Modifier::ITALIC),
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
}
