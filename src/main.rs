use std::default;

use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};
use ratatui::Frame;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default)]
struct PlayerState {
    musics: Vec<Audio>,
    list_state: ListState,
}

#[derive(Debug, Default)]
struct Audio {
    is_playing: bool,
    name: String,
    author: String,
}

fn main() -> Result<()> {
    let mut state = PlayerState::default();

    state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Hello from the other side")),
        author: (String::from("Adele")),
    });
    state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Hail to the king")),
        author: (String::from("Adele")),
    });
    state.musics.push(Audio {
        is_playing: (false),
        name: (String::from("Lemon Tree")),
        author: (String::from("Adele")),
    });

    color_eyre::install()?;
    let terminal = ratatui::init();

    let result = run(terminal, &mut state);

    ratatui::try_restore();
    result
}

fn run(mut terminal: DefaultTerminal, player_state: &mut PlayerState) -> Result<()> {
    loop {
        //Rendring
        terminal.draw(|f| render(f, player_state))?;
        //Input
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                event::KeyCode::Char(char) => match char {
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
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, player_state: &mut PlayerState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());
    let list = List::new(
        player_state
            .musics
            .iter()
            .map(|x| ListItem::from(x.name.clone())),
    )
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));

    frame.render_stateful_widget(list, inner_area, &mut player_state.list_state);

    Paragraph::new("Lemons are used to make lemonade.").render(frame.area(), frame.buffer_mut());
}
