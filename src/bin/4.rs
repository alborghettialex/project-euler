// # 4
// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 * 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
//
// Answer: 906609

fn is_palindrome(mut n: u32) -> bool {
    let original = n;
    let mut reversed = 0;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    original == reversed
}

fn main() {
    let mut max_pal_found = 0;
    for n in (1..=999).rev() {
        for m in (1..=n).rev() {
            let prod = n * m;
            if prod < max_pal_found {
                break;
            }
            if is_palindrome(n * m) {
                if prod > max_pal_found {
                    max_pal_found = prod;
                }
            }
        }
    }
    println!("{}", max_pal_found)
}
