use itertools::Itertools;
use time::precise_time_s;

pub fn start_timer() -> f64 {
    precise_time_s()
}

pub fn stop_timer(t: f64) {
    println!("time elapse {} second", precise_time_s() - t);
}

pub fn primes() -> impl Iterator<Item = u32> {
    (2..)
        .scan(Vec::<u32>::new(), |state, x| {
            if state
                .iter()
                .take_while(|&y| *y <= x / 2)
                .all(|&y| x % y != 0)
            {
                state.push(x);
                Some(x)
            } else {
                Some(0)
            }
        })
        .filter(|&x| x != 0)
}

/**
    费马小定理，这是一个质数的必要非充分条件。

    可以使用“二次检验定理”，提高正确性
 */
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    let ps: Vec<u64> = vec![2, 3, 5, 7, 11];

    if ps.contains(&n) {
        return true;
    }

    for p in ps {
        if qpow_mod(p, (n - 1) as u64, n as u64) == 1 {
            continue;
        } else {
            return false;
        }
    }

    true
}

/**
   quick power

   see https://zhuanlan.zhihu.com/p/95902286
*/
pub fn qpow(base: u64, p: u64) -> u64 {
    let mut ans: u64 = 1;
    let mut base = base;
    let mut p = p;

    while p != 0 {
        if p & 1 == 1 {
            ans *= base;
        }
        base *= base;
        p >>= 1;
    }

    return ans;
}

pub fn qpow_mod(base: u64, p: u64, m: u64) -> u64 {
    let mut ans: u64 = 1;
    let mut base = base;
    let mut p = p;

    while p != 0 {
        if p & 1 == 1 {
            ans = ans * base % m;
        }
        base = base * base % m;
        p >>= 1;
    }

    return ans;
}

/**
    With FlatMap and Map, we expand the orignal list to every possbile combination. For example

    ([], [1,2,3])                                       map in draw() and flat_map in permutation() =>
    [ ([1], [2, 3]), ([2], [1, 3]), ([3], [1, 2]) ]     again =>
    [ ([1, 2], [3]), ([1, 3], [2]), ... ]               again =>
    ...
*/
pub fn permutation<'a, T: Clone>(list: &'a [T]) -> impl Iterator<Item = Vec<T>> + 'a {
    let p = vec![(Vec::new(), list.to_vec())];

    // Vec wrapped in FlatMap, whose type is very complicated,
    // so cast it to a trait object.
    let mut p_iter = Box::new(p.into_iter()) as Box<dyn Iterator<Item = (Vec<T>, Vec<T>)>>;

    for _ in 0..list.len() {
        p_iter = Box::new(p_iter.flat_map(|(drawn, to_draw)| draw(drawn, to_draw)));
    }

    fn draw<T: Clone>(drawn: Vec<T>, to_draw: Vec<T>) -> impl Iterator<Item = (Vec<T>, Vec<T>)> {
        (0..to_draw.len()).map(move |i| {
            let mut to_draw_new = to_draw.to_vec();
            let drawn_item = to_draw_new.remove(i);
            let mut drawn_new = drawn.to_vec();
            drawn_new.push(drawn_item);
            (drawn_new, to_draw_new)
        })
    }

    p_iter.map(|x| x.0)
}

/**
   For example [1, 2, 3, 4] => 1234
*/
pub fn list2number(list: &[u32]) -> u32 {
    let mut r: u32 = 0;
    for (i, v) in list.iter().rev().enumerate() {
        r += v * 10u32.pow(i as u32);
    }
    r
}

pub fn number2list(n: u32) -> Vec<u32> {
    n.to_string()
        .as_bytes()
        .into_iter()
        .map(|b| (b - 48) as u32)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::{qpow, qpow_mod};

    #[test]
    fn test_qpow() {
        assert_eq!(282_475_249, qpow(7, 10));
    }

    #[test]
    fn test_qpow_mod() {
        assert_eq!(4, qpow_mod(7, 10, 5));
    }
}
