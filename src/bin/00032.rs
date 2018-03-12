#![feature(test)]
#![feature(conservative_impl_trait)]
#![feature(universal_impl_trait)]

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
    // the start value: (the drawn result, the collection to draw)
    let p = vec![(Vec::new(), (1..10).collect::<Vec<u32>>())];
    // rust can't loop with FlatMap because FlatMap's type augment FnMut.
    // so cast it to a trait object.
    let mut p_iter = Box::new(p.into_iter()) 
        as Box<Iterator<Item=(Vec<u32>, Vec<u32>)>>;
    for _ in 0..9 {
        p_iter = Box::new(p_iter.flat_map(|(drawn, to_draw)| { 
                draw_digit(drawn, to_draw) 
            })
        );
    }

    //find products
    let mut p = p_iter
        .flat_map(|(p, _)| {
            let mut products: Vec<u32> = Vec::new();
            let a = p[0]*10  + p[1];
            let b = p[2]*100 + p[3]*10 + p[4]; 
            let c = p[5]*1000+ p[6]*100+ p[7]*10 + p[8]; 
            if a*b == c { 
                // println!("{} * {} ={}", a, b, c);
                products.push(c); 
            }
            
            // the same value in the different sequence
            // let a = p[0]*100 + p[1]*10 + p[2];
            // let b = p[3]*10  + p[4]; 
            // let c = p[5]*1000+ p[6]*100+ p[7]*10 + p[8]; 
            // if a*b == c { println!("{} * {} = {}", a, b, c);products.push(c); }

            let a = p[0];
            let b = p[1]*1000 + p[2]*100 + p[3]*10 + p[4]; 
            let c = p[5]*1000 + p[6]*100 + p[7]*10 + p[8]; 
            if a*b == c { 
                // println!("{} * {} = {}", a, b, c);
                products.push(c); 
            }

            products
        })
        .collect::<Vec<u32>>();
        
    // sum
    p.sort();
    p.dedup();
    p.iter().sum()
}

// Don't know how to solve at this moment
// fn my_flat_map<I, U, F>(iter: impl Iterator<Item=(Vec<u32>, Vec<u32>)>)
//     -> FlatMap<I, U, F> 
//     where 
//         I: Iterator<Item=(Vec<u32>, Vec<u32>)>,
//         U: Iterator,
//         F: FnMut(I::Item) -> U
// {
//     iter.flat_map(|(drawn, to_draw)| { draw_digit(drawn, to_draw) })
// }

// drawn - the drawn digits
// to_draw - the digits to draw
fn draw_digit(drawn: Vec<u32>, to_draw: Vec<u32>) 
    -> impl Iterator<Item=(Vec<u32>, Vec<u32>)> {
    (0..to_draw.len()).map(move |i| {
        let mut left = to_draw.to_vec();
        let next = left.remove(i);
        let mut drawn_new = drawn.to_vec();
        drawn_new.push(next);
        (drawn_new, left)
    })
}

#[cfg(test)]
mod tests {
    use super::s1;

    #[test]
    fn test_s1() {
        assert_eq!(45228, s1());
    }
}
