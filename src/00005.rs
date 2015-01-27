
fn main() {
    let base = 2..21;
    let mut numbers = 21u32..(base.fold(1, |product, aa| product*aa)+1);

    // fn all() 's performance is worse than for loop
    println!("{}", numbers.find(|&x| base.all(|y| x % y == 0)).unwrap());
}