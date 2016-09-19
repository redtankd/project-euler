extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("Solution 1 = {}", (1..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |a, b| a + b));

    stop_timer(t);

    println!("");
    
    let t = start_timer();

    let mut sum = 0;
    for i in 1..1000 {
        if i%3 == 0 || i%5 == 0 {
            sum += i;
        }
    }
    println!("Solution 2 = {}", sum);

    stop_timer(t);
}