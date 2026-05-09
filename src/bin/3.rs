// # 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143?

fn main() {
    let mut number = 600851475143 as u64;
    let mut d = 2;

    'outer: loop {
        while number % d == 0 {
            number = number / d;
            if number == 1 {
                break 'outer;
            }
        }
        d += 1;
    }
    println!("{}", d) // 6857
}
