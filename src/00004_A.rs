
fn main() {
    let mut max = (0, 0, 0);

    for i in (100..1000).rev() {
        for j in (100..i).rev() {
            let p = i*j;
            if p < max.0 { continue; }
            let str = p.to_string();
            let strs = str.as_slice();
            if strs.chars().rev().collect::<String>() == str {
                max = (p, i, j);
            }
        }
    }

    println!("{}, {}, {}", max.0, max.1, max.2);
}