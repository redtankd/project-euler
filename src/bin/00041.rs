use project_euler::*;

#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    project_euler::stop_timer(t);
}

fn s1() -> u32 {
    let list = (1..=7).collect::<Vec<u32>>();
    let mut max = 0;

    for p in permutation(&list) {
        let n = list2number(&p);
        if is_prime(n as u64) && n > max {
            max = n;
        }
    }

    return max;
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(7652413, s1());
    }
}
