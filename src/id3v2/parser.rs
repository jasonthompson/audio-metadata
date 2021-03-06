use std::io::{BufferedReader, File};
use id3v2::header;
use id3v2::frame;

pub  struct Parser {
    pub current_position: uint,
    pub length_of_data: uint
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            current_position: 0u,
            length_of_data: 0u
        }
    }
    
    pub fn parse(&mut self, path: &Path) -> SongMetadata {
        let mut reader = BufferedReader::new(File::open(path));

        // parse header
        let header_vec = reader.read_exact(10);
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
            &song.frames.push(frame);

            self.current_position = header_size + 10;
        }
        song
    }
}

pub struct SongMetadata {
    pub header: header::Header,
    pub frames: Vec<frame::Frame>
}

impl SongMetadata {
    pub fn new(header: header::Header) -> SongMetadata {
        SongMetadata {
            header: header,
            frames: Vec::new()
        }
    }
}
