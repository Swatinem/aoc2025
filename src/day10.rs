// <https://matklad.github.io/2021/11/07/generate-all-the-things.html>

struct Gen {
    started: bool,
    v: Vec<(u32, u32)>,
    p: usize,
}

impl Gen {
    fn new() -> Gen {
        Gen {
            started: false,
            v: Vec::new(),
            p: 0,
        }
    }
    fn done(&mut self) -> bool {
        if !self.started {
            self.started = true;
            return false;
        }

        for i in (0..self.v.len()).rev() {
            if self.v[i].0 < self.v[i].1 {
                self.v[i].0 += 1;
                self.v.truncate(i + 1);
                self.p = 0;
                return false;
            }
        }

        true
    }
    fn get(&mut self, bound: u32) -> u32 {
        if self.p == self.v.len() {
            self.v.push((0, 0));
        }
        self.p += 1;
        self.v[self.p - 1].1 = bound;
        self.v[self.p - 1].0
    }
}

#[derive(Debug)]
struct Manual {
    indicators: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}
fn parse_input(input: &str) -> Manual {
    let (expected_mapping, buttons) = input.trim_start_matches('[').split_once("] (").unwrap();
    let (buttons, joltage) = buttons.split_once(") {").unwrap();
    let joltage = joltage.trim_end_matches('}');

    let expected_mapping = expected_mapping
        .char_indices()
        .fold(0, |m, (idx, ch)| m | ((ch == '#') as u32) << idx);

    let buttons = buttons
        .split(") (")
        .map(|b| {
            b.split(',')
                .fold(0, |b, n| b | 1 << n.parse::<usize>().unwrap())
        })
        .collect();

    let joltage = joltage.split(',').map(|j| j.parse().unwrap()).collect();

    Manual {
        indicators: expected_mapping,
        buttons,
        joltage,
    }
}

pub fn a(input: &str) -> u32 {
    let mut sum = 0;

    for manual in input.trim().lines() {
        let manual = parse_input(manual);
        // dbg!(manual);

        let mut fewest = u32::MAX;

        let mut g = Gen::new();
        while !g.done() {
            let num_presses = g.get(manual.buttons.len() as u32);
            if num_presses > fewest {
                continue;
            }

            let mut indicators = 0;
            let mut buttons = manual.buttons.clone();

            for _ in 0..num_presses {
                let idx = g.get(buttons.len() as u32 - 1);
                let wiring = buttons.remove(idx as usize);

                indicators ^= wiring;
            }
            if indicators == manual.indicators {
                fewest = fewest.min(num_presses);
            }
        }
        sum += fewest;
    }

    sum
}

pub fn b(input: &str) -> u64 {
    let mut sum = 0;

    let mut joltage = vec![];
    for manual in input.trim().lines() {
        let manual = parse_input(manual);

        let max_per_button: Vec<_> = manual
            .buttons
            .iter()
            .map(|wiring| {
                manual
                    .joltage
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, joltage)| (wiring & (1 << idx) > 0).then_some(*joltage))
                    .min()
                    .unwrap()
            })
            .collect();
        dbg!(&manual, &max_per_button);

        let mut fewest = max_per_button.iter().sum();

        let mut g = Gen::new();
        'permute: while !g.done() {
            joltage.clear();
            joltage.resize(manual.joltage.len(), 0);

            let button_presses = max_per_button.iter().map(|max| g.get(*max));

            let mut pressed_buttons = 0;
            for (presses, wiring) in button_presses.zip(manual.buttons.iter()) {
                if presses > 0 {
                    pressed_buttons += presses;
                    if pressed_buttons > fewest {
                        continue 'permute;
                    }
                    for idx in 0..manual.joltage.len() {
                        if wiring & (1 << idx) > 0 {
                            joltage[idx] += presses;

                            if joltage[idx] > manual.joltage[idx] {
                                continue 'permute;
                            }
                        }
                    }
                }
            }
            if joltage == manual.joltage {
                fewest = fewest.min(pressed_buttons);
            }
        }
        dbg!(fewest);
        sum += fewest as u64;
    }

    sum
}

#[cfg(test)]
const TEST: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

#[test]
fn test() {
    assert_eq!(a(TEST), 7);
    assert_eq!(b(TEST), 33);
}
