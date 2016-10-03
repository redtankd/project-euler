#![feature(test)]

extern crate test;
#[macro_use]
extern crate itertools;
extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    stop_timer(t);
}

fn s1() -> usize {
    iproduct!(0..201, 0..101, 0..41, 0..21, 0..11, 0..5, 0..3, 0..2)
        .filter(|&(p1, p2, p5, p10, p20, p50, d1, d2)|
            200 == p1 + p2 * 2 + p5 * 5 + p10 * 10 + p20 * 20 + p50 * 50 + d1 * 100 + d2 * 200
        )
        .count()
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    #[ignore]
    fn test_s1() {
        assert_eq!(73682, s1());
    }
}
