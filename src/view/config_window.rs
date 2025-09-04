use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Padding},
};

use crate::{State, model::PlayerModel, order::Order, view::view_utility};

pub fn draw(model: &PlayerModel, right_top: Rect, frame: &mut Frame) {
    let mut list_model = model.list_state.clone();
    let settings = Block::default()
        .fg(Color::Green)
        .padding(Padding::uniform(4))
        .title("PLAYBACK ORDER")
        .borders(Borders::TOP | Borders::BOTTOM);
    let highlight = if model.state == &State::Configuring {
        Style::new().reversed()
    } else {
        Style::new()
    };

    let options = [
        Order::Shuffle.to_string(),
        Order::Album.to_string(),
        Order::Artist.to_string(),
        Order::Track.to_string(),
    ];

    // TODO: This should be inside view_utility.
    let rows: Vec<Span> = options
        .iter()
        .map(|item| {
            let style = match *item == model.playback_order.to_string() {
                true => Style::default().add_modifier(Modifier::UNDERLINED),
                _ => Style::default(),
            };

            Span::from(item).style(style)
        })
        .collect();

    let list = view_utility::create_list(rows, highlight);
    frame.render_stateful_widget(list.block(settings), right_top, &mut list_model);
}
