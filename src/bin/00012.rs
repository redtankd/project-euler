/*
so slow

i=12375,t=76576500,n=576

real    3h7m11.82s
user    2h21m19.63s
sys     0m6.64s
*/
fn main() {
    for i in 1u32.. {
        let t = (1..i + 1).fold(0, |sum, x| sum + x);
        let n = (1..t + 1).filter(|&x| t % x == 0).count();
        if n > 500 {
            println!("i={},t={},n={}", i, t, n);
            break;
        }
    }
}
