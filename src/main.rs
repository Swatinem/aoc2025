use std::fs::read_to_string;

mod day1;

fn main() {
    let input = read_to_string("inputs/day1.txt").unwrap();
    dbg!(day1::a(&input));
    dbg!(day1::b(&input));
}
