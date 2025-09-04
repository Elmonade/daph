use crate::view::BorderType;
use ratatui::prelude::Stylize;
use ratatui::prelude::Widget;
use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
    widgets::{Block, Clear, Padding, Paragraph},
};

use crate::model::PlayerModel;

pub fn draw(model: &PlayerModel, right: Rect, frame: &mut Frame) {
    frame.render_widget(Clear, right);
    Paragraph::new(model.keyword.as_str())
        .block(
            Block::bordered()
                .fg(Color::Green)
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(1))
                .title("SEARCH"),
        )
        .render(right, frame.buffer_mut());
}

