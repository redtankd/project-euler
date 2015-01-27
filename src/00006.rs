
fn main() {
    let number = 1..101;
    
    let sum = number.fold((0, 0), |(x1, x2), y| (x1 + y*y, x2 + y));
    
    println!("{}", sum.1*sum.1 - sum.0);
}