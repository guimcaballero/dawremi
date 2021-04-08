use crate::song::Song;
use std::io::stdin;

pub fn select_song(mut songs: Vec<Song>) -> Song {
    println!("Select a song:");

    for (idx, song) in songs.iter().enumerate() {
        println!("[{}]: {}", idx, song.name());
    }

    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    let result = s.trim().parse().unwrap_or(0);
    songs.remove(result)
}
