use std::io::{BufferedReader, File};
use id3v2::header;
use id3v2::frame;

struct Parser<'a> {
    pub current_position: &'a mut uint,
    pub length_of_data: uint
}

impl<'a> Parser<'a> {
    pub fn new<'a>() -> Parser<'a> {
        return Parser {
            current_position: &mut 0u,
            length_of_data: 0
        }
    }
    
    pub fn parse(&mut self, path: &Path) -> SongMetadata {
        let mut reader = BufferedReader::new(File::open(path));

        // parse header
        let mut header_vec = reader.read_exact(10);
        let header = header::Header::new(&header_vec.unwrap());
        let mut song = SongMetadata::new(header);


        // iterate through frames
        while self.current_position.to_uint().unwrap() <= self.length_of_data {
            // read frame header
            let frame_header_vec = reader.read_exact(10).unwrap();
            let frame_header = frame::FrameHeader::new(frame_header_vec);
            let header_size = frame_header.size.clone();
            let frame_contents_vec = reader.read_exact(frame_header.size).unwrap();

            let frame = frame::Frame::new(frame_header, frame_contents_vec);
            song.frames.push(frame);
            self.current_position = &mut (header_size + 10);
        }
        song
    }
}

pub struct SongMetadata {
    header: header::Header,
    frames: Vec<frame::Frame>
}

impl SongMetadata {
    pub fn new(header: header::Header) -> SongMetadata {
        SongMetadata {
            header: header,
            frames: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufferedReader, File};
    use id3v2::header;
    
    #[test]
    fn test_parser_initialization() {
        let path = Path::new("/home/jason/dev/audio-metadata/sample-data/discotrax.mp3");
        let mut parser = super::Parser::new;
        let song_data = parser.parse(&path);
        
        assert_eq!(song_data.header.tag_length, 1033);
    }
}
