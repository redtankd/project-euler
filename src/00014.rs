extern crate project_euler;

use project_euler::*;
use std::collections::HashMap;

const MAX_NUMBER: u64 = 1000000;

fn main() {
    let t = start_timer();
    println!("\nsolution 1:\n");
    let (number1, max1) = s1();
    println!("{} has the the max length {:?}\n", number1, max1);
    stop_timer(t);

    let t = start_timer();
    println!("\nsolution 2:\n");
    let (number2, max2) = s2();
    println!("{} has the the max length {:?}\n", number2, max2);
    stop_timer(t);
}

fn s1() -> (u64, u64) {
    // the sequence's length for all numbers
    // it is a cache
    let mut lengths = HashMap::new();
    lengths.insert(1u64, 1u64);

    // the sequence for a number
    let mut seq: [u64; 1000] = [0; 1000];

    let mut n = MAX_NUMBER;
    while n > 1 {
        let mut i = 0;
        seq[i] = n;
        while seq[i] != 1 && !lengths.contains_key(&seq[i]) {
            // skips if number is in the cache
            i += 1;
            seq[i] = f(seq[i - 1]);
        }
        let r = *lengths.get(&seq[i]).unwrap();
        for k in 0..i {
            lengths.insert(seq[k], (i as u64) - (k as u64) + r);
        }

        n -= 1;
    }

    let mut max = 0;
    let mut number = 0;
    for (nn, nlengths) in lengths {
        if nn <= MAX_NUMBER && nlengths > max {
            max = nlengths;
            number = nn
        }
    }

    (number, max)
}

fn s2() -> (u64, u64) {
    // the sequence's length for all numbers
    // it is a cache
    let mut lengths: HashMap<u64, u64> = HashMap::new();
    lengths.insert(1, 1);

    for i in (1..MAX_NUMBER).rev() {
        ff(i, &mut lengths);
    }

    let mut max = 0;
    let mut number = 0;
    for (n, length) in lengths {
        if n < MAX_NUMBER && length > max {
            max = length;
            number = n
        }
    }

    (number, max)
}

fn f(n: u64) -> u64 {
    match n % 2 {
        0 => n / 2,
        1 => 3 * n + 1,
        _ => 0,
    }
}

fn ff(n: u64, lengths: &mut HashMap<u64, u64>) -> (u64, &mut HashMap<u64, u64>) {
    if n == 1 {
        return (1, lengths);
    } else if lengths.contains_key(&n) {
        return (*lengths.get(&n).unwrap(), lengths);
    } else {
        let (next_length, new_lengths) = ff(f(n), lengths);
        let length = next_length + 1;
        new_lengths.insert(n, length);
        return (length, new_lengths);
    }
}

#[test]
fn s1_test() {
    assert_eq!((837799, 525), s1());
}

#[test]
fn s2_test() {
    assert_eq!((837799, 525), s2());
}
