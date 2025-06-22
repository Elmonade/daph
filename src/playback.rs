use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, sync::mpsc, thread};

use crate::Audio;
use crate::Command;
use crate::PlayerState;

// TODO: Spawn a thread for the playback.
// OOOOOH I'm creating new sink every single time...
// Create only one sink in the PlayerState struct.
pub fn setup(state: &mut PlayerState) {
    let (tx, rx) = mpsc::channel::<Command>();
    state.tx = tx;
    state.message = String::from("I got updated.");
    let _ = thread::Builder::new()
        .name("playback".to_string())
        .spawn(move || {

            loop {
                if let Ok(command) = rx.try_recv() {
                    println!("Received command: {:?}", command);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
}

pub fn play(index: usize, _is_toggle: bool, path: PathBuf, state: &mut PlayerState) {
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);
    let source = Decoder::new(buffer).unwrap();

    state.sink.append(source);
    state.sink.sleep_until_end();
}
