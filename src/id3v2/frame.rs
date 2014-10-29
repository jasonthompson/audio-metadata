use std::string;
use std::fmt;

use util;

pub struct Frame {
    header: FrameHeader,
    contents: String
}

impl Frame {
    pub fn new(header: FrameHeader, contents_bytes: Vec<u8>) -> Frame {
        Frame {
            header: header,
            contents: String::from_utf8(contents_bytes).unwrap()
        }
    }
}

impl fmt::Show for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} \n", self.header.id, self.contents)
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

    pub id: String,
    pub size: u32,
    pub flags: Flags
}

impl FrameHeader {
    pub fn new(header_bytes: Vec<u8>) -> FrameHeader {
        FrameHeader {
            id: String::from_utf8(header_bytes.slice(0,4).to_vec()).unwrap(),
            size: util::calculate_size(header_bytes.slice(4,8)),
            flags: Flags::new(header_bytes.slice(8,10).to_vec())
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
        let flag_vec = vec![0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00];
        assert_eq!("TIT2".to_string(), super::FrameHeader::new(flag_vec).id);
    }

    #[test]
    fn test_frame_size(){
        let flag_vec = vec![0x54, 0x49, 0x54, 0x32, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x00];
        assert_eq!(10, super::FrameHeader::new(flag_vec).size);
    }
}
