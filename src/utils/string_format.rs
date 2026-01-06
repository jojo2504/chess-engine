/// Display a 64-character string as a chessboard.
/// LSB (index 0) = a1, MSB (index 63) = h8
pub fn display_bitstring_as_chessboard(s: &str) {
    assert!(
        s.len() == 64,
        "Input string must be exactly 64 characters"
    );

    let bytes = s.as_bytes();

    // ranks 1 to 8 (bottom to top)
    for rank in 0..8 {
        let start = rank * 8;
        let end = start + 8;

        // reverse each rank (a..h)
        let line: String = bytes[start..end]
            .iter()
            .rev()
            .map(|&b| b as char)
            .collect();

        println!("{}", line);
    }
}
