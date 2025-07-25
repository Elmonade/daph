use super::*;
use walkdir::WalkDir; 
use crate::PATH;

use lofty::read_from_path;

use color_eyre::eyre::Error;
use lofty::tag::Accessor;
use lofty::file::{AudioFile, TaggedFileExt};

pub(crate) fn load_audio() -> Result<(Vec<Audio>, usize), Error> {
    let mut musics = Vec::new();
    let mut number_of_tracks = 0;
    for entry in WalkDir::new(PATH) {
        // TODO: Catch it on main, or deal with it here. Don't just throw it.
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            if extension == "mp3" || extension == "flac" || extension == "wav" {
                number_of_tracks += 1;
                let path = entry.path();
                let tagged_file = match read_from_path(path) {
                    Ok(it) => it,
                    Err(_) => {
                        eprintln!("\nCan't read the file: {}", path.display());
                        continue
                    },
                };

                let tag = match tagged_file.primary_tag() {
                    Some(primary_tag) => primary_tag,
                    None => {
                        eprintln!("\nGiven file has no readable tags: {}", path.display());
                        continue;
                    }
                };

                let tag_title = tag.title();
                let title = String::from(tag_title.as_deref().unwrap_or("None"));
                let tag_artist = tag.artist();
                let artist = String::from(tag_artist.as_deref().unwrap_or("None"));
                let properties = tagged_file.properties();
                let seconds = properties.duration().as_secs();

                musics.push(Audio {
                    is_playing: (false),
                    name: (title),
                    author: (artist),
                    length: seconds,
                    path: path.to_path_buf(),
                });
            }
        }
    }
    Ok((musics, number_of_tracks))
}

pub(crate) fn play_new_track(index: usize, state: &mut PlayerState) {
    state.current_track_index = Some(index);
    state.musics[index].is_playing = true;

    let path = state.musics[index].path.clone();
    state.tx.send(Command::New(path)).unwrap_or(());
}
