pub fn a(input: &str) -> u64 {
    let mut invalid_numbers = 0;
    for range in input.split(',') {
        let (start, end) = range.trim().split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        for n in start..=end {
            if is_invalid_a(n) {
                invalid_numbers += n;
            }
        }
    }
    invalid_numbers
}

fn is_invalid_a(n: u64) -> bool {
    let num_digits = n.ilog10() + 1;
    if !num_digits.is_multiple_of(2) {
        return false;
    }

    let exponent = 10u64.pow(num_digits / 2);

    let first_half = n / exponent;
    let second_half = n - first_half * exponent;

    first_half == second_half
}

#[test]
fn test_a() {
    let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;
    assert_eq!(a(input), 1227775554);
}

pub fn b(input: &str) -> u64 {
    let mut buf = Vec::new();
    let mut invalid_numbers = 0;
    for range in input.split(',') {
        let (start, end) = range.trim().split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();

        for n in start..=end {
            if is_invalid_b(&mut buf, n) {
                invalid_numbers += n;
            }
        }
    }
    invalid_numbers
}

fn is_invalid_b(buf: &mut Vec<u64>, n: u64) -> bool {
    let num_digits = n.ilog10() + 1;

    'div: for divisor in 1..=(num_digits / 2) {
        if !num_digits.is_multiple_of(divisor) {
            continue;
        }
        buf.clear();
        let mut n = n;
        for e_inv in 0..(num_digits / divisor) - 1 {
            let exponent = 10u64.pow(num_digits - divisor * (1 + e_inv));
            let part = n / exponent;
            n -= part * exponent;
            buf.push(part);
        }
        buf.push(n);

        for s in buf.windows(2) {
            if s[0] != s[1] {
                continue 'div;
            }
        }
        return true;
    }
    false
}

#[test]
fn test_b() {
    let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;
    assert_eq!(b(input), 4174379265);
}
