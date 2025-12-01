pub fn a(input: &str) -> i32 {
    const MOD: i32 = 100;

    let mut position = 50;
    let mut hit_zero = 0;

    for line in input.trim().lines() {
        if let Some(n) = line.strip_prefix('R') {
            let n: i32 = n.parse().unwrap();
            position += n;
            position = position % MOD;
        } else if let Some(n) = line.strip_prefix('L') {
            let n: i32 = n.parse().unwrap();
            position -= n;
            if position < 0 {
                position = (MOD - -position) % MOD;
            }
        }

        if position == 0 {
            hit_zero += 1;
        }
    }

    hit_zero
}

#[test]
fn test_a() {
    let input = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;
    assert_eq!(a(input), 3);
}

pub fn b(input: &str) -> i32 {
    const MOD: i32 = 100;

    let mut position = 50;
    let mut hit_zero = 0;

    for line in input.trim().lines() {
        if let Some(n) = line.strip_prefix('R') {
            let n: i32 = n.parse().unwrap();

            position += n;
            hit_zero += position / MOD;
            position = position % MOD;
        } else if let Some(n) = line.strip_prefix('L') {
            let mut n: i32 = n.parse().unwrap();

            hit_zero += n / MOD;
            n = n % MOD;

            if position == 0 {
                hit_zero -= 1;
            }
            position -= n;
            hit_zero += position / MOD;
            position = position % MOD;

            hit_zero += (position <= 0) as i32;
            if position < 0 {
                position = position + MOD;
            }
        }
    }

    hit_zero
}

#[test]
fn test_b() {
    let input = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;
    assert_eq!(b(input), 6);
}
