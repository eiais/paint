extern crate lzma_rs;

use std::io;

pub fn main() {
    lzma_rs::lzma_compress(&mut &[0u8, 1u8][..], &mut io::stdout());
}
