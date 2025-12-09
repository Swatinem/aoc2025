use std::fs::read_to_string;

// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
mod day9;

fn main() {
    // let input = read_to_string("inputs/day1.txt").unwrap();
    // dbg!(day1::a(&input));
    // dbg!(day1::b(&input));

    // let input = read_to_string("inputs/day2.txt").unwrap();
    // dbg!(day2::a(&input));
    // dbg!(day2::b(&input));

    // let input = read_to_string("inputs/day3.txt").unwrap();
    // dbg!(day3::a(&input));
    // dbg!(day3::b(&input));

    // let input = read_to_string("inputs/day4.txt").unwrap();
    // dbg!(day4::a(&input));
    // dbg!(day4::b(&input));

    // let input = read_to_string("inputs/day5.txt").unwrap();
    // dbg!(day5::a(&input));
    // dbg!(day5::b(&input));

    // let input = read_to_string("inputs/day6.txt").unwrap();
    // dbg!(day6::a(&input));
    // dbg!(day6::b(&input));

    // let input = read_to_string("inputs/day7.txt").unwrap();
    // dbg!(day7::a(&input));
    // dbg!(day7::b(&input));

    // let input = read_to_string("inputs/day9.txt").unwrap();
    // dbg!(day8::a(&input, 1000));
    // dbg!(day8::b(&input));

    let input = read_to_string("inputs/day9.txt").unwrap();
    dbg!(day9::a(&input));
    dbg!(day9::b(&input));
}
