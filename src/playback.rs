use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, sync::mpsc, thread, time};

use crate::Command;

pub(crate) struct SinkState {
    pub que_len: usize,
    pub is_paused: bool,
    pub is_empty: bool,
}

pub fn setup() -> (mpsc::Sender<Command>, mpsc::Receiver<SinkState>) {
    let (command_tx, command_rx) = mpsc::channel::<Command>();
    let (state_tx, state_rx) = mpsc::channel::<SinkState>();

    let _ = thread::Builder::new()
        .name("playback".to_string())
        .spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            loop {
                if let Ok(command) = command_rx.try_recv() {
                    audio_command(command, &sink);
                }
                thread::sleep(time::Duration::from_millis(100));

                //TODO: Not a good idea to recreate this variable every 100ms.
                let sink_state = SinkState {
                    que_len: sink.len(),
                    is_paused: sink.is_paused(),
                    is_empty: sink.empty(),
                };

                state_tx.send(sink_state).unwrap_or(());
            }
        });
    (command_tx, state_rx)
}

fn audio_command(_message: Command, sink: &Sink) {
    match _message {
        Command::PlayPause(path, _) => play_pause(sink, &path),
        Command::Forward(path, _) => skip_forward(sink, &path),
        Command::Backward(_, _) => skip_backward(sink),
        Command::New(path, _) => new_song(sink, &path),
        Command::Next(_, _) => todo!(),
        Command::Previous(_, _) => todo!(),
    }
}

fn new_song(sink: &Sink, path: &PathBuf) {
    if sink.is_paused() {
        let file = File::open(path).unwrap();
        let buffer = BufReader::new(file);
        let source = Decoder::new(buffer).unwrap();

        //TODO: Does queue get full? I assume it's based on RAM.
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
