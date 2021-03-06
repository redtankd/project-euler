use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[cfg(not(test))]
fn main() {
    use project_euler::*;

    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("sum={}\n", s1("src/resource/00022.txt"));

    stop_timer(t);
}

fn s1(name: &str) -> u64 {
    let f = File::open(name).unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);

    let mut s = line
        .split(',')
        .map(|x| x.trim_matches('"'))
        .collect::<Vec<&str>>();
    s.sort();
    s.iter()
        .enumerate()
        .map(|(p, &x)| {
            x.as_bytes()
                .iter()
                .map(|&x| x as u64 - 64)
                .fold(0, |sum, x| sum + x)
                * (p as u64 + 1)
        })
        .fold(0, |sum, x| sum + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1_test() {
        assert_eq!(871198282, s1("src/resource/00022.txt"));
    }
}
