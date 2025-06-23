use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, sync::mpsc, thread};

use crate::Command;

pub fn setup() -> mpsc::Sender<Command>{
    let (tx, rx) = mpsc::channel::<Command>();

    let _ = thread::Builder::new()
        .name("playback".to_string())
        .spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            loop {
                if let Ok(command) = rx.try_recv() {
                    println!("Received command: {:?}", command);
                    audio_command(command, &sink);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
    tx
}

pub(crate) fn audio_command(_message: Command, sink: &Sink) {
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
    sink.stop();

    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let source = Decoder::new(buffer).unwrap();

    sink.append(source);
}

fn play_pause(sink: &Sink, _path: &PathBuf) {
    match sink.is_paused() {
        false => sink.pause(),
        true => sink.play(),
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
