use std::io::{BufferedReader, File, IoError};
use id3v2::header;
use id3v2::frame;

pub  struct Parser {
    reader: BufferedReader<File>,
}

impl Parser {
    pub fn new(path: &Path) -> Parser {
        Parser {
            reader: BufferedReader::new(File::open(path).unwrap()),
        }
    }

    fn next_frame(&mut self) -> Result<frame::Frame, IoError> {
        let frame_header_vec = try!(self.reader.read_exact(10));
        let frame_header = frame::FrameHeader::new(frame_header_vec).unwrap();
        let frame_header_size = frame_header.size;
        let frame_contents_vec = try!(self.reader.read_exact(frame_header_size));

        Ok(frame::Frame::new(frame_header, frame_contents_vec))
    }
            
    pub fn parse(&mut self) -> SongMetadata {
        let header_vec = self.reader.read_exact(10).unwrap();
        let header = header::Header::new(&header_vec);
        let song = SongMetadata::new(header);

        // iterate through frames
        while let Ok(x) = self.next_frame() {

            println!("{}", &x);
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

