/*
*
* Map the user input to set of pre-defined Messages.
*
*/
use crate::PlayerModel;
use crate::{Message, State};
use crossterm::event::{self, KeyEvent};

pub(crate) fn map_to_message(key: KeyEvent, model: &PlayerModel) -> Message {
    match model.state {
        State::Searching => from_search(key),
        State::Configuring => from_config(key),
        State::Playing => from_playback(key),
        State::Adjusting => from_volume(key),
    }
}

pub(crate) fn from_config(key: KeyEvent) -> Message {
    match key.code {
        event::KeyCode::Tab => return Message::Swap,
        event::KeyCode::Char(char) => match char {
            'K' => return Message::SwapToAdjusting,
            'J' => return Message::SwapToAdjusting,
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

pub(crate) fn from_search(key: KeyEvent) -> Message {
    match key.code {
        event::KeyCode::Char(c) => Message::AppendKeyword(c),
        event::KeyCode::Backspace => Message::RemoveKeyword,
        event::KeyCode::Esc => Message::Escape,
        event::KeyCode::Enter => return Message::Submit,
        _ => Message::None,
    };
    Message::None
}

pub(crate) fn from_playback(key: KeyEvent) -> Message {
    match key.code {
        event::KeyCode::Char(char) => match char {
            'K' => return Message::SwapToAdjusting,
            'J' => return Message::SwapToAdjusting,
            '/' => return Message::SwapToSearching,
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
        event::KeyCode::Tab => return Message::Swap,
        event::KeyCode::Esc => return Message::Escape,
        _ => Message::None,
    };
    Message::None
}

pub(crate) fn from_volume(key: KeyEvent) -> Message {
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
