extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("sum={}\n", s1());

    stop_timer(t);
}

fn s1() -> u64 {
    let mut n = vec![0; 28124];

    let abundant = (12..28124)
        .filter(|&x| f(x).iter().fold(0, |sum, y| sum + y) > x)
        .collect::<Vec<u64>>();

    abundant
        .iter() // which iterates over &T
        .enumerate()
        .filter(|&(_, &a)| a + a < 28124) // the closure's type is FnMut(&Self::Item), so it is &&u64
        .inspect(|&(i, &a)| {
            abundant[i..]
                .iter()
                .inspect(|&&b| {
                    if a + b < 28124 {
                        n[(a + b) as usize] = 1;
                    }
                })
                .last();
        })
        .last();

    n.iter()
        .enumerate()
        .filter(|&(_, &x)| x == 0)
        .fold(0, |sum, (i, &_)| sum + i as u64)
}

fn f(x: u64) -> Vec<u64> {
    (2..(x as f64).sqrt() as u64 + 1)
        .filter(|&y| x % y == 0)
        .fold(vec![1], |mut divs, y| {
            divs.push(y);
            let z = x / y;
            if z != y {
                divs.push(z);
            }
            divs
        })
}

#[test]
fn s1_test() {
    assert_eq!(4179871, s1());
}
