use super::*;
use crate::PATH;
use walkdir::WalkDir;

use lofty::read_from_path;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;

pub(crate) fn load_audio() -> (usize, Vec<Audio>) {
    let mut tracks = Vec::new();
    for entry in WalkDir::new(PATH) {
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

pub(crate) fn play_new_track(index: usize, state: &mut PlayerState) {
    state.current_track_index = Some(index);
    state.tracks[index].is_playing = true;

    let path = state.tracks[index].path.clone();
    state.tx.send(Command::New(path)).unwrap_or(());
}
