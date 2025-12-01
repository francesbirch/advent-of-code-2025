use advent_of_code_2025::run_one_or_all;

fn main() {
    let mut args = std::env::args();
    // skip first element because it is the binary's path
    args.next();
    run_one_or_all(args.next().as_deref());
}
