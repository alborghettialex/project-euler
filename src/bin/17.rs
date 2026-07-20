// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters.
// The use of "and" when writing out numbers is in compliance with British usage.
//
// Answer: 21124

use number_to_words::number_to_words;

fn main() {
    let mut total_length = 0;
    for n in 1..=1000 {
        let s = number_to_words(n, false).replace("-", "").replace(" ", "");
        // All numbers between 101 and 999 include "and", except the exact hundreds
        // (100, 200, 300, ..., 900).
        if n > 100 && n < 1000 && n % 100 != 0 {
            total_length += "and".len();
        }
        total_length += s.len();
    }
    println!("{}", total_length); // 21124
}
