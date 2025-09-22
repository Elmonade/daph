/*
*
* Based on the given Message mutate the PlayerModel.
*
*/
use crate::State;
use crate::fuzzy_search::search;
use crate::order::Order;
use crate::utility::order_by;
use crate::{Message, PlayerModel, play_new_track};
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) enum Command {
    PlayPause(PathBuf),
    New(PathBuf),
    Forward(usize, usize),
    Backward(usize),
    Volume(f32),
}

pub(crate) fn update<'a>(message: &Message<'a>, model: &mut PlayerModel<'a>) {
    match model.state {
        State::Searching => in_search(message, model),
        State::Configuring => in_config(message, model),
        State::Playing => in_playback(message, model),
        State::Adjusting => in_volume(message, model),
    };
}

pub(crate) fn in_config<'a>(message: &Message<'a>, model: &mut PlayerModel<'a>) -> Message<'a> {
    match message {
        Message::Escape => model.state = &State::Playing,
        Message::SwapTo(state) => match state {
            Some(state) => model.state = state,
            None => {
                if model.state == &State::Playing {
                    model.state = &State::Configuring;
                } else if model.state == &State::Configuring {
                    model.state = &State::Playing;
                }
            }
        },
        Message::Down => {
            if let Some(selected_index) = model.order_list_state.selected()
                && selected_index < 3
            {
                model.order_list_state.select_next();
            }
        }
        Message::Up => model.order_list_state.select_previous(),
        Message::Submit => {
            if let Some(index) = model.order_list_state.selected() {
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
            return Message::Submit;
        }
        _ => {}
    };
    Message::None
}

pub(crate) fn in_search<'a>(message: &Message<'a>, model: &mut PlayerModel<'a>) -> Message<'a> {
    match message {
        Message::AppendKeyword(c) => {
            model.keyword.push(*c);
            model.matched_tracks = search(&model.tracks, &model.keyword);
        }
        Message::RemoveKeyword => {
            model.keyword.pop();
            model.matched_tracks = search(&model.tracks, &model.keyword);
        }
        Message::Down => {
            if let Some(selected_index) = model.search_list_state.selected()
                && selected_index < model.matched_tracks.len()
            {
                model.search_list_state.select_next();
            }
        }
        Message::Up => model.search_list_state.select_previous(),
        Message::Escape => model.state = &State::Playing,
        //TODO: Add to que. If none, start playing immediately.
        Message::Submit => model.state = &State::Playing,
        _ => (),
    };
    Message::None
}

pub(crate) fn in_playback<'a>(message: &Message<'a>, model: &mut PlayerModel<'a>) -> Message<'a> {
    match message {
        Message::SwapTo(state) => match state {
            Some(state) => model.state = state,
            None => {
                if model.state == &State::Playing {
                    model.state = &State::Configuring;
                } else if model.state == &State::Configuring {
                    model.state = &State::Playing;
                }
            }
        },
        Message::Escape => std::process::exit(0),
        Message::PlayPause => {
            model
                .tx
                .send(Command::PlayPause(PathBuf::new()))
                .unwrap_or(());
        }
        Message::AppendTrack => {
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
        Message::Delete => {
            // TODO: Implement.
            if let Some(index) = model.table_state.selected() {
                model.tracks.remove(index);
            }
        }
        Message::Down => {
            if let Some(selected_index) = model.table_state.selected()
                && selected_index < model.number_of_tracks - 1
            {
                model.table_state.select_next();
                model.scrollbar_state = model.scrollbar_state.position(selected_index);
            }
        }
        Message::Up => {
            model.table_state.select_previous();
            if let Some(selected_index) = model.table_state.selected() {
                model.scrollbar_state = model.scrollbar_state.position(selected_index);
            }
        }
        Message::Previous => {
            if let Some(mut index) = model.current_track_index {
                model.tracks[index].is_playing = false;
                index = (index + model.number_of_tracks - 1) % model.number_of_tracks;
                play_new_track(index, model);
            }
        }
        Message::Next => {
            if let Some(mut index) = model.current_track_index {
                model.tracks[index].is_playing = false;
                index = (index + 1) % model.number_of_tracks;
                play_new_track(index, model);
            }
        }
        Message::SeekBack => {
            model
                .tx
                .send(Command::Backward(model.seek_distance))
                .unwrap_or(());
        }
        Message::SeekForward => {
            if let Some(index) = model.current_track_index {
                let length = model.tracks[index].length;
                model
                    .tx
                    .send(Command::Forward(model.seek_distance, length as usize))
                    .unwrap_or(());
            }
        }
        _ => (),
    }
    Message::None
}

pub(crate) fn in_volume<'a>(message: &Message<'a>, model: &mut PlayerModel<'a>) -> Message<'a> {
    match message {
        Message::Up => {
            model.iteration_count = 0;
            if model.volume < 2.0 {
                model
                    .tx
                    .send(Command::Volume(model.volume_step))
                    .unwrap_or(());
            }
        }
        Message::Down => {
            model.iteration_count = 0;
            if model.volume > 0.0 {
                model
                    .tx
                    .send(Command::Volume(-model.volume_step))
                    .unwrap_or(());
            }
        }
        _ => (),
    }
    Message::None
}
