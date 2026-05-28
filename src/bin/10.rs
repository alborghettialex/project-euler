// # 10
// The sum of the primes below is 17.
//
// Find the sum of all the primes below two million.
//
// Answer: 142913828922

fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }

    if n == 2 {
        return true;
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
    let mut sum: u64 = 0;

    for n in 1..=2_000_000 {
        if is_prime(n) {
            sum += n;
        }
    }

    println!("{}", sum)
}
