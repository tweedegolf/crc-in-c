extern "C" {
    fn CRC32(data: *const u8, data_length: usize) -> u32;
}

fn crc32(data: &[u8]) -> u32 {
    unsafe { CRC32(data.as_ptr(), data.len()) }
}

fn main() {
    println!("{}", crc32(b"12345678"));
}
