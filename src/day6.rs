pub fn a(input: &str) -> u64 {
    let (lines, last_line) = input.trim().rsplit_once('\n').unwrap();

    let mut lines: Vec<Vec<u64>> = lines
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut sum = 0;
    let mut nums = vec![];
    for op in last_line.split_whitespace().rev() {
        nums.clear();
        for line in &mut lines {
            nums.push(line.pop().unwrap());
        }

        let res: u64 = match op {
            "*" => nums.iter().product(),
            "+" => nums.iter().sum(),
            _ => panic!("unknown op"),
        };
        sum += res;
    }
    sum
}

pub fn b(input: &str) -> u64 {
    let (lines, last_line) = input
        .trim_start_matches('\n')
        .trim_end_matches('\n')
        .rsplit_once('\n')
        .unwrap();

    let mut lines: Vec<&str> = lines.lines().collect();

    let mut sum = 0;
    let mut str_column = vec![];
    let mut num_column = vec![];

    let mut last_line = last_line.chars().peekable();
    while let Some(op) = last_line.next() {
        let mut num_chars = 0;
        while last_line.next_if_eq(&' ').is_some() {
            num_chars += 1;
        }
        if last_line.peek().is_none() {
            num_chars += 1;
        }

        str_column.clear();
        num_column.clear();
        for line in &mut lines {
            let (n, rest) = line.split_at(num_chars);
            *line = rest.get(1..).unwrap_or_default();
            str_column.push(n);
        }

        for s in &str_column {
            num_column.resize(s.len(), 0);
            for (ch, num) in s.chars().zip(&mut num_column) {
                if ch != ' ' {
                    *num *= 10;
                    *num += ch.to_digit(10).unwrap() as u64;
                }
            }
        }
        // dbg!(&str_column, &num_column);

        let res: u64 = match op {
            '*' => num_column.iter().product(),
            '+' => num_column.iter().sum(),
            op => panic!("unknown op '{op}'"),
        };
        sum += res;
    }
    sum
}

#[cfg(test)]
const TEST: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

#[test]
fn test() {
    assert_eq!(a(TEST), 4277556);
    assert_eq!(b(TEST), 3263827);
}
