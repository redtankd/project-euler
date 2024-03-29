use std::collections::BTreeSet;

use project_euler::is_prime;

#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    project_euler::stop_timer(t);
}

fn s1() -> u32 {
    (11..)
        .scan(
            vec![2, 3, 5, 7].into_iter().collect::<BTreeSet<u32>>(),
            |primes, x: u32| {
                if is_prime(x as u64) {
                    primes.insert(x);

                    // truncate from left to right
                    let mut str = x.to_string();
                    for _ in 1..str.len() {
                        str = str.split_off(1);

                        if !primes.contains(&str.parse::<u32>().unwrap()) {
                            return Some(0);
                        }
                    }

                    // truncate from right to left
                    let mut str = x.to_string();
                    for _ in 1..str.len() {
                        let len = str.len() - 1;
                        let _ = str.split_off(len);

                        if !primes.contains(&str.parse::<u32>().unwrap()) {
                            return Some(0);
                        }
                    }

                    println!("{:?}", x);
                    Some(x)
                } else {
                    Some(0)
                }
            },
        )
        .filter(|&x| x != 0)
        .take(11)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(748317, s1());
    }
}
