#![feature(portable_simd)]
#![feature(slice_as_chunks)]

use std::simd::Simd;

const SIMD_LANES: usize = 4;
const SIMD_MIN_COMPRESS: usize = 65536;

pub fn compress_remainder(values: &[f32]) -> Vec<u8> {
    todo!()
}

pub fn compress_bulk(values: &[f32], total_size: usize) -> Vec<u8> {
    let result = Vec::<u8>::with_capacity(total_size);
    return result;
}

pub fn compress(values: &[f32]) -> Vec<u8> {
    if values.len() < SIMD_MIN_COMPRESS {
        return compress_remainder(values);
    }

    let remainder_size = values.len() % SIMD_LANES;
    let bulk_size = values.len() - remainder_size;
    let bulk_values = &values[..bulk_size];
    let mut bulk_compressed = compress_bulk(bulk_values, values.len());
    if remainder_size == 0 {
        return bulk_compressed;
    }
    let remainder_values = &values[bulk_size..];
    let remainder_compressed = compress_remainder(remainder_values);
    bulk_compressed.extend(remainder_compressed);
    return bulk_compressed;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, 4);
    }
}
