extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    let mut it: Vec<(u32, u32)> = Vec::new();
    for i in (100..1000).rev() {
        for j in (100..i).rev() {
            it.push((i,j));
        }
    }

    let mut max = 0u32;

    while let Some(p) = it.iter()
        .map(|&(x, y)| x*y)
        .filter(|&x| { // great for performance
            x > max
        })
        .filter(|&x| {
            let str = x.to_string();
            str.chars().rev().collect::<String>() == str
        }).next() {
        max = p;
    }

    println!("{}", max);

    stop_timer(t);
}