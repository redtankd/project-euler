#![feature(test)]

extern crate test;
extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution:");
    println!("The answer is {:?}\n", s1());

    stop_timer(t);
}

fn s1() -> u32 {
    // permutation for 1..10
    let mut p = vec![(Vec::new(), (1..10).collect::<Vec<u32>>())];
    for _ in 1..10 {
        p = p
            .iter()
            .flat_map(|&(ref p, ref left)| { draw_digit(p, left) })
            .collect::<Vec<(Vec<u32>, Vec<u32>)>>();
    }

    // find products
    let mut p = p.iter()
        .flat_map(|&(ref p, _)| {
            let mut products: Vec<u32> = Vec::new();
            let a = p[0]*10  + p[1];
            let b = p[2]*100 + p[3]*10 + p[4]; 
            let c = p[5]*1000+ p[6]*100+ p[7]*10 + p[8]; 
            if a*b == c { println!("{} * {} ={}", a, b, c);products.push(c); }
            
            // the same value in the different sequence
            // let a = p[0]*100 + p[1]*10 + p[2];
            // let b = p[3]*10  + p[4]; 
            // let c = p[5]*1000+ p[6]*100+ p[7]*10 + p[8]; 
            // if a*b == c { println!("{} * {} = {}", a, b, c);products.push(c); }

            let a = p[0];
            let b = p[1]*1000 + p[2]*100 + p[3]*10 + p[4]; 
            let c = p[5]*1000 + p[6]*100 + p[7]*10 + p[8]; 
            if a*b == c { println!("{} * {} = {}", a, b, c);products.push(c); }

            products
        })
        .collect::<Vec<u32>>();
        
    // sum
    p.sort();
    p.dedup();
    p.iter().sum()
}

// p - the drawn digits
// digits - the digits to draw
fn draw_digit(p: &Vec<u32>, digits: &Vec<u32>) -> Vec<(Vec<u32>, Vec<u32>)> {
    digits.iter().enumerate().map(|(i, _)| {
        let mut digits_new = digits.to_vec();
        let next = digits_new.remove(i);
        let mut p_new = p.to_vec();
        p_new.push(next);
        (p_new, digits_new)
        })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(45228, s1());
    }
}
