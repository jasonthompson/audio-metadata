#![feature(globs)]
extern crate audio_metadata;

use std::io::{File, BufferedReader};
use std::string::String;
use std::ascii::AsciiStr;

use audio_metadata::id3v2::header;
use audio_metadata::id3v2::frame;

fn is_id3v2(header_info: &Vec<u8>) -> bool {
    header_info[0] as char == 'I' && header_info[1] as char == 'D' && header_info[2] as char == '3'
}

fn main(){
    let mut song: Vec<frame::Frame> = vec![];
    let path = Path::new("/home/jason/dev/audio-metadata/sample-data/high.mp3");
    let mut reader = BufferedReader::new(File::open(&path));
    let header_vec = reader.read_exact(10).unwrap();
    let id3v2_header = header::Header::new(&header_vec);
    let id3_size: &uint = &id3v2_header.tag_size.to_uint().unwrap();
    
    if is_id3v2(&header_vec) {
        let position: &uint = &10;

        while position <= id3_size {
            let fheader = frame::FrameHeader::new(reader.read_exact(10).unwrap());
            let frame_size = fheader.size as uint;
            let mut text = reader.read_exact(frame_size).unwrap();
            let encoding = text.remove(0);
            song.push(frame::Frame::new(fheader, text));
            let position = position + frame_size;
        }
    }
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
