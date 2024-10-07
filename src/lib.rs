#![feature(portable_simd)]
#![feature(trait_alias)]

use std::{iter, ops::Range, simd::Simd};

const SIMD_MIN_COMPRESS: usize = 65536;
const SIMD_LANES: usize = 4;

const SIMD_ITER: Range<usize> = 0..SIMD_LANES;
type MySimd = Simd<i64, SIMD_LANES>;

fn compress_remainder(values: &[i64]) -> Vec<u8> {
    todo!()
}

#[inline]
fn horizontal_simd(values: &[i64], chunk_size: usize, index: usize) -> MySimd {
    let mut me_simd = MySimd::default();

    for i in SIMD_ITER {
        me_simd[i] = values[index + i * chunk_size]
    }

    return me_simd;
}

#[inline]
fn horizontal_simd_skip_first(values: &[i64], chunk_size: usize, index: isize) -> MySimd {
    let mut me_simd = MySimd::default();
    for i in 1..SIMD_LANES {
        let idx = index + (i * chunk_size) as isize;
        me_simd[i] = values[idx as usize];
    }
    return me_simd;
}

#[inline]
fn horizontal_unsimd(simdd: MySimd, unraveled: &mut [i64], simdd_size: usize, index: usize) {
    for i in SIMD_ITER {
        unraveled[index + i * simdd_size] = simdd[i];
    }
}

fn unravel_simdd<I: IntoIterator<Item = MySimd>>(
    simdd: I,
    simdd_size: usize,
    unsimded_size: usize,
) -> Vec<i64> {
    let mut unraveled = Vec::with_capacity(unsimded_size);
    unraveled.resize_with(unsimded_size, Default::default);

    for (i, x) in simdd.into_iter().enumerate() {
        horizontal_unsimd(x, &mut unraveled, simdd_size, i)
    }

    return unraveled;
}

#[inline]
fn double_delta_simd_map(
    values: &[i64],
    simdd_size: usize,
    index: usize,
    delta: &mut MySimd,
    curr: &mut MySimd,
) -> MySimd {
    let next = horizontal_simd(values, simdd_size, index);
    let next_delta_i64 = next - *curr;
    let next_delta = next_delta_i64;
    let dd = next_delta - *delta;
    *curr = next;
    *delta = next_delta;
    return dd;
}

fn double_delta_simd(values: &[i64]) -> impl Iterator<Item = MySimd> + use<'_> {
    let simdd_size = values.len() / SIMD_LANES;

    let mut curr = horizontal_simd(values, simdd_size, 0);
    let prev = horizontal_simd_skip_first(values, simdd_size, -1);
    let prev_prev = horizontal_simd_skip_first(values, simdd_size, -2);
    let mut delta = curr - prev;
    let prev_delta = prev - prev_prev;

    let first_dd = delta - prev_delta;

    delta[0] = 0;
    let range = 1..simdd_size;
    let double_deltad_bulk =
        range.map(move |i| double_delta_simd_map(values, simdd_size, i, &mut delta, &mut curr));

    let double_deltad = iter::once(first_dd).chain(double_deltad_bulk);

    return double_deltad;
}

fn rle_bitlanes_simd() {}

fn compress_values_bulk(values: &[i64]) -> Vec<u8> {
    let double_deltad = double_delta_simd(values);

    todo!();
}

pub fn compress_values(values: &[i64]) -> Vec<u8> {
    if values.len() < SIMD_MIN_COMPRESS {
        return compress_remainder(values);
    }

    let remainder_size = values.len() % SIMD_LANES;
    let bulk_size = values.len() - remainder_size;
    let bulk_values = &values[..bulk_size];
    let mut bulk_compressed = compress_values_bulk(bulk_values);
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
        let input = vec![
            8, 7, 12, 14, 18, 22, 25, 27, 32, 37, 73, 78, 83, 89, 92, 24, 23, 27, 29, 32,
        ];

        let result = double_delta_simd(input.as_slice());

        let expected = vec![
            8, -1, 6, -3, 2, 0, -1, -1, 3, 0, 31, -31, 0, 1, -3, -71, 67, 5, -2, 1,
        ];

        let simdd_size = input.len() / SIMD_LANES;
        let unsimdded_size = input.len();

        let result = unravel_simdd(result, simdd_size, unsimdded_size);
        let result_len = result.len();
        let expected_len = expected.len();

        assert_eq!(
            result_len, expected_len,
            "unraveled result is of size {result_len} expected: {expected_len}"
        );

        for i in 0..result_len {
            let result_value = result[i];
            let expected_value = expected[i];
            if result_value == expected_value {
                continue;
            }

            let prev_prev = if i < 2 { 0 } else { input[i - 2] };
            let prev = if i == 0 { 0 } else { input[i - 1] };
            let curr = input[i];
            panic!(
                "double delta expected {expected_value} at index {i}, instead got {result_value} with inputs [prev_prev, prev, curr] [{prev_prev}, {prev}, {curr}]",
            )
        }
    }
}
