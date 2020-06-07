#![cfg_attr(all(feature = "nightly", test), feature(test))]
#[cfg(all(feature = "nightly", test))]
extern crate test;

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("");
    let (wanted, wanted_reciprocal, wanted_len) = s1(1000);
    println!(
        "\nthe longest recurring cycle comes from d = {}, reciprocal = {}, the length = {}\n",
        wanted, wanted_reciprocal, wanted_len
    );

    project_euler::stop_timer(t);
}

pub fn s1(max: u16) -> (u16, String, usize) {
    let mut wanted = 0;
    let mut wanted_reciprocal = String::from("");
    let mut wanted_len = 0;

    for i in 2..max {
        if let Some((recip, len)) = reciprocal(i) {
            // println!("d = {}, the length = {}, reciprocal = {}", i, len, recip);
            if len > wanted_len {
                wanted = i;
                wanted_reciprocal = recip;
                wanted_len = len;
            }
        } else {
            panic!("Array overflowed when d = {}", i);
        }
    }

    return (wanted, wanted_reciprocal, wanted_len);
}

const MAX: usize = 1000;

// the return value is (reciprocal, cycle's length)
fn reciprocal(divisor: u16) -> Option<(String, usize)> {
    let mut vec_q = vec![String::new(); MAX];
    let mut vec_r = vec![0; MAX];
    let mut hash = HashMap::<u16, usize>::with_capacity(MAX);

    vec_r[0] = 1;
    hash.insert(1, 0);
    for i in 1..MAX {
        // preventing from overflow
        let (q, r) = div(vec_r[i - 1], divisor);
        vec_q[i] = q;
        vec_r[i] = r;

        // no remainder
        if r == 0 {
            let mut str = String::from("0.");
            for ref qq in vec_q.iter().take(i + 1) {
                str.push_str(&format!("{}", qq));
            }
            return Some((str, 0));
        }

        // find repeated remainder, so a cycle exists.
        // the method is not perfect, sometimes first digits in a cylce are unable to be idendified
        if let Some(pos) = hash.get(&r) {
            let mut str = String::from("0.");
            // vec[pos+1] is the start the cycle.
            for ref qq in vec_q.iter().take(pos + 1) {
                str.push_str(&format!("{}", qq));
            }

            str.push_str("(");
            let len1 = str.len();
            for ref qq in vec_q.iter().take(i + 1).skip(pos + 1) {
                str.push_str(&format!("{}", qq));
            }
            let len2 = str.len() - len1;
            str.push_str(&format!(")"));

            return Some((str, len2));
        }

        hash.insert(r, i);
    }

    return None;
}

// The return value is (quotient, remainder)
fn div(dividend: u16, divisor: u16) -> (String, u16) {
    if dividend / divisor >= 1 {
        return (format!("{}", dividend / divisor), dividend % divisor);
    }

    if dividend * 10 / divisor >= 1 {
        return (
            format!("{}", dividend * 10 / divisor),
            dividend * 10 % divisor,
        );
    }

    if dividend * 100 / divisor >= 1 {
        return (
            format!("0{}", dividend * 100 / divisor),
            dividend * 100 % divisor,
        );
    }

    if dividend * 1000 / divisor >= 1 {
        return (
            format!("00{}", dividend * 1000 / divisor),
            dividend * 1000 % divisor,
        );
    }

    return (String::new(), 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s1() {
        let (wanted, _wanted_reciprocal, wanted_len) = s1(1000);
        assert_eq!(983, wanted);
        assert_eq!(982, wanted_len);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_s1(b: &mut Bencher) {
        b.iter(|| s1(1000));
    }
}
