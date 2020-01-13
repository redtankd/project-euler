#![cfg_attr(all(feature = "nightly", test), feature(test))]
#[cfg(all(feature = "nightly", test))]
extern crate test;

extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution:");
    println!("The answer is {}\n", s1());

    stop_timer(t);
}

fn s1() -> u32 {
    let mut sum_all = 0;

    for d in 2..200000 {
        let mut dl = d;
        let mut sum = 0;
        while dl > 0 {
            let dr: u32 = dl % 10;
            dl = dl / 10;
            sum += dr.pow(5);
        }
        if d == sum {
            sum_all += sum;
        }
    }

    return sum_all;
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use super::s1;
    use test::Bencher;

    #[bench]
    fn bench_s1(b: &mut Bencher) {
        assert_eq!(443_839, s1());
        b.iter(move || s1());
    }
}
