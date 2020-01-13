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
    iproduct!(0..100, 0..40, 0..20, 0..10, 0..4, 0..2)
        .filter(|&(p2, p5, p10, p20, p50, d1)| {
            200 >= p2 * 2 + p5 * 5 + p10 * 10 + p20 * 20 + p50 * 50 + d1 * 100
        })
        .count()
        + 7 // for p2 *100, p5 * 40 .. d2 * 1
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(73682, s1());
    }
}
