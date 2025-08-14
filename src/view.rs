use std::time::Duration;

use crate::Audio;
use crate::order::Order;
use crate::PlayerState;
use crate::SinkState;
use number_drawer::NumberDrawer;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Flex;
use ratatui::layout::Rect;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::LineGauge;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;
use ratatui::widgets::{Block, BorderType, Padding};
use ratatui::widgets::{Row, Table};

mod number_drawer;
mod view_utility;

const CUSTOM_LABEL_COLOR: Color = tailwind::SKY.c200;
const BY_COLOR: Color = tailwind::RED.c300;
const GAUGE_COLOR: Color = tailwind::GREEN.c800;

pub(crate) fn render(frame: &mut Frame, state: &PlayerState, sink: &SinkState) {
    let [mut left, mut right] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(25)])
            .margin(0)
            .areas(frame.area());

    if frame.area().width < 120 {
        [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(0)])
            .margin(0)
            .areas(frame.area());
    }

    let settings = Block::default()
        .fg(Color::Green)
        .padding(Padding::uniform(4))
        .title("PLAYBACK ORDER")
        .borders(Borders::TOP | Borders::BOTTOM);

    let [left_top, left_bottom] =
        Layout::vertical([Constraint::Fill(1), Constraint::Percentage(12)])
            .margin(2)
            .areas(left);

    let [right_top, right_bottom] =
        Layout::vertical([Constraint::Fill(1), Constraint::Percentage(75)])
            .margin(2)
            .areas(right);

    let [music_list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(left_top);
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

    let left_top_block = Block::bordered()
        .title("LIBRARY")
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow);

    let left_bottom_block = Block::default().fg(Color::Yellow);
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

    let table = view_utility::create_table(&state.tracks);
    let dolphin =
        Paragraph::new(NumberDrawer::draw("bird")).block(Block::default().padding(Padding {
            left: (20),
            right: (0),
            top: (20),
            bottom: (0),
        }));

    let mut table_state = state.table_state.clone();
    let mut list_state = state.list_state.clone();
    frame.render_widget(left_top_block, left_top);
    frame.render_widget(left_bottom_block, left_bottom);
    frame.render_widget(dolphin, right_bottom);
    frame.render_stateful_widget(table, music_list_area, &mut table_state);

    // Config Section
    let highlight = if state.is_configuring {
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
            let style = match *item == state.playback_order.to_string() {
                true => Style::default().add_modifier(Modifier::UNDERLINED),
                _ => Style::default(),
            };

            Span::from(item).style(style)
        })
        .collect();

    let list = view_utility::create_list(rows, highlight);
    frame.render_stateful_widget(list.block(settings), right_top, &mut list_state);

    // Search Section
    if state.is_searching {
        frame.render_widget(Clear, right);
        Paragraph::new(state.keyword.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::uniform(1))
                    .title("SEARCH"),
            )
            .render(right, frame.buffer_mut());
    }

    // Volume Section
    if state.is_adjusting {
        let volume = (sink.volume * 10.0) as u32;
        let mut string_volume = volume.to_string();
        if volume < 10 {
            string_volume = format!("0{volume}")
        }

        let enlarged_volume = NumberDrawer::draw(&string_volume);

        let centered_area = view_utility::center(
            left_top,
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        );

        let mut spacer = 0;
        if centered_area.width > 10 && centered_area.height > 10 {
            spacer = 5;
        }

        let volume_paragraph = Paragraph::new(enlarged_volume)
            .style(Style::default().fg(CUSTOM_LABEL_COLOR))
            .block(Block::new().borders(Borders::NONE).padding(Padding::new(
                centered_area.width / 2 - spacer,
                0,
                centered_area.height / 2 - spacer,
                0,
            )));

        frame.render_widget(Clear, left_top);
        frame.render_widget(volume_paragraph, centered_area);
    }

    // Player Section
    let player_color = match sink.is_playing {
        true => CUSTOM_LABEL_COLOR,
        false => Color::Gray,
    };

    let mut index = 8; // Point at something on startup.

    if let Some(current_index) = state.current_track_index {
        index = current_index;
    }

    if let Some(music) = state.tracks.get(index) {
        let progress_bar_style = Style::new().italic().bold().fg(player_color);
        let elapsed_label = Span::styled(format!("{}", sink.position.as_secs()), progress_bar_style);
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
