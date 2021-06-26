
#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    project_euler::stop_timer(t);
}

fn s1() -> u32 {
    let mut max = 0;
    let mut p_max = 0;

    (1..=1000).for_each(|p| {
        let mut count = 0;

        (1..p).for_each(|a| {
            (a..(p - a)).for_each(|b| {
                let c = p - a - b;
                if a * a + b * b == c * c {
                    count += 1;
                }
            });
        });

        if count > max {
            max = count;
            p_max = p;
        }
    });

    return p_max;
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(840, s1());
    }
}
