use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, thread};

use crate::Audio;

// TODO: Spawn a thread for the playback.
pub fn play(index: usize, _is_toggle: bool, path: PathBuf) {
    let _ = thread::Builder::new()
        .name("playback".to_string())
        .spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let file = File::open(path).unwrap();

            let buffer = BufReader::new(file);
            let source = Decoder::new(buffer).unwrap();

            sink.append(source);
            sink.sleep_until_end();

            loop {
                // TODO: Listen for command from main thread
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });
}
