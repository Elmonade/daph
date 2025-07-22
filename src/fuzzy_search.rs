use crate::Audio;

pub(crate) fn search(tracks: &Vec<Audio>, keyword: &str) -> Vec<Audio> {
    // Create new vector to save search results in
    let mut matches = Vec::new();

    // Iterate over all tracks, check if match
    for track in tracks {
        if track.name.contains(keyword) || track.author.contains(keyword) {
            println!("it does!");
            // Add match to vector
            matches.push(track.clone());
        }
    }
    matches
}

#[cfg(test)]
mod test;
