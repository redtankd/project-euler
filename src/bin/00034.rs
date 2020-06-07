#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    project_euler::stop_timer(t);
}

fn s1() -> u32 {
    (11u32..)
        .filter(|n| {
            *n == n
                .to_string()
                .chars()
                .map(|c| u32::from(c) - 48)
                .map(|c| (1..c + 1).fold(1, |pn, p| pn * p))
                .sum::<u32>()
        })
        .take(2)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(40730, s1());
    }
}
