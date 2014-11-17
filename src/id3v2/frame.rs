use std::fmt;
use std::str;

use util;

pub enum FrameType {
    Text,
    Other,
}

pub struct Frame {
    pub kind: FrameType, 
    pub header: FrameHeader,
    pub contents: String
}

impl Frame {
    pub fn new(header: FrameHeader, contents_bytes: Vec<u8>) -> Frame {
        if header.id.chars().nth(0).unwrap() == 'T' {
            Frame {
                kind: FrameType::Text,
                header: header,
                contents: String::from_utf8(contents_bytes).unwrap(),
                }
        } else {
            Frame {
                kind: FrameType::Other,
                header: header,
                contents: String::new(),
                }
        }
        
    }
}

impl fmt::Show for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            FrameType::Text => write!(f, "{}: {} \n", self.header, self.contents),
            FrameType::Other => write!(f, "{}", self.header.id),
        }
    }
}


pub struct FrameHeader {
    /// ID3v2 frame overview
    ///
    /// As the tag consists of a tag header and a tag body with one or more
    /// frames, all the frames consists of a frame header followed by one or
    /// more fields containing the actual information. The layout of the
    /// frame header:
    ///
    ///   Frame ID   $xx xx xx xx  (four characters)
    ///   Size       $xx xx xx xx
    ///   Flags      $xx xx
    ///

    pub id: &'static str,
    pub size: uint,
    pub flags: Flags
}

impl FrameHeader {
    pub fn new(header_bytes: [u8,..10]) -> Option<FrameHeader> {
        if header_bytes[0] == 0x0 {
            None
        } else {
            let id = header_bytes.slice(0,4);
            
            Some(FrameHeader {
                id: str::from_utf8(id).unwrap(),
                size: util::calculate_size(header_bytes.slice(4,8)),
                flags: Flags::new(header_bytes.slice(8,10).to_vec())
            })
        }
    }
}

impl fmt::Show for FrameHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " ID: {}\n Size: {}\n Flags: {}\n", self.id, self.size, self.flags)
    }
}

pub struct Flags {
    preserve_frame_if_tag_altered: bool,
    preserve_frame_if_file_altered: bool,
    frame_read_only: bool,
    frame_compressed: bool,
    frame_encrypted: bool,
    grouped_with_other_frames: bool
}

impl Flags {
    pub fn new(flag_bytes: Vec<u8>) -> Flags {
        Flags {
            preserve_frame_if_tag_altered: (flag_bytes[0] & 0x80) == 0,
            preserve_frame_if_file_altered: (flag_bytes[0] & 0x40) == 0,
            frame_read_only: (flag_bytes[0] & 0x20) != 0,
            frame_compressed: (flag_bytes[1] & 0x80) != 0,
            frame_encrypted: (flag_bytes[1] & 0x40) != 0,
            grouped_with_other_frames: (flag_bytes[1] & 0x20) != 0
        }
    }
}

impl fmt::Show for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
    preserve_frame_if_tag_altered: {}
    preserve_frame_if_file_altered: {}
    frame_read_only: {}
    frame_compressed: {}
    frame_encrypted: {}
    grouped_with_other_frames {}",
                   self.preserve_frame_if_tag_altered, self.preserve_frame_if_file_altered, self.frame_read_only,
                   self.frame_compressed, self.frame_encrypted, self.grouped_with_other_frames)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_frame_id(){
        let flags: [u8,..10] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00];
        let frame_header = super::FrameHeader::new(flags).unwrap();

        assert_eq!("TIT2".as_slice(), frame_header.id);
    }

    #[test]
    fn test_frame_size(){
        let flags: [u8,..10] = [0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00];
        assert_eq!(10, super::FrameHeader::new(flags).unwrap().size);
    }
}
