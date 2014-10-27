#![feature(globs)]
extern crate audio_metadata;

use std::io::{File, BufferedReader};

use audio_metadata::id3v2::header;

#[allow(dead_code)]
fn is_id3v2(header_info: &Vec<u8>) -> bool {
    header_info[0] as char == 'I' && header_info[1] as char == 'D' && header_info[2] as char == '3'
}

#[allow(dead_code)]
fn main(){
    let path = Path::new("/home/jason/dev/audio-metadata/sample-data/way.mp3");
    let mut reader = BufferedReader::new(File::open(&path));
    let header_vec = reader.read_exact(10).unwrap();
    if is_id3v2(&header_vec) {
        let header = header::Header::new(header_vec);
        println!("{}", header);
    }
}

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
