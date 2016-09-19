// the solution is very slow. time elapse 6261.09284 second

extern crate project_euler;

use project_euler::*;

const GRID_DIM: u32 = 20;

fn main() {
    let t = start_timer();

    let mut count: u64 = 0;
    f(0, 0, &mut count);

    println!("count = {}", count);

    stop_timer(t);
}

fn f(x: u32, y: u32, count: &mut u64) {
    match (x, y) {
        (GRID_DIM, GRID_DIM) => *count += 1,
        (GRID_DIM, _       ) => f(x, y+1, count),
        (_,        GRID_DIM) => f(x+1, y, count),
        _ => {
            f(x+1, y, count);
            f(x, y+1, count);
        }
    }
}