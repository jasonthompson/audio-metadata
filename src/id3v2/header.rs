use std::fmt;
use util;

pub struct Header {
    pub major_version: u8,
    pub revision_number: u8,
    pub flags: HeaderFlags,
    pub tag_length: uint
}

impl Header {
    pub fn new(header: &Vec<u8>) -> Header {
        Header { major_version: header[3], 
                 revision_number: header[4], 
                 flags: HeaderFlags::new(header[5]),
                 tag_length: util::calculate_size(header.slice(6, 10)),
        }
    }
}

impl fmt::Show for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Version: ID3v2.{}.{}\n{}\nTag Length: {}",
               self.major_version, self.revision_number, self.flags, self.tag_length)
    }
}


pub struct HeaderFlags {
    
    /// Bit 7 in the 'ID3v2 flags' indicates whether or not
    /// unsynchronisation is used (see section 5 for details); a set bit
    /// indicates usage.
    ///
    unsynchronization: bool,

    /// The second bit (bit 6) indicates whether or not the header is
    /// followed by an extended header. The extended header is described in
    /// section 3.2.
    ///
    extended_header: bool,

    /// The third bit (bit 5) should be used as an 'experimental
    /// indicator'. This flag should always be set when the tag is in an
    /// experimental stage.
    ///
    
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

#[cfg(test)]
mod test {
    #[test]
    fn test_header_initialization() {
        let header_vec = vec![0x49, 0x44, 0x33, 0x03, 0x00, 0x00, 0x00, 0x00, 0x08, 0x09];
        let header = super::Header::new(&header_vec);
        assert_eq!(header.major_version, 3);
        assert_eq!(header.tag_length, 1033);
    }
}

