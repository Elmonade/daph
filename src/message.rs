use crate::fuzzy_search::search;
use crate::order::Order;
use crate::utility::order_by;
use crate::{Command, PlayerModel, play_new_track};
use crate::{Message, State};
use crossterm::event::Event;
use crossterm::event::{self, KeyEvent};
use std::path::PathBuf;

pub(crate) fn map_to_message(key: KeyEvent, model: &PlayerModel) -> Message {
    match model.state {
        State::Searching => from_search(key),
        State::Configuring => from_config(key, model),
        State::Playing => from_playback(key),
        State::Adjusting => from_volume(key),
    }
}

pub(crate) fn from_config(key: KeyEvent, model: &PlayerModel) -> Message {
    match key.code {
        event::KeyCode::Tab => Message::Swap,
        event::KeyCode::Char(char) => match char {
            'K' => Message::ToAdjusting,
            'J' => Message::ToAdjusting,
            'j' => Message::SelectNext,
            'k' => Message::SelectPrev,
            _ => Message::None,
        },
        event::KeyCode::Esc => return Message::Escape,
        event::KeyCode::Enter => {
            if let Some(index) = model.list_state.selected() {
                match index {
                    0 => Message::Shuffle,
                    1 => Message::Album,
                    2 => Message::Artist,
                    3 => Message::Track,
                    _ => Message::Shuffle,
                };
            }
            return Message::Submit;
        }
        _ => Message::None,
    };
    Message::None
}

pub(crate) fn from_search(key: KeyEvent) -> Message {
    match key.code {
        event::KeyCode::Char(c) => {

            //TODO: Just tell the update program to grab the exsiting text there.
            model.keyword.push(c);
            model.matched_tracks = search(&model.tracks, &model.keyword);
        }
        event::KeyCode::Backspace => {
            model.keyword.pop();
            model.matched_tracks = search(&model.tracks, &model.keyword);
        }
        event::KeyCode::Esc => {
            return Message::Escape;
        }
        event::KeyCode::Enter => {
            return Message::Submit;
        }
        _ => {}
    };
    Message::None
}

pub(crate) fn from_playback(key: KeyEvent) -> Message {
    match key.code {
        event::KeyCode::Tab => {
            if model.state == &State::Playing {
                model.state = &State::Configuring;
            } else if model.state == &State::Configuring {
                model.state = &State::Playing;
            }
        }
        event::KeyCode::Esc => return Message::Escape,
        event::KeyCode::Char(char) => match char {
            'K' => model.state = &State::Adjusting,
            'J' => model.state = &State::Adjusting,
            '/' => model.state = &State::Searching,
            ' ' => {
                model
                    .tx
                    .send(Command::PlayPause(PathBuf::new()))
                    .unwrap_or(());
            }
            ':' => {
                if let Some(index) = model.table_state.selected() {
                    match model.current_track_index {
                        Some(current_index) => {
                            if index == current_index {
                                model
                                    .tx
                                    .send(Command::PlayPause(PathBuf::new()))
                                    .unwrap_or(());
                            } else {
                                model.tracks[current_index].is_playing = false;
                                play_new_track(index, model);
                            }
                        }
                        None => {
                            play_new_track(index, model);
                        }
                    }
                }
            }
            'D' => {
                // TODO: Implement.
                if let Some(index) = model.table_state.selected() {
                    model.tracks.remove(index);
                }
            }
            'j' => {
                if let Some(selected_index) = model.table_state.selected()
                    && selected_index < model.number_of_tracks - 1
                {
                    model.table_state.select_next();
                }
            }
            'k' => {
                model.table_state.select_previous();
            }
            'p' => {
                if let Some(mut index) = model.current_track_index {
                    model.tracks[index].is_playing = false;
                    index = (index + model.number_of_tracks - 1) % model.number_of_tracks;
                    play_new_track(index, model);
                }
            }
            'n' => {
                if let Some(mut index) = model.current_track_index {
                    model.tracks[index].is_playing = false;
                    index = (index + 1) % model.number_of_tracks;
                    play_new_track(index, model);
                }
            }
            '<' => {
                model
                    .tx
                    .send(Command::Backward(model.seek_distance))
                    .unwrap_or(());
            }
            '>' => {
                if let Some(index) = model.current_track_index {
                    let length = model.tracks[index].length;
                    model
                        .tx
                        .send(Command::Forward(model.seek_distance, length as usize))
                        .unwrap_or(());
                }
            }
            _ => {}
        },
        _ => {}
    }
    Message::None
}

pub(crate) fn from_volume(key: KeyEvent) -> Message {
    match key.code {
        event::KeyCode::Char(char) => match char {
            'K' => {
                model.iteration_count = 0;
                if model.volume < 2.0 {
                    model
                        .tx
                        .send(Command::Volume(model.volume_step))
                        .unwrap_or(());
                }
            }
            'J' => {
                model.iteration_count = 0;
                if model.volume > 0.0 {
                    model
                        .tx
                        .send(Command::Volume(-model.volume_step))
                        .unwrap_or(());
                }
            }
            _ => {}
        },
        _ => {}
    }
    Message::None
}
