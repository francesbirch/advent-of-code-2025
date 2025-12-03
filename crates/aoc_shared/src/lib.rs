pub fn byte_slice_to_u32(x: &[u8]) -> u64 {
    x.iter()
        .filter_map(|v| -> Option<u64> {
            // if there's a newline at the end of the file the program will just skip it.
            // Checking for CR as well in case the input data was saved with CRLF line endings
            if (*v == b'\n') || (*v == b'\r') {
                None
            } else if !v.is_ascii_digit() {
                panic!(
                    "You messed up somewhere... Non-ASCII digit: {:?}",
                    std::str::from_utf8(&[*v])
                )
            } else {
                // offset from the 8-bit representation of the ASCII 0 gives the unsigned integer
                // value of any ASCII digit. Because I've already checked that it is an ascii digit
                // I don't need to worry about the case where it is outside the 48-57 decimal range
                Some((v - b'0') as u64)
            }
        })
        // for each additional digit folded, the accumulated digits must be multiplied by ten to
        // reflect the fact that there is an additional power of ten to the right.
        // Using checked_mul and checked_add because I do not want overflow to go unhandled
        .try_fold(0u64, |cml, v| cml.checked_mul(10)?.checked_add(v))
        .unwrap()
}
