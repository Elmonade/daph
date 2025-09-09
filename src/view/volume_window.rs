use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::view::{CUSTOM_LABEL_COLOR, number_drawer::NumberDrawer, view_utility};

pub fn draw(sink_volume: f32, left_top: Rect, frame: &mut Frame) {
    let volume = (sink_volume * 10.0) as u32;
    let mut string_volume = volume.to_string();
    if volume < 10 {
        string_volume = format!("0{volume}")
    }

    let enlarged_volume = NumberDrawer::draw(&string_volume);

    let centered_area =
        view_utility::center(left_top, Constraint::Length(11), Constraint::Percentage(18));

    let volume_paragraph = Paragraph::new(enlarged_volume)
        .style(Style::default().fg(CUSTOM_LABEL_COLOR))
        .block(Block::new().borders(Borders::NONE));

    frame.render_widget(Clear, left_top);
    frame.render_widget(volume_paragraph, centered_area);
}
