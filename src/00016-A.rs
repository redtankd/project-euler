extern crate project_euler;

use project_euler::*;

const MAX_POWER: u32 = 1000;
const MAX_DIGITS_NUMBER: usize = 400;

fn main() {
    let t = start_timer();

    let mut digits: Vec<u32> = vec![0; MAX_DIGITS_NUMBER];
    digits[0] = 2;
    let mut digits_num = 1;
    
    for _ in 0..MAX_POWER-1 {
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

    let sum = digits.iter().take(digits_num).fold(0, |acc, &item| acc + item);

    println!("");
    println!("sum={}", sum);

    stop_timer(t);
}

