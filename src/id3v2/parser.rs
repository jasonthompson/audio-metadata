use std::io::{BufferedReader, Reader, File};
use id3v2::header;
use id3v2::frame;

pub struct Parser {
    pub reader: BufferedReader<File>,
    pub current_position: uint,
    pub length_of_data: uint
}

impl Parser {
    pub fn new(path: &Path, reader: &BufferedReader<File>) -> Parser {
        let mut file =File::open(path).unwrap();
        
        Parser {
            reader: reader,
            current_position: 0u,
            length_of_data: 0u
        }
    }

    pub fn get_next_frame_header(&self) -> Option<frame::FrameHeader> {
        let frame_header_vec = self.reader.read_exact(10).unwrap();
        if frame_header_vec[0] as char != 'T' {
            None
        } else {
            Some(frame::FrameHeader::new(frame_header_vec))
        }
    }
    
    pub fn parse(&mut self) -> SongMetadata {
        // parse header
        
        let header = header::Header::new(&self.reader.read_exact(10).unwrap());

        let mut song = SongMetadata::new(header);

        let mut frame_header: frame::FrameHeader;
        
        match self.get_next_frame_header(){
            Some(ref h) => h,
            None => &frame_header,
        };

        //let header_size = frame_header.
        let frame_contents_vec = self.reader.read_exact(frame_header.size).unwrap();

        let frame = frame::Frame::new(frame_header, frame_contents_vec);
        &song.frames.push(frame);

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
