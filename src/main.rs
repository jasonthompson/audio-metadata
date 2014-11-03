#![feature(globs)]
extern crate audio_metadata;

use std::io::{File, BufferedReader};
use std::string::String;
use std::ascii::AsciiStr;

use audio_metadata::id3v2::header;
use audio_metadata::id3v2::frame;
use audio_metadata::id3v2::parser;

fn is_id3v2(header_info: &Vec<u8>) -> bool {
    header_info[0] as char == 'I' && header_info[1] as char == 'D' && header_info[2] as char == '3'
}

fn main(){
    let path = Path::new("/home/jason/dev/audio-metadata/sample-data/discotrax.mp3");
    let mut parser = parser::Parser::new();
    let song_data = parser.parse(&path);

}

#[cfg(test)]
mod test {
    #[test]
    fn test_is_id3v2_true() {
        let header_vec: Vec<u8> = vec![73, 68, 51];
        assert!(super::is_id3v2(&header_vec));
    }

    #[test]
    fn test_is_id3v2_false(){
        let wrong_header_vec: Vec<u8> = vec![97, 76, 102, 67];
        assert_eq!(super::is_id3v2(&wrong_header_vec), false);
    }
}
