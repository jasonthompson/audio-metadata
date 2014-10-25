#![feature(globs)]
extern crate audio_metadata;

use std::io::{File, BufferedReader};

use audio_metadata::id3v2;

fn is_id3(header_info: &Vec<u8>) -> bool {
    header_info[0] == 73 && header_info[1] == 68 && header_info[2] == 51
}

fn main(){
    let path = Path::new("/home/jason/dev/audio-metadata/sample-data/way.mp3");
    let mut reader = BufferedReader::new(File::open(&path));
    let header_vec = reader.read_exact(10).unwrap();
    if is_id3(&header_vec) {
        let header = id3v2::Header::new(header_vec);
        println!("{}", header);
        }
}
