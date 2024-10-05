#![feature(portable_simd)]
#![feature(slice_as_chunks)]

use std::simd::f32x4;

const SIMD_MIN_COMPRESS: usize = 65536;

pub fn compress_remainder(values: &[f32]) -> Vec<u8> {
    todo!()
}

pub fn double_delta_simd(values: &[f32]) -> Vec<f32x4> {
    let simdd_size = values.len() / 4;
    let mut double_deltad = Vec::<f32x4>::with_capacity(simdd_size);

    let mut curr = f32x4::from_array([
        values[0],
        values[simdd_size],
        values[simdd_size * 2],
        values[simdd_size * 3],
    ]);
    let prev = f32x4::from_array([
        0.0,
        values[simdd_size - 1],
        values[simdd_size * 2 - 1],
        values[simdd_size * 3 - 1],
    ]);
    let prev_prev = f32x4::from_array([
        0.0,
        values[simdd_size - 2],
        values[simdd_size * 2 - 2],
        values[simdd_size * 3 - 2],
    ]);
    let mut delta = curr - prev;
    let prev_delta = prev - prev_prev;

    double_deltad.push(delta - prev_delta);

    delta[0] = 0.0;

    for i in 1..simdd_size {
        let next = f32x4::from_array([
            values[i],
            values[i + simdd_size],
            values[i + simdd_size * 2],
            values[i + simdd_size * 3],
        ]);
        let next_delta = next - curr;
        let dd = next_delta - delta;
        double_deltad.push(dd);
        curr = next;
        delta = next_delta;
    }
    return double_deltad;
}

pub fn compress_bulk(values: &[f32], total_size: usize) -> Vec<u8> {
    let result = Vec::<u8>::with_capacity(total_size);
    let double_deltad = double_delta_simd(values);
    return result;
}

pub fn compress(values: &[f32]) -> Vec<u8> {
    if values.len() < SIMD_MIN_COMPRESS {
        return compress_remainder(values);
    }

    let remainder_size = values.len() % 4;
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
    fn test_double_delta() {
        #[rustfmt::skip]
        let input = vec![
            8.0, 7.0, 12.0, 14.0, 18.0, 
            22.0, 25.0, 27.0, 32.0, 37.0,
            73.0, 78.0, 83.0, 89.0, 92.0, 
            24.0, 23.0, 27.0, 29.0, 32.0,
        ];

        let result = double_delta_simd(input.as_slice());
        let expected = vec![
            f32x4::from_array([8.0, 0.0, 31.0, -71.0]),
            f32x4::from_array([-1.0, -1.0, -31.0, 67.0]),
            f32x4::from_array([6.0, -1.0, 0.0, 5.0]),
            f32x4::from_array([-3.0, 3.0, 1.0, -2.0]),
            f32x4::from_array([2.0, 0.0, -3.0, 1.0]),
        ];
        assert_eq!(result.len(), expected.len());
        for i in 0..result.len() {
            assert_eq!(result[i], expected[i])
        }
    }
}
