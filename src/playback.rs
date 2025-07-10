use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, sync::mpsc, thread, time};

use crate::Command;

pub(crate) struct SinkState {
    pub que_len: usize,
    pub _is_paused: bool,
    pub _is_empty: bool,
    pub is_playing: bool,
    pub current_track_finished: bool,
}

pub fn setup() -> (mpsc::Sender<Command>, mpsc::Receiver<SinkState>) {
    let (command_tx, command_rx) = mpsc::channel::<Command>();
    let (state_tx, state_rx) = mpsc::channel::<SinkState>();

    let _ = thread::Builder::new()
        .name("playback".to_string())
        .spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            let mut was_playing = false;
            let mut sink_state;
            let mut current_track_finished = false;

            loop {
                if let Ok(command) = command_rx.try_recv() {
                    audio_command(command, &sink);
                }

                let is_playing = !sink.empty() && !sink.is_paused();
                if was_playing && sink.empty() {
                    current_track_finished = true;
                }

                sink_state = SinkState {
                    que_len: sink.len(),
                    _is_paused: sink.is_paused(),
                    _is_empty: sink.empty(),
                    is_playing,
                    current_track_finished,
                };

                state_tx.send(sink_state).unwrap_or(current_track_finished = false);

                was_playing = is_playing;
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    (command_tx, state_rx)
}

fn audio_command(_message: Command, sink: &Sink) {
    match _message {
        Command::Append(path, _) => append(sink, &path),
        Command::PlayPause(path, _) => play_pause(sink, &path),
        Command::Forward(path, _) => skip_forward(sink, &path),
        Command::Backward(_, _) => skip_backward(sink),
        Command::New(path, _) => new_song(sink, &path),
        Command::Next(_, _) => next(sink),
        Command::Previous(_, _) => todo!(),
    }
}

fn next(sink: &Sink) {
    sink.skip_one();
}

fn append(sink: &Sink, path: &PathBuf) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let source = Decoder::new(buffer).unwrap();

    sink.append(source);
}

fn new_song(sink: &Sink, path: &PathBuf) {
    if sink.is_paused() {
        let file = File::open(path).unwrap();
        let buffer = BufReader::new(file);
        let source = Decoder::new(buffer).unwrap();

        sink.append(source);
        sink.skip_one();
        sink.play();
    } else {
        sink.stop();
        let file = File::open(path).unwrap();
        let buffer = BufReader::new(file);
        let source = Decoder::new(buffer).unwrap();

        sink.append(source);
    }
}

fn play_pause(sink: &Sink, _path: &PathBuf) {
    match sink.is_paused() {
        false => {
            sink.pause();
        }
        true => {
            sink.play();
        }
    }
}

fn skip_forward(sink: &Sink, path: &PathBuf) {
    sink.stop();

    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let source = Decoder::new(buffer).unwrap();

    sink.append(source);
}

fn skip_backward(_sink: &Sink) {
    todo!();
}
