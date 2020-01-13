extern crate project_euler;

use project_euler::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let t = start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    stop_timer(t);
}

fn s1() -> u32 {
    let mut circular = HashMap::<Vec<u8>, u8>::new();

    // find all primes and mark all as non circular
    let ps = primes()
        .take_while(|&x| x < 1000000)
        .map(|x| {
            let x1 = x.to_string().into_bytes();
            circular.insert(x1.clone(), 0); // 0 - not scan
                                            // 1 - scan
                                            // 2 - circular
            x1
        })
        .collect::<Vec<Vec<u8>>>();

    for x in ps.iter() {
        // skip if scanned
        if *circular.get(x).unwrap() != 0u8 {
            continue;
        }

        let mut rotate = HashSet::<Vec<u8>>::new();
        let mut digits = x.clone();
        let mut all_exist = true;

        for _ in 0..digits.len() {
            if circular.contains_key(&digits) {
                circular.insert(digits.clone(), 1);
            } else {
                all_exist = false;
            }
            rotate.insert(digits.clone());
            digits.rotate_left(1);
        }

        if all_exist {
            for x in rotate {
                circular.insert(x.clone(), 2);
            }
        }
    }

    circular.values().filter(|x| **x == 2).count() as u32
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(55, s1());
    }
}
