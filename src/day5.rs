use std::ops::RangeInclusive;

pub fn a(input: &str) -> i32 {
    let (fresh, ingredients) = input.trim().split_once("\n\n").unwrap();

    let ranges = merge_ranges(
        fresh
            .lines()
            .map(|l| {
                let (from, to) = l.split_once('-').unwrap();
                from.parse().unwrap()..=to.parse().unwrap()
            })
            .collect(),
    );

    let mut fresh_ingredients = 0;
    for ingredient in ingredients.lines() {
        let ingredient: u64 = ingredient.parse().unwrap();

        let idx = match ranges.binary_search_by_key(&ingredient, |r| *r.start()) {
            Err(0) => continue,
            Err(i) => i - 1,
            Ok(i) => i,
        };
        if *ranges[idx].end() >= ingredient {
            fresh_ingredients += 1;
        }
    }

    fresh_ingredients
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by_key(|r| *r.start());
    ranges.dedup_by(|second, first| {
        if first.end() + 1 >= *second.start() {
            *first = (*first.start())..=(*first.end()).max(*second.end());
            return true;
        }
        false
    });
    ranges
}

pub fn b(input: &str) -> u64 {
    let (fresh, _ingredients) = input.trim().split_once("\n\n").unwrap();

    let ranges = merge_ranges(
        fresh
            .lines()
            .map(|l| {
                let (from, to) = l.split_once('-').unwrap();
                from.parse().unwrap()..=to.parse().unwrap()
            })
            .collect(),
    );

    ranges.into_iter().map(|r| 1 + *r.end() - *r.start()).sum()
}

#[cfg(test)]
const TEST: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

#[test]
fn test() {
    assert_eq!(a(TEST), 3);

    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    assert_eq!(a(&input), 789);

    assert_eq!(b(TEST), 14);
}
