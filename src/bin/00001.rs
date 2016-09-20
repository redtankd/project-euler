#![feature(test)]

extern crate test;
extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();
    let sum = s1(1000);
    stop_timer(t);
    println!("Solution 1 = {}", sum);

    println!("");
    
    let t = start_timer();
    let sum = s2(1000);
    stop_timer(t);
    println!("Solution 2 = {}", sum);
}

pub fn s1(max: u64) -> u64 {
     (1..max)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |a, b| a + b)
}

pub fn s2(max: u64) -> u64 {
    let mut sum = 0;
    for i in 1..max {
        if i%3 == 0 || i%5 == 0 {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn s1_bench(b: &mut Bencher) {
        b.iter(|| s1(1000));
    }

    #[bench]
    fn s2_bench(b: &mut Bencher) {
        b.iter(|| s2(1000));
    }
}