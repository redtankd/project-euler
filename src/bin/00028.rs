#![feature(test)]

extern crate test;
extern crate project_euler;

use project_euler::*;

fn main() {
    let t = start_timer();

    println!("\nsolution:\n");
    println!("the sum of the numbers on the diagonals is {}\n", s1(1001));

    stop_timer(t);
}

fn s1(max: usize) -> usize {
    let mut pos_x = max / 2;
    let mut pos_y = max / 2;
    let mut dir_cur = 0;
    let mut matrix = vec![vec![0; max]; max];

    for i in 1..max*max+1 {
        matrix[pos_x][pos_y] = i;
        match dir_cur {
            1 => { // 1 means right. next want to down
                if matrix[pos_x][pos_y+1] == 0 {
                    dir_cur = 2;
                    pos_y = pos_y + 1;
                } else {
                    pos_x = pos_x + 1;
                }
            }
            2 => { // 2 means down. next want to left
                if matrix[pos_x-1][pos_y] == 0 {
                    dir_cur = 3;
                    pos_x = pos_x - 1;
                } else {
                    pos_y = pos_y + 1;
                }
            }
            3 => { // 3 means left. next want to up
                if matrix[pos_x][pos_y-1] == 0 {
                    dir_cur = 4;
                    pos_y = pos_y - 1;
                } else {
                    pos_x = pos_x - 1;
                }
            }
            4 => { // 4 means up. next want to right
                if matrix[pos_x+1][pos_y] == 0 {
                    dir_cur = 1;
                    pos_x = pos_x + 1;
                } else {
                    pos_y = pos_y - 1;
                }
            }
            _ => { // starting point
                dir_cur = 1;
                pos_x = pos_x + 1;
            }
        }
    }

    let mut sum = 0;
    for i in 0..max {
        sum += matrix[i][i] + matrix[max-i-1][i];
    }
    sum -= 1;
        
    return sum;
}

#[cfg(test)]
mod tests {
    use super::s1;
    use test::Bencher;

    #[test]
    fn test() {
        assert_eq!(669171001, s1(1001));
    }

    #[bench]
    fn bench_s1(b: &mut Bencher) {
        b.iter(move || s1(1001));
    }
}
