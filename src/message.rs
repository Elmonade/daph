/*
*
* Map the user input to set of pre-defined Messages.
*
*/
use crate::PlayerModel;
use crate::{Message, State};
use crossterm::event::{self, KeyEvent};

pub(crate) fn map_to_message<'a>(key: KeyEvent, model: &PlayerModel) -> Message<'a> {
    match model.state {
        State::Searching => from_search(key),
        State::Configuring => from_config(key),
        State::Playing => from_playback(key),
        State::Adjusting => from_volume(key),
    }
}

pub(crate) fn from_config<'a>(key: KeyEvent) -> Message<'a> {
    match key.code {
        event::KeyCode::Tab => return Message::SwapTo(None),
        event::KeyCode::Char(char) => match char {
            'K' => return Message::SwapTo(Some(&State::Adjusting)),
            'J' => return Message::SwapTo(Some(&State::Adjusting)),
            'j' => return Message::Down,
            'k' => return Message::Up,
            _ => Message::None,
        },
        event::KeyCode::Esc => return Message::Escape,
        event::KeyCode::Enter => return Message::Submit,
        _ => Message::None,
    };
    Message::None
}

pub(crate) fn from_search<'a>(key: KeyEvent) -> Message<'a> {
    match key.code {
        event::KeyCode::Char(c) => return Message::AppendKeyword(c),
        event::KeyCode::Backspace => return Message::RemoveKeyword,
        event::KeyCode::Esc => return Message::Escape,
        event::KeyCode::Enter => return Message::Submit,
        event::KeyCode::Down => return Message::Down,
        event::KeyCode::Up => return Message::Up,
        _ => Message::None,
    };
    Message::None
}

pub(crate) fn from_playback<'a>(key: KeyEvent) -> Message<'a> {
    match key.code {
        event::KeyCode::Char(char) => match char {
            'K' => return Message::SwapTo(Some(&State::Adjusting)),
            'J' => return Message::SwapTo(Some(&State::Adjusting)),
            '/' => return Message::SwapTo(Some(&State::Searching)),
            ' ' => return Message::PlayPause,
            ':' => return Message::AppendTrack,
            'D' => return Message::Delete,
            'j' => return Message::Down,
            'k' => return Message::Up,
            'p' => return Message::Previous,
            'n' => return Message::Next,
            '<' => return Message::SeekBack,
            '>' => return Message::SeekForward,
            _ => Message::None,
        },
        event::KeyCode::Tab => return Message::SwapTo(None),
        event::KeyCode::Esc => return Message::Escape,
        _ => Message::None,
    };
    Message::None
}

pub(crate) fn from_volume<'a>(key: KeyEvent) -> Message<'a> {
    match key.code {
        event::KeyCode::Char(char) => match char {
            'K' => return Message::Up,
            'J' => return Message::Down,
            _ => Message::None,
        },
        _ => Message::None,
    };
    Message::None
}
