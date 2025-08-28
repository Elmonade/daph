use crate::State;
use crate::fuzzy_search::search;
use crate::order::Order;
use crate::utility::order_by;
use crate::{Action, Command, PlayerModel, play_new_track};
use crossterm::event::{self, KeyEvent};
use std::path::PathBuf;

pub(crate) fn handle_config(key: KeyEvent, model: &mut PlayerModel) -> Action {
    match key.code {
        event::KeyCode::Tab => {
            if model.state == &State::Playing {
                model.state = &State::Configuring;
            } else if model.state == &State::Configuring {
                model.state = &State::Playing;
            }
        }
        event::KeyCode::Char(char) => match char {
            'K' => model.state = &State::Adjusting,
            'J' => model.state = &State::Adjusting,
            'j' => {
                if let Some(selected_index) = model.list_state.selected()
                    && selected_index < 3
                {
                    model.list_state.select_next();
                }
            }
            'k' => {
                model.list_state.select_previous();
            }
            _ => {}
        },
        event::KeyCode::Esc => {
            return Action::Escape;
        }
        event::KeyCode::Enter => {
            if let Some(index) = model.list_state.selected() {
                match index {
                    0 => {
                        if let Some(index) =
                            order_by(&Order::Shuffle, &model.playback_order, &mut model.tracks)
                        {
                            model.current_track_index = Some(index);
                            model.playback_order = Order::Shuffle;
                        }
                    }
                    1 => {
                        if let Some(index) =
                            order_by(&Order::Album, &model.playback_order, &mut model.tracks)
                        {
                            model.current_track_index = Some(index);
                            model.playback_order = Order::Album;
                        }
                    }
                    2 => {
                        if let Some(index) =
                            order_by(&Order::Artist, &model.playback_order, &mut model.tracks)
                        {
                            model.current_track_index = Some(index);
                            model.playback_order = Order::Artist;
                        }
                    }
                    3 => {
                        if let Some(index) =
                            order_by(&Order::Track, &model.playback_order, &mut model.tracks)
                        {
                            model.current_track_index = Some(index);
                            model.playback_order = Order::Track;
                        }
                    }
                    _ => {
                        if let Some(index) =
                            order_by(&Order::Shuffle, &model.playback_order, &mut model.tracks)
                        {
                            model.current_track_index = Some(index);
                            model.playback_order = Order::Shuffle;
                        }
                    }
                }
            }
            return Action::Submit;
        }
        _ => {}
    };
    Action::None
}

pub(crate) fn handle_search(key: KeyEvent, model: &mut PlayerModel) -> Action {
    match key.code {
        event::KeyCode::Char(c) => {
            model.keyword.push(c);
            model.matched_tracks = search(&model.tracks, &model.keyword);
        }
        event::KeyCode::Backspace => {
            model.keyword.pop();
            model.matched_tracks = search(&model.tracks, &model.keyword);
        }
        event::KeyCode::Esc => {
            return Action::Escape;
        }
        event::KeyCode::Enter => {
            return Action::Submit;
        }
        _ => {}
    };
    Action::None
}

pub(crate) fn handle_playback(key: KeyEvent, model: &mut PlayerModel) -> Action {
    match key.code {
        event::KeyCode::Tab => {
            if model.state == &State::Playing {
                model.state = &State::Configuring;
            } else if model.state == &State::Configuring {
                model.state = &State::Playing;
            }
        }
        event::KeyCode::Esc => return Action::Escape,
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
    Action::None
}

pub(crate) fn handle_volume(key: KeyEvent, model: &mut PlayerModel) -> Action {
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
    Action::None
}
