extern crate time;

use time::{ precise_time_s };

pub fn start_timer() -> f64 {
    precise_time_s()
}

pub fn stop_timer(t: f64) {
    println!("time elapse {} second", precise_time_s() - t);
}