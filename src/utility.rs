use super::*;
use crate::order::Order;
use rand::Rng;
use walkdir::WalkDir;

use lofty::read_from_path;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;

pub(crate) fn load_audio(path: PathBuf) -> (usize, Vec<Audio>) {
    let mut tracks = Vec::new();
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                if let Some(extension) = entry.path().extension() {
                    if extension == "mp3" || extension == "flac" || extension == "wav" {
                        let path = entry.path();
                        let tagged_file = match read_from_path(path) {
                            Ok(it) => it,
                            Err(_) => {
                                eprintln!("\nCan't read the file: {}", path.display());
                                continue;
                            }
                        };

                        let tag = match tagged_file.primary_tag() {
                            Some(primary_tag) => primary_tag,
                            None => {
                                eprintln!("\nGiven file has no readable tags: {}", path.display());
                                continue;
                            }
                        };

                        let tag_title = tag.title();
                        let tag_artist = tag.artist();
                        let duration = tagged_file.properties().duration();

                        let title = String::from(tag_title.as_deref().unwrap_or("None"));
                        let artist = String::from(tag_artist.as_deref().unwrap_or("None"));
                        let seconds = duration.as_secs();

                        tracks.push(Audio {
                            is_playing: (false),
                            name: title,
                            author: artist,
                            length: seconds,
                            path: path.to_path_buf(),
                        });
                    }
                }
            }
            Err(_) => eprintln!(
                "Cannot access this path: {}",
                entry.unwrap().path().to_str().unwrap()
            ),
        }
    }
    (tracks.len(), tracks)
}

pub(crate) fn order_by(new: &Order, old: &Order, tracks: &mut [Audio]) {
    if new == old {
        return;
    }
    match new {
        Order::Shuffle => order_shuffle(tracks),
        Order::Album => order_album(tracks),
        Order::Artist => order_artist(tracks),
        Order::Track => order_tracks(tracks),
    }
}

fn order_tracks(tracks: &mut [Audio]) {
    tracks.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
}

fn order_artist(tracks: &mut [Audio]) {
    tracks.sort_by(|a, b| {
        let artist_cmp = a.author.to_lowercase().cmp(&b.author.to_lowercase());
        if artist_cmp == std::cmp::Ordering::Equal {
            // If artists are the same, sort by track name
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        } else {
            artist_cmp
        }
    });
}

fn order_album(_tracks: &mut [Audio]) {}

fn order_shuffle(tracks: &mut [Audio]) {
    // Fisher-Yate Algorithm
    let size: usize = tracks.len();
    let mut rng = rand::rng();
    for i in 0..size {
        let random_number: u64 = rng.random();
        let j = random_number as usize % (size - i) + i;
        tracks.swap(i, j);
    }
}

pub(crate) fn play_new_track(index: usize, state: &mut PlayerState) {
    state.current_track_index = Some(index);
    state.tracks[index].is_playing = true;

    let path = state.tracks[index].path.clone();
    state.tx.send(Command::New(path)).unwrap_or(());
}
