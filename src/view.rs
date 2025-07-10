use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Padding};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Paragraph, Widget, Row, Table},
};
use crate::PlayerState;
use crate::Audio;

pub(crate) fn render(frame: &mut Frame, player_state: & PlayerState, is_playing: bool) {
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)])
            .margin(0)
            .areas(frame.area());
    let [left_top, left_bottom] =
        Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)])
            .margin(0)
            .areas(left);

    if player_state.is_searching {
        //TODO: Dynamic Scaling OR Make it toggleable
        Paragraph::new(player_state.keyword.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .border_type(BorderType::Rounded)
                    .padding(Padding::uniform(1))
                    .title("SEARCH"),
            )
            .render(right, frame.buffer_mut());
    }

    let [music_list_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(left_top);
    let [player_area] = Layout::horizontal([Constraint::Fill(1)])
        .margin(1)
        .areas(left_bottom);

    let left_top_block = Block::bordered()
        .title("LIBRARY")
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow);

    let left_bottom_block = Block::bordered()
        .title("PLAYER")
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow);

    frame.render_widget(left_top_block, left_top);
    frame.render_widget(left_bottom_block, left_bottom);
    let musics = &player_state.musics;
    let table = create_table(musics);
    // TODO: Find more efficient way than cloning.
    let mut table_state = player_state.table_state.clone();
    frame.render_stateful_widget(table, music_list_area, &mut table_state);

    let mut index = 0;
    if let Some(current_index) = player_state.current_track_index {
        index = current_index;
    }

    // TODO: Use iterator to replace the clone
    let current_track_name = player_state.musics[index].name.clone();
    let current_track_artist = player_state.musics[index].author.clone();

    if is_playing {
        Paragraph::new(format!(
            " || \n {} - {}",
            current_track_name, current_track_artist
        ))
        .render(player_area, frame.buffer_mut());
    } else {
        Paragraph::new(format!(
            " > \n {} - {}",
            current_track_name, current_track_artist
        ))
        .render(player_area, frame.buffer_mut());
    }
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
        .header(header)
        //.footer(footer.italic())
        .column_spacing(1)
        //.style(Color::White)
        //.row_highlight_style(Style::new().on_black().bold())
        .row_highlight_style(Style::new().fg(Color::Green))
        //.column_highlight_style(Color::Gray)
        //.cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("- ");
    table
}

