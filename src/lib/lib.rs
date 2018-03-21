#![feature(conservative_impl_trait)]

extern crate time;

use time::{ precise_time_s };

pub fn start_timer() -> f64 {
    precise_time_s()
}

pub fn stop_timer(t: f64) {
    println!("time elapse {} second", precise_time_s() - t);
}

pub fn primes() -> impl Iterator<Item=u32> {
	(2..).scan(Vec::<u32>::new(), |state, x| {
        if state.iter()
        .take_while(|&y| *y <= x/2)
        .all(|&y| x % y != 0) {
            state.push(x);
            Some(x)
        } else {
            Some(0)
        }
    })
    .filter(|&x| x != 0)
}