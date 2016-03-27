extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("sum={}\n", s1());

    stop_timer(t);
}

fn s1() -> u64 {
    let dn_all: Vec<u64> = (0..10001)
        .map(|x| (2..x/2+1)
                .filter(|&y| x%y == 0 && y <= x/y)
                .fold(1, |acc, z| if x/z == z { acc + z } else { acc + z + x/z } )
            )
        .collect();

    let mut sum = 0;
    for (n, &dn) in dn_all.iter().enumerate() {
        if dn < 10001 && n == dn_all[dn as usize] as usize && n != dn as usize {
            sum += n as u64;
        }
    }

    sum
}

#[test]
fn s1_test() {
    assert_eq!(31626, s1());
}