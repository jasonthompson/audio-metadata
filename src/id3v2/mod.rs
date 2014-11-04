pub mod header;
pub mod frame;
pub mod parser;

fn is_id3v2(header_info: &Vec<u8>) -> bool {
    header_info[0] as char == 'I' && header_info[1] as char == 'D' && header_info[2] as char == '3'
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
