use crate::button_handler::handle_config;
use crate::button_handler::handle_playback;
use crate::button_handler::handle_search;
use crate::button_handler::handle_volume;
use crate::config::Config;
use crate::model::PlayerModel;
use crate::utility::play_new_track;
use crate::view::render;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use player::SinkModel;
use ratatui::DefaultTerminal;
use std::path::PathBuf;
use std::result::Result::Ok;
use std::time::Duration;
mod button_handler;
mod config;
mod fuzzy_search;
mod model;
mod order;
mod player;
mod utility;
mod view;

#[derive(Debug, Clone)]
struct Audio {
    is_playing: bool,
    name: String,
    author: String,
    length: u64,
    path: PathBuf,
}

#[derive(Debug)]
pub(crate) enum Command {
    PlayPause(PathBuf),
    New(PathBuf),
    Forward(usize, usize),
    Backward(usize),
    Volume(f32),
}

#[derive(PartialEq, Debug)]
enum State {
    Searching,
    Configuring,
    Adjusting,
    Playing,
}

enum Action {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    env_logger::init();

    // TODO: Handle this unwrap.
    let config_path = home::home_dir().unwrap().join(".config/daph.toml");

    let mut model = PlayerModel::create(Config::new(&config_path));
    let (command_tx, sink_rx) = SinkModel::create();
    model.tx = command_tx;
    model.sink_rx = sink_rx;

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut model);
    let _ = ratatui::try_restore();
    result
}

fn run(mut terminal: DefaultTerminal, model: &mut PlayerModel) -> Result<()> {
    let mut previous_state = model.state;
    loop {
        if let Ok(sink) = model.sink_rx.recv_timeout(Duration::from_millis(33)) {
            // Render
            terminal.draw(|f| render(f, model, &sink))?;

            // Input
            if event::poll(std::time::Duration::from_millis(16))?
                && let Event::Key(key) = event::read()?
            {
                if model.state != &State::Adjusting {
                    previous_state = model.state;
                }
                match model.state {
                    State::Searching => match handle_search(key, model) {
                        Action::Escape => model.state = &State::Playing,
                        Action::Submit => {}
                        Action::None => {}
                    },
                    State::Configuring => match handle_config(key, model) {
                        Action::Escape => model.state = &State::Playing,
                        Action::Submit => {}
                        Action::None => {}
                    },
                    State::Playing => match handle_playback(key, model) {
                        Action::Escape => break,
                        Action::Submit => {}
                        Action::None => {}
                    },
                    State::Adjusting => match handle_volume(key, model) {
                        Action::Escape => break,
                        Action::Submit => {}
                        Action::None => {}
                    },
                }
            }

            // Auto-Queue
            if sink.current_track_finished
                && let Some(mut index) = model.current_track_index
            {
                model.tracks[index].is_playing = false;
                index = (index + 1) % model.number_of_tracks;
                play_new_track(index, model);
            }

            /*
            Assume two threads are perfectly in sync(probably impossible).
            In total, one iteration should take 49ms when no button is pressed.
            2s / 49ms = ~41
            */
            if model.state == &State::Adjusting {
                model.iteration_count += 1;
                if model.iteration_count % 41 == 0 {
                    eprintln!("{:?}", previous_state);
                    model.state = previous_state;
                    model.iteration_count = 0;
                }
            }
        }
    }
    Ok(())
}
