use number_drawer::NumberDrawer;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    widgets::{Block, Paragraph},
};

use crate::{
    model::PlayerModel,
    view::{number_drawer, view_utility},
};

pub fn draw(_model: &PlayerModel, right_bottom: Rect, frame: &mut Frame) {
    let dolphin = Paragraph::new(NumberDrawer::draw("bird")).block(Block::default());

    let centered_area = view_utility::center(
        right_bottom,
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    );

    frame.render_widget(dolphin, centered_area);
}
