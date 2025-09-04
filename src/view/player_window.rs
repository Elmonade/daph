use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Padding};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::Span,
    widgets::Paragraph,
};

use crate::{
    model::PlayerModel,
    player::SinkModel,
    view::{CUSTOM_LABEL_COLOR, view_utility},
};

pub fn draw(sink: &SinkModel, model: &PlayerModel, left_bottom: Rect, frame: &mut Frame) {
    let [player_area_left, player_area, player_area_right] = Layout::horizontal([
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .flex(ratatui::layout::Flex::Center)
    .margin(0)
    .areas(left_bottom);

    let [progress_bar] = Layout::horizontal([Constraint::Fill(1)])
        .margin(1)
        .flex(ratatui::layout::Flex::Center)
        .areas(player_area);

    let elapsed_time_block = Block::default().borders(Borders::NONE).padding(Padding {
        left: (0),
        right: (0),
        top: (2),
        bottom: (0),
    });

    let total_time_block = Block::default().borders(Borders::NONE).padding(Padding {
        left: (0),
        right: (0),
        top: (2),
        bottom: (0),
    });

    let player_color = match sink.is_playing {
        true => CUSTOM_LABEL_COLOR,
        false => Color::Gray,
    };

    let mut index = 8; // Point at something on startup.

    if let Some(current_index) = model.current_track_index {
        index = current_index;
    }

    if let Some(music) = model.tracks.get(index) {
        let progress_bar_style = Style::new().italic().bold().fg(player_color);
        let elapsed_label =
            Span::styled(format!("{}", sink.position.as_secs()), progress_bar_style);
        let total_label = Span::styled(format!(" {}", music.length), progress_bar_style);
        let total_time = Paragraph::new(total_label).block(total_time_block);
        let elapsed_time = Paragraph::new(elapsed_label)
            .block(elapsed_time_block)
            .right_aligned();

        frame.render_widget(elapsed_time, player_area_left);
        frame.render_widget(total_time, player_area_right);
        let title = view_utility::title_block(&player_color, &music.author, &music.name);

        view_utility::render_progress(
            &sink.position,
            progress_bar,
            frame.buffer_mut(),
            title,
            music.length as f64,
        );
    }
}
