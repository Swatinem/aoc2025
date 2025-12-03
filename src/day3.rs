pub fn a(input: &str) -> u64 {
    let mut sum = 0;

    let mut digits = vec![];
    for row in input.trim().lines() {
        digits.clear();
        for c in row.chars() {
            let digit = c.to_digit(10).unwrap();
            digits.push(digit);
        }

        sum += find_largest(&digits, 2);
    }
    sum
}

fn find_largest(mut digits: &[u32], n: usize) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        let end_idx = digits.len().saturating_sub(n - 1 - i);

        let largest = (&digits[..end_idx]).iter().max().unwrap();
        let largest_idx = digits.iter().position(|n| n == largest).unwrap();

        digits = &digits[largest_idx + 1..];

        sum *= 10;
        sum += *largest as u64;
    }

    sum
}

#[test]
fn test_a() {
    let input = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;
    assert_eq!(a(input), 357);
}

pub fn b(input: &str) -> u64 {
    let mut sum = 0;

    let mut digits = vec![];
    for row in input.trim().lines() {
        digits.clear();
        for c in row.chars() {
            let digit = c.to_digit(10).unwrap();
            digits.push(digit);
        }

        sum += find_largest(&digits, 12);
    }
    sum
}

#[test]
fn test_b() {
    let input = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;
    assert_eq!(b(input), 3121910778619);
}
