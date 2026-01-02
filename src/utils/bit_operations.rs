/// Removes and returns the index of the least significant set bit.
pub(crate) fn pop_1st_bit(bitboard: &mut u64) -> u32 {
    let pos = bitboard.trailing_zeros();
    *bitboard &= *bitboard - 1_u64;  // Remove the rightmost bit
    pos
}

/// Transforms a bitboard into an index using a magic number.
pub(crate) fn transform(bitboard: u64, magic: u64, bits: i32) -> i32 {
    ((bitboard * magic) >> (64 - bits)) as i32
}

/// Converts an index into a bitboard given a mask and number of bits.
pub(crate) fn index_to_bitboard(index: i32, bits: u32, mut m: u64) -> u64 {
    let mut result = 0u64;
    let mut j;
    for i in 0..bits {
        j = pop_1st_bit(&mut m);
        if (index & (1 << i)) != 0 {
            result |= 1u64 << j;
        }
    }
    result
}
