const MAX_POWER: u32 = 1000;
const MAX_DIGITS_NUMBER: usize = 400;

#[cfg(not(test))]
fn main() {
    use project_euler::*;

    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("sum={}\n", s1());

    stop_timer(t);

    let t = start_timer();

    println!("\nsolution 2:\n");
    println!("sum={}\n", s2());

    stop_timer(t);
}

fn s1() -> u32 {
    let mut digits: Vec<u32> = vec![0; MAX_DIGITS_NUMBER];
    digits[0] = 2;
    let mut digits_num = 1;

    for _ in 0..MAX_POWER - 1 {
        let mut carry = 0;

        for digit in digits.iter_mut().take(digits_num) {
            *digit = *digit * 2 + carry;
            carry = *digit / 10;
            *digit %= 10;
        }

        if carry > 0 {
            digits[digits_num] = carry;
            digits_num += 1;
        }
    }

    digits
        .iter()
        .take(digits_num)
        .fold(0, |acc, &item| acc + item)
}

fn s2() -> u32 {
    let mut digits: Vec<u32> = vec![2, 0]; // zero is for carry.

    for _ in 0..MAX_POWER - 1 {
        digits = digits
            .into_iter()
            // scan() is slower than map()
            // when map() is used, the 'carry' could be an environment variable.
            .scan(0, |carry, d| {
                let d_new = d * 2 + *carry;
                *carry = d_new / 10;
                Some(d_new % 10)
            })
            .collect();

        if *digits.last().unwrap() > 0 {
            digits.push(0);
        }
    }

    digits.iter().fold(0, |acc, &item| acc + item)
}

#[test]
fn s1_test() {
    assert_eq!(1366, s1());
}

#[test]
fn s2_test() {
    assert_eq!(1366, s2());
}
