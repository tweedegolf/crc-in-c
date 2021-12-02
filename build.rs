extern crate cc;

fn main() {
    cc::Build::new().file("crc32.c").compile("crc32.a");
}
