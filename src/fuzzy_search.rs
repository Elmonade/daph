use crate::Audio;

// TODO: We can probably do a iter, filter, collect chain without intermediary value like
// matches. Looks more rust-like that way.

// Also this is not technically fuzzy. Probably needs bit more work.
pub(crate) fn search(tracks: &Vec<Audio>, keyword: &str) -> Vec<Audio> {
    // Create new vector to save search results in
    let mut matches = Vec::new();

    // Iterate over all tracks, check if match
    for track in tracks {
        if track.name.contains(keyword) || track.author.contains(keyword) {
            // Add match to vector
            matches.push(track.clone());
        }
    }

    matches
}

#[cfg(test)]
mod test;
