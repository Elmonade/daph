use std::path::PathBuf;

use super::*;

#[test]
fn test_search() {
    let mut tracks = Vec::new();
    tracks.push(Audio {
        is_playing: (false),
        name: (String::from("Hello")),
        author: (String::from("Adele")),
        length: 999,
        path: PathBuf::new(),
    });

    tracks.push(Audio {
        is_playing: (false),
        name: (String::from("Commit Ballad")),
        author: (String::from("toe")),
        length: 999,
        path: PathBuf::new(),
    });

    assert_eq!(search(&tracks, &String::from("Hello")).len(), 1);
    assert_eq!(search(&tracks, &String::from("Bye")).len(), 0);
}
