mod day01;

const ALL_DAYS: [&str; 1] = ["day01"];

pub fn run_one_or_all(d: Option<&str>) {
    match d {
        Some(v) => {
            run_single_problem(v);
        }
        None => {
            for day in ALL_DAYS.iter() {
                print!("{day}:  ");
                run_single_problem(day);
            }
        }
    }
}
fn run_single_problem(d: &str) {
    match d {
        "day01" => {
            day01::day01();
        }
        _ => panic!("idk what this is but it isn't a day I've finished..."),
    }
}
