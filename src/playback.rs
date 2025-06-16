use rodio::{Decoder, OutputStream, Sink};
use std::sync::mpsc::{Sender, channel};
use std::{fs::File, io::BufReader, path::PathBuf, sync::mpsc::Receiver, thread};

use crate::Command;

pub struct PlayBack {
    tx: Sender<Command>,
    rx: Receiver<Command>,
    sink: Sink,
}

impl PlayBack {
    pub fn new() -> Self {
        let (tx, rx) = channel::<Command>();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self { tx, rx, sink }
    }

    pub fn sink_setup() {}

    pub fn play(self: Self, _index: usize, _is_toggle: bool, path: PathBuf) {
        let _ = thread::Builder::new()
            .name("playback".to_string())
            .spawn(move || {
                let file = File::open(path).unwrap();

                let buffer = BufReader::new(file);
                let source = Decoder::new(buffer).unwrap();

                self.sink.append(source);
                loop {
                    if let Ok(command) = self.rx.recv() {
                        println!("Received command.");
                        self.sink.pause();
                    }
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            });
    }

    pub fn send_command(&self, command: Command) {
        self.tx.send(command);
    }
}
