const MAX: u64 = 20;

fn main() {
    let max_product = (2..(MAX + 1)).fold(1, |product, aa| product * aa);

    // fn all() 's performance is worse than for loop
    println!(
        "{}",
        ((MAX + 1)..(max_product + 1))
            .find(|&x| (2..(MAX + 1)).all(|y| x % y == 0))
            .unwrap()
    );
}
