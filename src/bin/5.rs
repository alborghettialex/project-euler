// # 5
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible with no remainder by all of the numbers from 1 to 20?

fn divisible_1_20(n: u32) -> bool {
    n % 19 == 0
        && n % 18 == 0
        && n % 17 == 0
        && n % 16 == 0
        && n % 15 == 0
        && n % 14 == 0
        && n % 13 == 0
        && n % 11 == 0
}

fn main() {
    let mut min_n_found = 2520;
    loop {
        min_n_found += 2520;
        if divisible_1_20(min_n_found) {
            break;
        }
    }
    println!("{}", min_n_found) // 232792560
}
