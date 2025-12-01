use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Split<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).split(b'\n'))
}

pub fn day01() {
    let lines = read_lines("./data/day01.txt").unwrap();
    let mut cnt: u32 = 0;
    let mut curr: i32 = 50;
    let mut tot_passes: i32 = 0;
    for x in lines {
        let x = x.unwrap();
        let s = if x[0] == b'R' { 1 } else { -1 };
        let n: i32 = s * std::str::from_utf8(&x[1..])
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let c = count_zero_passes(curr, n);
        tot_passes += c.abs();
        curr += n;
        if curr % 100 == 0 {
            cnt += 1;
        }
    }
    println!("Part 1: {}, Part 2: {}", cnt, tot_passes);
}

fn count_zero_passes(start: i32, chg: i32) -> i32 {
    // if the starting position is at 0 or the rotation is a whole rotation,
    // then the number of times we pass zero is equal to the number of whole rotations
    if (start % 100 == 0) || (chg % 100 == 0) {
        return (chg / 100).abs();
    }
    // express negative starting position as a positive integer
    let start = if start > 0 {
        start % 100
    } else {
        100 + start % 100
    };
    let (c, r) = if chg > 0 {
        (start, chg)
    } else {
        // convert the counter clockwise case into an equivalent clockwise one
        (100 - start, chg.abs())
    };
    r / 100 + (c + r % 100) / 100
}
