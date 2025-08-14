use std::path::PathBuf;

use crate::fuzzy_search::search;
use crate::order::Order;
use crate::utility::order_by;
use crate::{play_new_track, Action, Command, PlayerState};
use crossterm::event::{self, KeyEvent};

// TODO: This shoud be inside state.rs
const VOLUME_STEP: f32 = 0.1;

pub(crate) fn handle_config(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Tab => state.is_configuring = !state.is_configuring,
        event::KeyCode::Char(char) => match char {
            'j' => {
                if let Some(selected_index) = state.list_state.selected() {
                    if selected_index < 3 {
                        state.list_state.select_next();
                    }
                }
            }
            'k' => {
                state.list_state.select_previous();
            }
            _ => {}
        },
        event::KeyCode::Esc => {
            return Action::Escape;
        }
        event::KeyCode::Enter => {
            if let Some(index) = state.list_state.selected() {
                match index {
                    0 => {
                        order_by(&Order::Shuffle, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Shuffle;
                    }
                    1 => {
                        order_by(&Order::Album, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Album;
                    }
                    2 => {
                        order_by(&Order::Artist, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Artist;
                    }

                    3 => {
                        order_by(&Order::Track, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Track;
                    }
                    _ => {
                        order_by(&Order::Shuffle, &state.playback_order, &mut state.tracks);
                        state.playback_order = Order::Shuffle;
                    }
                }
            }
            return Action::Submit;
        }
        _ => {}
    };
    Action::None
}

pub(crate) fn handle_search(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Char(c) => {
            state.keyword.push(c);
            state.matched_tracks = search(&state.tracks, &state.keyword);
        }
        event::KeyCode::Backspace => {
            state.keyword.pop();
            state.matched_tracks = search(&state.tracks, &state.keyword);
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

pub(crate) fn handle_playback(key: KeyEvent, state: &mut PlayerState) -> Action {
    match key.code {
        event::KeyCode::Tab => state.is_configuring = !state.is_configuring,
        event::KeyCode::Esc => return Action::Escape,
        event::KeyCode::Char(char) => match char {
            ' ' => {
                state
                    .tx
                    .send(Command::PlayPause(PathBuf::new()))
                    .unwrap_or(());
            }
            ':' => {
                if let Some(index) = state.table_state.selected() {
                    match state.current_track_index {
                        Some(current_index) => {
                            if index == current_index {
                                state
                                    .tx
                                    .send(Command::PlayPause(PathBuf::new()))
                                    .unwrap_or(());
                            } else {
                                state.tracks[current_index].is_playing = false;
                                play_new_track(index, state);
                            }
                        }
                        None => {
                            play_new_track(index, state);
                        }
                    }
                }
            }
            '/' => {
                state.is_searching = true;
            }
            'D' => {
                if let Some(index) = state.table_state.selected() {
                    state.tracks.remove(index);
                }
            }
            'j' => {
                if let Some(selected_index) = state.table_state.selected() {
                    if selected_index < state.number_of_tracks - 1 {
                        state.table_state.select_next();
                    }
                }
            }
            'k' => {
                state.table_state.select_previous();
            }
            'p' => {
                if let Some(mut index) = state.current_track_index {
                    state.tracks[index].is_playing = false;
                    index = (index + state.number_of_tracks - 1) % state.number_of_tracks;
                    play_new_track(index, state);
                }
            }
            'n' => {
                if let Some(mut index) = state.current_track_index {
                    state.tracks[index].is_playing = false;
                    index = (index + 1) % state.number_of_tracks;
                    play_new_track(index, state);
                }
            }
            '<' => {
                state
                    .tx
                    .send(Command::Backward(state.seek_distance))
                    .unwrap_or(());
            }
            '>' => {
                if let Some(index) = state.current_track_index {
                    let length = state.tracks[index].length;
                    state
                        .tx
                        .send(Command::Forward(state.seek_distance, length as usize))
                        .unwrap_or(());
                }
            }
            'K' => {
                state.is_adjusting = true;
                state.iteration_count = 0;
                if state.volume < 2.0 {
                    state.tx.send(Command::Volume(VOLUME_STEP)).unwrap_or(());
                }
            }
            'J' => {
                state.is_adjusting = true;
                state.iteration_count = 0;
                if state.volume > 0.0 {
                    state.tx.send(Command::Volume(-VOLUME_STEP)).unwrap_or(());
                }
            }
            _ => {}
        },
        _ => {}
    }
    Action::None
}
