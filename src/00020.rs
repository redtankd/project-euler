extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("sum={}\n", s1());

    stop_timer(t);
}

fn s1() -> u64 {
    let mut n = vec![1];

    for i in 2..101 {
        n = n.iter().map(|x| x * i).collect();
        advance(&mut n);
    }

    n.iter().fold(0, |s, &x| s + x)
}

fn advance(n: &mut Vec<u64>) {
    let mut adv = 0;

    for ni in n.iter_mut() {
        let _ni = adv + *ni;
        adv = _ni / 10;
        *ni = _ni % 10;
    }
    while adv > 9 { 
        n.push(adv % 10); 
        adv /= 10;
    }
    if adv > 0 { n.push(adv); }
}

#[test]
fn s1_test() {
    assert_eq!(648, s1());
}