use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

use crate::Audio;

pub fn play(index: usize, is_toggle: bool, tracks: &Vec<Audio>) {
    println!("Playing track: {}. Toggle: {}", index, is_toggle);
    println!("Playing track: {}", tracks[index].path);
    let path = tracks[index].path.clone();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = BufReader::new(File::open(path).unwrap());
    println!("Opened the file.");
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    // I 'had' to do this. So is_paused will pick it up.
    //sink.play();
    //sink.pause();


    sink.play();
}
