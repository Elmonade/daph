use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::{self, Duration},
};

use crate::Command;

pub(crate) struct SinkState {
    pub _is_paused: bool,
    pub _is_empty: bool,
    pub is_playing: bool,
    pub current_track_finished: bool,
    pub position: Duration,
    pub volume: f32,
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
                    _is_paused: sink.is_paused(),
                    _is_empty: sink.empty(),
                    is_playing,
                    current_track_finished,
                    position: sink.get_pos(),
                    volume: sink.volume(),
                };

                current_track_finished = false;
                state_tx.send(sink_state).unwrap_or(());

                was_playing = is_playing;
                thread::sleep(time::Duration::from_millis(33));
            }
        });
    (command_tx, state_rx)
}

fn audio_command(_message: Command, sink: &Sink) {
    match _message {
        Command::PlayPause(path) => play_pause(sink, &path),
        Command::New(path) => new_song(sink, &path),
        Command::Forward(distance, length) => seek_forward(sink, distance, length),
        Command::Backward(distance) => seek_backward(sink, distance),
        Command::_Next(_, _) => next(sink),
        Command::_Append(path, _) => append(sink, &path),
        Command::_Previous(_, _) => todo!(),
        Command::Volume(step) => volume_control(sink, step),
    }
}

fn volume_control(sink: &Sink, step: f32) {
    let mut volume = sink.volume();
    if (step < 0.0 && volume > 0.0) || (step > 0.0 && volume < 2.0) {
        //TODO: Isn't there a way to just limit the precision?
        volume = ((volume + step) * 100.0).round() / 100.0;
        sink.set_volume(volume);
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

fn play_pause(sink: &Sink, _path: &Path) {
    match sink.is_paused() {
        false => {
            sink.pause();
        }
        true => {
            sink.play();
        }
    }
}

fn seek_forward(sink: &Sink, distance: usize, length: usize) {
    let position = sink.get_pos();
    if (position.as_secs() + distance as u64) < length as u64 {
        let seek_to = sink.get_pos() + Duration::new(distance as u64, 0);
        _ = sink.try_seek(seek_to);
    }
}

fn seek_backward(sink: &Sink, distance: usize) {
    let position = sink.get_pos();
    if position.as_secs() > distance as u64 {
        let seek_to = position - Duration::new(distance as u64, 0);
        _ = sink.try_seek(seek_to);
        return;
    }

    _ = sink.try_seek(Duration::ZERO);
}
