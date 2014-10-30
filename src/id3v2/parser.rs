use std::io::{BufferedReader, File};
use id3v2::header;
use id3v2::frame;

struct Parser {
    pub current_position: uint,
    pub length_of_data: uint
}

impl Parser {
    fn new(current_position: uint, length_of_data: uint) -> Parser {
        Parser {
            current_position: current_position,
            length_of_data: length_of_data
        }
    }
}


pub fn parse(mut reader: BufferedReader<File>) -> SongMetadata {
    let mut header_vec = reader.read_exact(10);
    let header = header::Header::new(&header_vec.unwrap());
    let song = SongMetadata::new(header);

    let mut parser = Parser::new(10, header.tag_length);

    song
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
        let mut reader = BufferedReader::new(File::open(&path).unwrap());

        let song_data = super::parse(reader);
        assert_eq!(song_data.header.tag_length, 1033);
    }
}
