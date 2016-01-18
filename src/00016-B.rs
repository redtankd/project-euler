extern crate project_euler;

use project_euler::*;

const MAX_POWER: u32 = 1000;

fn main() {
    let t = start_timer();

    let mut digits: Vec<u32> = vec![2, 0]; // zero is for carry.

    for _ in 0..MAX_POWER-1 {

        digits = digits
            .into_iter()
            // scan() is slower than map()
            // when map() is used, the 'carry' could be an environment variable.
            .scan(0, |carry, d| {
                let d_new = d * 2 + *carry;
                *carry = d_new / 10;
                Some(d_new % 10) })
            .collect();

        if *digits.last().unwrap() > 0 {
            digits.push(0);
        }

    }

    let sum = digits.iter().fold(0, |acc, &item| acc + item);

    println!("");
    println!("sum={}", sum);

    stop_timer(t);
}

