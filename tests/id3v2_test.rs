#![feature(globs)]

extern crate audio_metadata;

//use std::io::BufferedReader;

use audio_metadata::id3v2::*;

#[test]
fn test_id3v2_parser() {
    let path = Path::new("/home/jason/dev/audio-metadata/sample-data/discotrax.mp3");
    let mut parser = parser::Parser::new();
    let song_data = parser.parse(&path);
    
    assert_eq!(song_data.header.tag_length, 1033);
    assert_eq!(song_data.frames[0].contents, "Discotraxx".to_string());
    assert_eq!(song_data.frames[1].contents, "Discotraxx".to_string());
}
