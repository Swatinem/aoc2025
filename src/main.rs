use std::fs::read_to_string;

// mod day1;
// mod day2;

mod day3;
mod day4;

fn main() {
    // let input = read_to_string("inputs/day1.txt").unwrap();
    // dbg!(day1::a(&input));
    // dbg!(day1::b(&input));

    // let input = read_to_string("inputs/day2.txt").unwrap();
    // dbg!(day2::a(&input));
    // dbg!(day2::b(&input));

    let input = read_to_string("inputs/day3.txt").unwrap();
    dbg!(day3::a(&input));
    dbg!(day3::b(&input));

    let input = read_to_string("inputs/day4.txt").unwrap();
    dbg!(day4::a(&input));
    dbg!(day4::b(&input));
}
