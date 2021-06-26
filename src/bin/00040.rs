#[cfg(not(test))]
fn main() {
    let t = project_euler::start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    project_euler::stop_timer(t);
}

fn s1() -> u32 {
    let mut str = String::with_capacity(1_000_100);
    str.push_str("0");

    let mut i = 1;
    while str.len() < 1_000_001 {
        str.push_str(&i.to_string());
        i += 1;
    }

    let bytes = str.as_bytes();
    let d1 = u32::from(bytes[1] - 48);
    let d10 = u32::from(bytes[10] - 48);
    let d100 = u32::from(bytes[100] - 48);
    let d1000 = u32::from(bytes[1000] - 48);
    let d10000 = u32::from(bytes[10000] - 48);
    let d100000 = u32::from(bytes[100000] - 48);
    let d1000000 = u32::from(bytes[1000000] - 48);

    d1 * d10 * d100 * d1000 * d10000 * d100000 * d1000000
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(210, s1());
    }
}
