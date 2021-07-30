extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::io;

#[wasm_bindgen]
pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut input = data;
    let mut output = io::Cursor::new(vec![]);
    lzma_rs::lzma_compress_with_options(&mut input, &mut output,
                                        &lzma_rs::compress::Options{
                                            unpacked_size: lzma_rs::compress::UnpackedSize::WriteToHeader(
                                                               Some(data.len() as u64))
                                        });
    output.into_inner()
}

