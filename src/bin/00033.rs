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

fn s1() -> u32 {
    let r = (1..10).map(|a|
        iproduct!((1..10), (1..10), (a..a+1))
            .filter(|&(x, y, a)| y>x && a*10+y>x*10+a && (a*10+y)*x==(x*10+a)*y)
            .inspect(|&(x, y, a)| println!("{:?}{:?}/{:?}{:?}", x, a, a, y))
            .fold((1,1), |(px, py), (x, y, _)| (px*x, py*y))
    )
    .fold((1,1), |(px, py), (x, y)| (px*x, py*y));
    r.1/r.0
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(100, s1());
    }
}
