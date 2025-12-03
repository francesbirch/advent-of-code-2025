use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_line_total(x: &[u8], n_dgt: &u32) -> u64 {
    let mut line_total: u64 = 0;
    let mut search_lb = 0;
    // There are `n_dgt` digits that will be make up the row's final integer. The 0th digit
    // corresponds to the greatest power of 10, the 1st digit corresponds to the second greatest
    // power of ten, and so on. The values selected for each of the `n_dgts` has only one
    // constraint: for all pairs (x[i], x[j]) selected for decimal places (k, l), respectively,
    // it must hold that `k < l => i < j`;
    // Let:
    //     - `n` represent the total number of digits in the row.
    //     - `m` represent the number of digits (not zero-indexed)
    //     - `i_k, 0 <= k < m` represent the index selected for the k^th (zero-indexed) digit of
    //     the final integer.
    //     - `d_k =  x[i_k]` represent the value selected for digit `k`
    // to be combined to form the single integer.
    //
    // The value of digit `k`, `0 <= k < m` that maximizes the value of the row's sum is the greatest value in
    // the slice x[(i_(k-1) + 1)..(n - m + k + 1)]. In other words, it is the greatest value
    // whose index is greater than the index of digit `k-1` and less than the total row width minus
    // the remaining number of digits to be selected.
    for i in 0..*n_dgt {
        let x = &x[search_lb..];
        // Find the upper bound of the slice that will be searched for the maximum value.
        let ub = x.len() - (n_dgt - i - 1) as usize;
        let mut max_val = 0;
        let mut max_idx = 0;
        for (j, v) in x[..ub].iter().enumerate() {
            // These are still bytes so we need to convert them
            let v = *v - b'0';
            if v > max_val {
                max_val = v;
                max_idx = j;
            }
        }
        let dig_total = (max_val as u64) * 10u64.pow(n_dgt - i - 1);
        line_total += dig_total;
        search_lb += max_idx + 1;
    }
    line_total
}

fn run_both_parts() -> (u64, u64) {
    let t = read_and_split_lines("./data/day03.txt").unwrap();
    let mut total_part1: u64 = 0;
    let mut total_part2: u64 = 0;
    for row in t {
        let row = row.unwrap();
        total_part1 += get_line_total(&row, &2);
        total_part2 += get_line_total(&row, &12);
    }
    (total_part1, total_part2)
}

fn read_and_split_lines<P>(filename: P) -> io::Result<io::Split<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).split(b'\n'))
}

pub fn day03() {
    let (p1, p2) = run_both_parts();
    println!("Part 1: {p1}, Part 2: {p2}");
}
