#![feature(test)]

extern crate test;
extern crate project_euler;

use std::collections::HashSet;
use project_euler::*;

pub type MyInt = i32;
pub const MAX: MyInt = 20000;

fn main() {
    let t = start_timer();

    println!("\nsolution 1:\n");
    let (n, a, b) = s1(&primes(MAX));
    println!("n = {}, a = {}, b={}, product of a and b = {}\n",
             n,
             a,
             b,
             a * b);

    stop_timer(t);
}

pub fn s1(primes: &HashSet<MyInt>) -> (usize, MyInt, MyInt) {
    (2..1001) // the loop for b
        .filter(|b| primes.contains(b)) // b must be a prime
        .flat_map(|b| { // flattens nested structure
            (-999..1000) // the loop for a
                .map(|a| { // map a and b to n
                    let n = (0..)
                        .map(|n| {
                            let z = n * n + a * n + b;
                            if z < 2 {
                                return false;
                            } else if z >= MAX {
                                panic!("Primes is overflow for n = {}, a = {}, b = {}", n, a, b);
                            } else {
                                return primes.contains(&z);
                            }
                        })
                        .position(|z| !z) // find the consecutive n for a and b
                        .unwrap();
                    return (n, a, b);
                })
                .collect::<Vec<(usize, MyInt, MyInt)>>() // flat_map() need a IntoIterator
        })
        .collect::<Vec<(usize, MyInt, MyInt)>>()
        .iter()
        .fold((0, 0, 0), // find the maximum number of primes for consecutive values of n
              |(n_max, a_max, b_max), &(n, a, b)| if n > n_max {
                  (n, a, b)
              } else {
                  (n_max, a_max, b_max)
              })
}

pub fn primes(max: MyInt) -> HashSet<MyInt> {
    let mut set = HashSet::new();
    set.insert(2);

    for x in 3..max {
        if set.iter().all(|&y| x % y != 0) {
            set.insert(x);
        }
    }

    return set;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_s1() {
        let (n, a, b) = s1(&primes(MAX));
        assert_eq!(71, n);
        assert_eq!(-59231, a*b);
    }

    #[bench]
    fn bench_s1(b: &mut Bencher) {
        let primes = primes(MAX);
        b.iter(move || s1(&primes));
    }

    #[bench]
    fn bench_primes(b: &mut Bencher) {
        b.iter(move || primes(MAX));
    }
}
