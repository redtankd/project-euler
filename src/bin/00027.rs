#![cfg_attr(all(feature = "nightly", test), feature(test))]
#[cfg(all(feature = "nightly", test))] extern crate test;

extern crate project_euler;

use std::collections::HashSet;
use project_euler::*;

pub type MyInt = i32;
pub const MAX: MyInt = 20000;

fn main() {
    let t = start_timer();

    println!("\nsolution:\n");
    let (n, a, b) = s3(&primes(MAX));
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

// there is no collect(), so the performance is best in the 3 solutions.
pub fn s2(primes: &HashSet<MyInt>) -> (usize, MyInt, MyInt) {
    (2..1001) // the loop for b
        .filter(|b| primes.contains(b)) // b must be a prime
        .map(|b| { // flattens nested structure
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
                .fold((0, 0, 0), // find the maximum number of primes for consecutive values of n
                    |(n_max, a_max, b_max), (n, a, b)| if n > n_max {
                        (n, a, b)
                    } else {
                        (n_max, a_max, b_max)
                    })
        })
        .fold((0, 0, 0), // find the maximum number of primes for consecutive values of n
              |(n_max, a_max, b_max), (n, a, b)| if n > n_max {
                  (n, a, b)
              } else {
                  (n_max, a_max, b_max)
              })
}

pub fn s3(primes: &HashSet<MyInt>) -> (MyInt, MyInt, MyInt) {
    let mut n_max = 0;
    let mut a_max = 0;
    let mut b_max = 0;

    for b in 2..1001 {
        if !primes.contains(&b) {
            continue;
        }

        for a in -999..1000 {
            let mut n = 0;
            for n1 in 0.. {
                let z = n1 * n1 + a * n1 + b;
                if z >= MAX {
                    panic!("Primes is overflow for n = {}, a = {}, b = {}", n, a, b);
                } else if z < 2 || !primes.contains(&z) {
                    n = n1;
                    break;
                }
            }
            if n > n_max {
                n_max = n;
                a_max = a;
                b_max = b;
            }
        }
    }

    return (n_max, a_max, b_max);
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

    #[test]
    fn test() {
        let primes = primes(MAX);

        let (n, a, b) = s1(&primes);
        assert_eq!(71, n);
        assert_eq!(-59231, a * b);

        let (n, a, b) = s2(&primes);
        assert_eq!(71, n);
        assert_eq!(-59231, a * b);

        let (n, a, b) = s3(&primes);
        assert_eq!(71, n);
        assert_eq!(-59231, a * b);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_s1(b: &mut Bencher) {
        let primes = primes(MAX);
        b.iter(move || s1(&primes));
    }

    #[bench]
    fn bench_s2(b: &mut Bencher) {
        let primes = primes(MAX);
        b.iter(move || s2(&primes));
    }

    #[bench]
    fn bench_s3(b: &mut Bencher) {
        let primes = primes(MAX);
        b.iter(move || s3(&primes));
    }

    #[bench]
    fn bench_primes(b: &mut Bencher) {
        b.iter(move || primes(MAX));
    }
}