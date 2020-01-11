#![cfg_attr(all(feature = "nightly", test), feature(test))]
#[cfg(all(feature = "nightly", test))] extern crate test;

extern crate project_euler;

use std::ops::*;
use std::cmp::*;
use std::default::Default;
use std::collections::HashSet;
use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution:");
    println!("The answer is {}\n", s1(100, 100));

    stop_timer(t);
}


fn s1(a: u16, b: u64) -> usize {
    let mut set = HashSet::new();

    for aa in 2..a + 1 {
        for bb in 2..b + 1 {
            // It could be more efficient if p is inserted into set in pow()
            let p = pow(aa, bb, 100);
            let s = p.iter().map(|x| x.to_string()).collect::<Vec<String>>().concat();
            set.insert(s);
        }
    }

    return set.len();
}

// n ** p
// ten_unit is 100, 1000, 10000, etc
fn pow<T>(n: T, p: u64, ten_unit: T) -> Vec<T>
    where T: Default + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + PartialOrd
{
    let mut value = vec![n];
    for _ in 0..p - 1 {
        for v in value.iter_mut() {
            *v = *v * n;
        }
        carry(&mut value, ten_unit);
    }
    return value;
}

// ten_unit is 100, 1000, 10000, etc
fn carry<T>(number: &mut Vec<T>, ten_unit: T)
    where T: Default + Copy + Add<Output = T> + Div<Output = T> + Rem<Output = T> + PartialOrd
{
    let mut carry: T = Default::default();

    for ni in number.iter_mut() {
        let _ni = carry + *ni;
        carry = _ni / ten_unit;
        *ni = _ni % ten_unit;
    }
    while carry >= ten_unit {
        number.push(carry % ten_unit);
        carry = carry / ten_unit;
    }
    if carry > Default::default() {
        number.push(carry);
    }
}

#[cfg(test)]
mod tests {
    use super::carry;
    use super::pow;
    use super::s1;

    #[test]
    fn test_carry() {
        let mut d = vec![4021];
        carry(&mut d, 10);
        assert_eq!(1, d[0]);
        assert_eq!(2, d[1]);
        assert_eq!(0, d[2]);
        assert_eq!(4, d[3]);
        assert_eq!(4, d.len());

        let mut d = vec![4021];
        carry(&mut d, 100);
        assert_eq!(21, d[0]);
        assert_eq!(40, d[1]);
        assert_eq!(2, d.len());
    }

    #[test]
    fn test_pow() {
        let d = pow(5, 5, 10);
        assert_eq!(5, d[0]);
        assert_eq!(2, d[1]);
        assert_eq!(1, d[2]);
        assert_eq!(3, d[3]);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use test::Bencher;
    use super::carry;
    use super::pow;
    use super::s1;

    #[bench]
    fn bench_s1(b: &mut Bencher) {
        assert_eq!(9183, s1(100, 100));
        b.iter(move || s1(100, 100));
    }
}