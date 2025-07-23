use std::time::Duration;

use crate::Audio;
use crate::PlayerState;
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Flex;
use ratatui::layout::Rect;
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::LineGauge;
use ratatui::widgets::{Block, BorderType, Padding};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Paragraph, Row, Table, Widget},
};

const CUSTOM_LABEL_COLOR: Color = tailwind::SKY.c200;
const BY_COLOR: Color = tailwind::RED.c300;
const GAUGE_COLOR: Color = tailwind::GREEN.c800;

pub(crate) fn render(
    frame: &mut Frame,
    state: &PlayerState,
    is_playing: bool,
    position: Duration,
    volume: f32,
) {
    let [mut left, mut right] =
        Layout::horizontal([Constraint::Percentage(100), Constraint::Percentage(0)])
            .margin(0)
            .areas(frame.area());

    if state.is_searching {
        [left, right] =
            Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
                .margin(0)
                .areas(frame.area());

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

    let [left_top, left_bottom] =
        Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
            .margin(2)
            .areas(left);

    let [music_list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(left_top);
    let [player_area] = Layout::horizontal([Constraint::Percentage(80)])
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

    frame.render_widget(left_top_block, left_top);
    frame.render_widget(left_bottom_block, left_bottom);
    let musics = &state.musics;
    let table = create_table(musics);
    // TODO: Find more efficient way than cloning.
    let mut table_state = state.table_state.clone();
    frame.render_stateful_widget(table, music_list_area, &mut table_state);

    if state.is_adjusting {
        let volume = Line::from(vec![
            Span::styled(" VOLUME: ", Style::default().fg(BY_COLOR)),
            Span::from(volume.to_string()),
        ])
        .centered();

        let centered_area = center(
            left_top,
            Constraint::Percentage(100),
            Constraint::Percentage(100),
        );
        let volume_paragraph = Paragraph::new(volume).block(Block::new().padding(Padding::new(
            0,
            0,
            centered_area.height / 2,
            0,
        )));

        frame.render_widget(Clear, left_top);
        frame.render_widget(volume_paragraph, centered_area);
    }

    // Player Section
    let player_color = if is_playing {
        CUSTOM_LABEL_COLOR
    } else {
        Color::Gray
    };
    let mut index = 8; // Point at something on startup.

    if let Some(current_index) = state.current_track_index {
        index = current_index;
    }

    if let Some(music) = state.musics.get(index) {
        let title = title_block(&player_color, &music.author, &music.name);
        render_progress(
            &position,
            progress_bar,
            frame.buffer_mut(),
            title,
            music.length as f64,
        );
    }
}

fn render_progress(progress: &Duration, area: Rect, buf: &mut Buffer, title: Block, duration: f64) {
    let label = Span::styled(
        format!("{}/{}", progress.as_secs(), duration),
        Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
    );

    let progress = progress.as_secs_f64();
    let ratio = ((progress / duration) * 100.0).round() / 100.0;
    if ratio > 1.0 {
        return;
    }

    // TODO: Use CLear before writing.
    LineGauge::default()
        .block(title)
        .filled_style(GAUGE_COLOR)
        .ratio(ratio)
        .label(label)
        .render(area, buf);
}

fn title_block<'a>(color: &'a Color, author: &'a str, name: &'a str) -> Block<'a> {
    let author = Line::from(vec![
        Span::styled(" by ", Style::default().fg(BY_COLOR)),
        Span::from(author),
    ])
    .right_aligned();
    let name = Line::from(name).centered();

    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title_bottom(author)
        .title_bottom(name)
        .fg(*color)
}

fn create_table(tracks: &Vec<Audio>) -> Table {
    let header = Row::new(["Song", "Artist", "Duration"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows: Vec<Row> = tracks
        .iter()
        .map(|item| {
            let style = match item.is_playing {
                true => Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
                _ => Style::default(),
            };

            //TODO: Is cloning the only way? Investigate.
            Row::new([
                item.name.clone(),
                item.author.clone(),
                item.length.to_string(),
            ])
            .style(style)
        })
        .collect();

    //let footer = Row::new(["Lemon", "Lemon Tree", "000"]);

    let widths = [
        Constraint::Percentage(50),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
    ];
    let table = Table::new(rows, widths)
        //.footer(footer.italic())
        //.style(Color::White)
        //.row_highlight_style(Style::new().on_black().bold())
        //.column_highlight_style(Color::Gray)
        //.cell_highlight_style(Style::new().reversed().yellow())
        .header(header)
        .column_spacing(1)
        .row_highlight_style(Style::new().fg(Color::Green))
        .highlight_symbol("  -  ");
    table
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
