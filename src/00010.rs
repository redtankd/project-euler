
fn main() {
    let mut list: Vec<u64> = vec![2];

    println!("{}", (3..2000000)
        .filter(|&x| if list.iter().any(|&y| x % y == 0) {
            false
        } else {
            list.push(x);
            true
        })
        .fold(0, |sum, x| sum + x) + 2);
}