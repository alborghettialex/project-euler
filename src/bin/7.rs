// # 7
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10,001st prime number?

fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    }

    let mut d = 1;
    while d <= n.isqrt() {
        d += 1;
        if n % d == 0 {
            return false;
        }
    }
    return true;
}
fn main() {
    let mut pos = 6;
    let mut n = 13;

    loop {
        n += 2;
        if is_prime(n) {
            pos += 1;
            if pos == 10_001 {
                break;
            }
        }
    }
    println!("{}", n) // 104743
}
