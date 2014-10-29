pub fn calculate_size(size_bytes: &[u8]) -> u32 {
    ((size_bytes[3] as u32 & 0xFF) | (size_bytes[2] & 0xFF) as u32 << 7 | 
     (size_bytes[1] as u32 & 0xFF) << 14 | (size_bytes[0] & 0xFF) as u32 << 21)
}
