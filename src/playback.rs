use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

use crate::Audio;

pub fn play(index: usize, is_toggle: bool, tracks: &Vec<Audio>) {
    println!("Playing track: {}. Toggle: {}", index, is_toggle);
    println!("Playing track: {}", tracks[index].path.display());
    let path = &tracks[index].path;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = File::open(path).unwrap();

    let buffer = BufReader::new(file);
    let source = Decoder::new(buffer).unwrap();

    println!("Append to the sink");
    sink.append(source);
    println!("Music should be playing by now.");
    sink.play();
}
