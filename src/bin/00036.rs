#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    project_euler::stop_timer(t);
}

fn s1() -> u32 {
    (1..1000000)
        .filter(|x| {
            let binary = format!("{:b}", x);
            let decimal = format!("{}", x);
            decimal.chars().rev().collect::<String>() == decimal
                && binary.chars().rev().collect::<String>() == binary
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(872187, s1());
    }
}
