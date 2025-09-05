use crate::Audio;
use crate::PlayerModel;
use crate::SinkModel;
use crate::State;
use number_drawer::NumberDrawer;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Borders;
use ratatui::widgets::LineGauge;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Scrollbar;
use ratatui::widgets::ScrollbarOrientation;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, BorderType, Padding};
use ratatui::widgets::{Row, Table};
use std::time::Duration;
mod config_window;
mod number_drawer;
mod player_window;
mod search_window;
mod view_utility;
mod volume_window;

const CUSTOM_LABEL_COLOR: Color = tailwind::SKY.c200;
const BY_COLOR: Color = tailwind::RED.c300;
const GAUGE_COLOR: Color = tailwind::GREEN.c800;

pub(crate) fn render(frame: &mut Frame, model: &PlayerModel, sink: &SinkModel) {
    let [mut left, mut right] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(25)]).areas(frame.area());

    if frame.area().width < 120 {
        [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(0)])
            .areas(frame.area());
    }

    let [mut left_top, mut left_bottom] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(6)])
            .horizontal_margin(2)
            .areas(left);

    let [mut right_top, mut right_bottom] =
        Layout::vertical([Constraint::Fill(1), Constraint::Percentage(75)])
            .margin(2)
            .areas(right);

    if frame.area().height < 14 {
        [left_top, left_bottom] =
            Layout::vertical([Constraint::Percentage(0), Constraint::Fill(1)])
                .vertical_margin(2)
                .areas(left);
    }
    if frame.area().height < 30 {
        [right_top, right_bottom] =
            Layout::vertical([Constraint::Fill(1), Constraint::Percentage(0)])
                .flex(Flex::Center)
                .vertical_margin(1)
                .areas(right);
    }

    // It is impossible to set top-margin independently, thus manual padding.
    let [_padding, left_top_padded] =
        Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(left_top);

    let [music_list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(left_top_padded);

    let left_top_block = Block::bordered()
        .title("LIBRARY")
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow);

    let table = view_utility::create_table(&model.tracks);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .thumb_style(Style::default().fg(Color::Green))
        .begin_symbol(Some(" "))
        .thumb_symbol("|")
        .track_symbol(None)
        .end_symbol(None);
    let dolphin =
        Paragraph::new(NumberDrawer::draw("bird")).block(Block::default().padding(Padding {
            left: (20),
            right: (0),
            top: (20),
            bottom: (0),
        }));

    let mut index = 8; // Point at something on startup.
    if let Some(current_index) = model.current_track_index {
        index = current_index;
    }
    let mut table_model = model.table_state.clone();
    let mut scrollbar_state = model.scrollbar_state;
    frame.render_widget(left_top_block, left_top_padded);
    frame.render_widget(dolphin, right_bottom);
    frame.render_stateful_widget(table, music_list_area, &mut table_model);
    frame.render_stateful_widget(scrollbar, music_list_area, &mut scrollbar_state);

    if model.state == &State::Searching {
        search_window::draw(model, right, frame)
    }
    if model.state == &State::Adjusting {
        volume_window::draw(sink.volume, left_top_padded, frame)
    }
    config_window::draw(model, right_top, frame);
    player_window::draw(index, sink, model, left_bottom, frame);
}
