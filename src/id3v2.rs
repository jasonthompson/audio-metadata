pub mod header {
    use std::fmt;
    use util;

    pub struct Header {
        major_version: u8,
        revision_number: u8,
        flags: HeaderFlags,
        tag_size: u32
    }

    impl Header {
        pub fn new(header: Vec<u8>) -> Header {
            Header { major_version: header[3], 
                     revision_number: header[4], 
                     flags: HeaderFlags::new(header[5]),
                     tag_size: util::calculate_size(header.slice(6, 10)),
            }
        }
    }

    impl fmt::Show for Header {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Version: ID3v2.{}.{}\n{}\nTag Size: {}",
                   self.major_version, self.revision_number, self.flags, self.tag_size)
        }
    }

    // Header Tags
    //
    // a - Unsynchronisation
    //
    //   Bit 7 in the 'ID3v2 flags' indicates whether or not
    //   unsynchronisation is used (see section 5 for details); a set bit
    //   indicates usage.
    //
    // b - Extended header
    //
    //   The second bit (bit 6) indicates whether or not the header is
    //   followed by an extended header. The extended header is described in
    //   section 3.2.
    //
    // c - Experimental indicator
    //
    //   The third bit (bit 5) should be used as an 'experimental
    //   indicator'. This flag should always be set when the tag is in an
    //   experimental stage.
    //
    pub struct HeaderFlags {
        unsynchronization: bool,
        extended_header: bool,
        experimental: bool
    }

    impl HeaderFlags {
        fn new(flags_byte: u8) -> HeaderFlags {
            HeaderFlags {
                unsynchronization: (flags_byte & 0x80) != 0,
                extended_header: (flags_byte & 0x40) != 0,
                experimental: (flags_byte & 0x20) != 0
            }
        }
    }

    impl fmt::Show for HeaderFlags {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Unsynchronized: {}\nExtended_header: {}\nExperimental: {}", 
                   self.unsynchronization, self.extended_header, self.experimental)
        }
    }
}

pub mod frame {
    use std::string;

    use util;
    // 3.3.   ID3v2 frame overview
    //
    //    As the tag consists of a tag header and a tag body with one or more
    //    frames, all the frames consists of a frame header followed by one or
    //    more fields containing the actual information. The layout of the
    //    frame header:
    //
    //      Frame ID   $xx xx xx xx  (four characters)
    //      Size       $xx xx xx xx
    //      Flags      $xx xx
    //
    struct Frame {
        id: String,
        size: u32,
        flags: Flags
    }
    
    impl Frame {
        pub fn new(header_bytes: Vec<u8>) -> Frame {
            Frame {
                id: String::from_utf8(header_bytes.slice(0,4).to_vec()).unwrap(),
                size: util::calculate_size(header_bytes.slice(4,8)),
                flags: Flags::new(header_bytes.slice(8,10))
            }
        }                ///  3.3.1 Frame Header Flags

    }
    
    struct Flags {
        preserve_frame_if_tag_altered: bool,
        preserve_frame_if_file_altered: bool,
        read_only: bool,
        compression: bool,
        encryption: bool,
        group_identity: bool
    }

    impl Flags {
        pub fn new(flag_bytes: Vec<u8>) -> Flags {
            Flags {

                /// a. This flag indicates whether or not this frame belongs in a group
                ///    with other frames. If set a group identifier byte is added to the
                ///    frame header. Every frame with the same group identifier belongs
                ///    to the same group.
                ///
                preserve_frame_if_tag_altered: (flag_bytes[0] & 0x80) == 0,
                
                /// b.  This flag tells the software what to do with this frame if it is
                ///     unknown and the file, excluding the tag, is altered. This does not
                ///     apply when the audio is completely replaced with other audio data.
                ///
                preserve_frame_if_file_alterd: (flag_bytes[0] & 0x40) == 0,
                
                /// c.  This flag, if set, tells the software that the contents of this
                ///     frame is intended to be read only. Changing the contents might
                ///     break something, e.g. a signature. If the contents are changed,
                ///     without knowledge in why the frame was flagged read only and
                ///     without taking the proper means to compensate, e.g. recalculating
                ///     the signature, the bit should be cleared.
                ///
                frame_read_only: (flag_bytes[0] & 0x20) != 0,
                
                /// i. This flag indicates whether or not the frame is compressed. If so,
                ///    frame is compressed using zlib with 4 bytes for 'decompressed size' 
                ///    appended to the frame header.
                ///
                frame_compressed: (flag_bytes[1] & 0x80) != 0,
                
                /// j. This flag indicates wether or not the frame is enrypted. If set
                ///    one byte indicating with which method it was encrypted will be
                ///    appended to the frame header. See section 4.26. for more
                ///    information about encryption method registration.
                ///
                frame_encrypted: (flag_bytes[1] & 0x40) != 0,
                
                /// k. This flag indicates whether or not this frame belongs in a group
                ///    with other frames. If set a group identifier byte is added to the
                ///    frame header. Every frame with the same group identifier belongs
                ///    to the same group.
                ///
                grouped_with_other_frames: (flag_bytes[1] & 0x20) != 0
            }
        }
    }
}

mod test {
    #[test]
    fn flags_new_translates_bytes_to_text(){
        
}
