extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("{}", (1..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |a, b| a + b));

    stop_timer(t);
}