pub fn a(input: &str) -> u64 {
    let mut lanes = vec![];
    let mut next_lanes = vec![];

    let mut splits = 0;
    for line in input.trim().lines() {
        let mut lanes_it = lanes.iter().peekable();
        for (idx, ch) in line.chars().enumerate() {
            match ch {
                'S' => next_lanes.push(idx),
                '^' => {
                    if lanes_it.next_if_eq(&&idx).is_some() {
                        splits += 1;
                        next_lanes.extend_from_slice(&[idx - 1, idx + 1]);
                    }
                }
                _ => {
                    if lanes_it.next_if_eq(&&idx).is_some() {
                        next_lanes.push(idx);
                    }
                }
            }
        }

        std::mem::swap(&mut lanes, &mut next_lanes);
        next_lanes.clear();
        lanes.sort_unstable();
        lanes.dedup();
    }
    splits
}

pub fn b(input: &str) -> u64 {
    let mut lanes = vec![];
    let mut next_lanes = vec![];

    for line in input.trim().lines() {
        let mut lanes_it = lanes.iter().peekable();
        for (idx, ch) in line.chars().enumerate() {
            match ch {
                'S' => next_lanes.push((idx, 1)),
                '^' => {
                    if let Some((_, n)) = lanes_it.next_if(|(i, _)| *i == idx) {
                        next_lanes.extend_from_slice(&[(idx - 1, *n), (idx + 1, *n)]);
                    }
                }
                _ => {
                    if let Some((_, n)) = lanes_it.next_if(|(i, _)| *i == idx) {
                        next_lanes.push((idx, *n));
                    }
                }
            }
        }

        std::mem::swap(&mut lanes, &mut next_lanes);
        next_lanes.clear();
        lanes.sort_unstable();
        lanes.dedup_by(|second, first| {
            if first.0 == second.0 {
                first.1 += second.1;
                return true;
            }
            false
        });
    }
    lanes.into_iter().map(|(_, n)| n).sum()
}

#[cfg(test)]
const TEST: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

#[test]
fn test() {
    assert_eq!(a(TEST), 21);
    assert_eq!(b(TEST), 40);
}
