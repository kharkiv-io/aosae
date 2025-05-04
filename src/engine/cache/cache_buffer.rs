use lz4_flex::block::{compress_prepend_size, decompress_size_prepended};
use std::fs::File;
use std::io::{Read, Write};

pub fn u8_to_f32v(v: &[u8]) -> Vec<f32> {
    v.chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(f32::from_le_bytes)
        .collect()
}

fn vf32_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub fn read_cache_buffer_decomp(path: &str) -> Vec<f32> {
    let file = File::open(path);
    let mut buffer = Vec::new();
    file.unwrap().read_to_end(&mut buffer).unwrap();
    let decoder = decompress_size_prepended(buffer.as_slice());
    u8_to_f32v(decoder.unwrap().as_slice())
}

pub fn write_cache_buffer_comp(path: &str, samples: Vec<f32>) {
    let file = File::create(path);
    let encoder = compress_prepend_size(vf32_to_u8(&samples));
    file.unwrap().write_all(encoder.as_slice()).unwrap();
    // println!("Compressed -> {:?}", encoder.as_slice());
}
