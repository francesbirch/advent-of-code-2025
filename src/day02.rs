use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;
use std::str::Utf8Error;

use aoc_shared::byte_slice_to_u32;

fn read_and_split_at_commas<P>(filename: P) -> io::Result<io::Split<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).split(b','))
}

fn split_pair(s: &[u8]) -> std::result::Result<(&[u8], &[u8]), Utf8Error> {
    let mut v = s.split(|c| *c == b'-');
    Ok((v.next().unwrap(), v.next().unwrap()))
}

pub fn day02() {
    let x = read_and_split_at_commas("./data/day02.txt").unwrap();
    let mut total_part1: u64 = 0;
    let mut total_part2: u64 = 0;

    for r in x {
        let r = r.unwrap();
        let (sb, eb) = split_pair(&r[..]).unwrap();
        // using slices because they are easy to index and it's safe to assume the input will
        // only be ASCII characters in this case so no characters will be split by doing this
        let (sn, en) = (byte_slice_to_u32(sb), byte_slice_to_u32(eb));
        for digit in sn..=en {
            // converting to bytes to make indexing easier
            let ds = digit.to_string();
            let s: &[u8] = ds.as_bytes();
            if satisfies_part_one_criteria(s) {
                total_part1 += digit;
            }
            if satisfies_part_two_criteria(s) {
                total_part2 += digit;
            }
        }
    }
    println!("Part 1: {total_part1}, Part 2: {total_part2}");
}

fn is_repeated_pattern(x: &[u8], pat: &[u8]) -> bool {
    // if a candidate pattern is not a multiple of the number's length,
    // then there is no way for any number of its repetitions to be
    // equal to the number.
    if !x.len().is_multiple_of(pat.len()) {
        return false;
    }
    // if the pattern is equal to the number when repeated enough times to be the
    // same length as it, then the number must be an invalid ID
    let req_reps = x.len() / pat.len();
    pat.repeat(req_reps) == x
}

fn satisfies_part_two_criteria(b: &[u8]) -> bool {
    // window whose start is fixed at 0 and expands by one until reaching half
    // the width of the number. There's no point in checking beyond this because
    // the pattern must repeat at least twice in the number.
    for i in 0..(b.len() / 2) {
        if !is_repeated_pattern(b, &b[..=i]) {
            continue;
        }
        return true;
    }
    false
}

fn satisfies_part_one_criteria(b: &[u8]) -> bool {
    let lngth = b.len();
    // only even-length numbers can be invalid IDs based
    // on the criteria given
    if !lngth.is_multiple_of(2) {
        return false;
    }
    // zip the left and right side of the slice together to compare the pairs directly
    // any inequality violates the criteria for an invalid ID, so we can return false as
    // soon as this condition is encountered. Finishing the for loop without returning false
    // means that the left and right sides were equal and the ID is invalid
    for (left, right) in zip(&b[..(lngth / 2)], &b[(lngth / 2)..]) {
        if left != right {
            return false;
        }
    }
    true
}
