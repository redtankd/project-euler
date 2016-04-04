extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution 1:\n");
    println!("answer={}\n", s1());

    stop_timer(t);

    let t = start_timer();

    println!("\nsolution 2:\n");
    println!("answer={}\n", s2());

    stop_timer(t);
}

fn s1() -> u64 {
    let mut fn_1 = vec![0 as u8; 1000];
    let mut fn_2 = vec![0 as u8; 1000];
    fn_1[0] = 1;
    fn_2[0] = 1;

    let mut i = 3;
    let mut fn_ = add(&fn_1, &fn_2);
    while fn_[999] == 0 {
        fn_2 = fn_1;
        fn_1 = fn_;
        fn_ = add(&fn_1, &fn_2);    
        i += 1;
    }
    
    i
}

fn s2() -> u64 {
    let mut fn_1 = vec![0 as u8; 1000];
    let mut fn_2 = vec![0 as u8; 1000];
    fn_1[0] = 1;
    fn_2[0] = 1;

    (3..)
    .scan( (Some(fn_1), fn_2),  
        |&mut(ref mut fn_1, ref mut fn_2), x| {

        // fn_1 is borrowed content, so we can't transfer ownership.
        // because this would leave the fn_1 in an inconsistent state.
        // In this case, you can use Option::take. This will leave 
        // the variable where it is, changing it in-place without moving it.
        let _fn_1 = fn_1.take().unwrap();

        let fn_ = add(&_fn_1, &fn_2);
        if fn_[999] != 0 { return None; }
        else { 
            *fn_2 = _fn_1; 
            *fn_1 = Some(fn_);
            return Some(x); 
        }
    })
    .fuse()
    .last()
    .unwrap() + 1
}

fn add(fn_1: &Vec<u8>, fn_2: &Vec<u8>) -> Vec<u8> {
    let mut carry = 0;  
    fn_1
    .iter()
    .zip(fn_2.iter())
    .map(|(&a, &b)| {
        let _x = a + b + carry;
        carry = _x/10; 
        _x % 10 
    })
    .collect::<Vec<u8>>()
}

#[test]
fn s1_test() {
    assert_eq!(4782, s1());
}

#[test]
fn s2_test() {
    assert_eq!(4782, s2());
}
