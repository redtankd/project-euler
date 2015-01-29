
fn main() {
    for i in 1..500 {
        for j in i..1000 {
            let k = 1000-i-j;
            if i*i + j*j == k*k { 
                println!("{}*{}*{}={}", i, j, k, i*j*k);
            }
        }
    }
}