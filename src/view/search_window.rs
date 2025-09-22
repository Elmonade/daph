use crate::State;
use crate::view::view_utility;
use ratatui::layout::Constraint;
use ratatui::layout::Flex;
use ratatui::layout::Layout;
use ratatui::layout::Margin;
use ratatui::prelude::Stylize;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::Borders;
use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, Clear, Padding, Paragraph},
};

use crate::model::PlayerModel;

pub fn draw(model: &PlayerModel, right: Rect, frame: &mut Frame) {
    frame.render_widget(Clear, right);

    let [right_top, right_bottom] = Layout::vertical([Constraint::Length(4), Constraint::Fill(1)])
        .flex(Flex::Center)
        .vertical_margin(2)
        .horizontal_margin(2)
        .areas(right);

    let mut list_model = model.search_list_state.clone();

    let search = Block::default()
        .fg(Color::Green)
        .padding(Padding::uniform(1))
        .title("SEARCH")
        .borders(Borders::TOP);
    let result = Block::default()
        .fg(Color::Green)
        .padding(Padding::uniform(1))
        .borders(Borders::TOP | Borders::BOTTOM);

    let list_container = Block::default().borders(Borders::NONE);

    let centered_area_top = view_utility::center(
        right_top.inner(Margin::new(1, 1)),
        Constraint::Percentage(80),
        Constraint::Length(6),
    );

    let centered_area_bottom = view_utility::center(
        right_bottom.inner(Margin::new(0, 0)),
        Constraint::Percentage(100),
        Constraint::Percentage(90),
    );

    let highlight = if model.state == &State::Configuring {
        Style::new().reversed()
    } else {
        Style::new()
    };

    let rows: Vec<Span> = model.matched_tracks
        .iter()
        .map(|item| {
            let style = match item.is_playing {
                true => Style::default().add_modifier(Modifier::UNDERLINED),
                _ => Style::default(),
            };

            Span::from(&item.name).style(style)
        })
        .collect();

    let keyword = Paragraph::new(model.keyword.as_str());
    let list = view_utility::create_list(rows, highlight, " - ");
    frame.render_widget(search, right_top);
    frame.render_widget(result, right_bottom);
    frame.render_widget(keyword, centered_area_top);
    frame.render_stateful_widget(
        list.block(list_container),
        centered_area_bottom,
        &mut list_model,
    );
}
