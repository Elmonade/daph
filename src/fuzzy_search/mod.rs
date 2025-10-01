use crate::Audio;

pub(crate) fn search(tracks: &[Audio], keyword: &str) -> Vec<usize> {
    let lowercase_key = keyword.to_lowercase();
    tracks
        .iter()
        .enumerate()
        .filter(|(_, track)| {
            track.name.to_lowercase().contains(&lowercase_key)
                || track.author.to_lowercase().contains(&lowercase_key)
        })
        .map(|(index, _)| index)
        .collect()
}

#[cfg(test)]
mod test;
