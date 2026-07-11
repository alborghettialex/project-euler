// # 14
// The following iterative sequence is defined for the set of positive integers: <br>
// n -> n/2 (n is even)<br>
// n -> 3n + 1 (n is odd)<br>
// Using the rule above and starting with 13, we generate the following sequence: <br>
// 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1. <br>
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
// Which starting number, under one million, produces the longest chain? <br>
// NOTE: Once the chain starts the terms are allowed to go above one million.
//
// Answer: 837799

use std::collections::HashMap;

fn next_collatz_term(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

fn main() {
    let mut cache = HashMap::new();
    cache.insert(1, 1);

    let mut max_len = 0;
    let mut selected = 0;

    for start in 1..1_000_000 {
        let mut path = Vec::new();
        let mut n = start;

        while !cache.contains_key(&n) {
            path.push(n);
            n = next_collatz_term(n);
        }

        let mut len = *cache.get(&n).unwrap();

        while let Some(value) = path.pop() {
            len += 1;
            cache.insert(value, len);
        }

        let chain_len = cache[&start];

        if chain_len > max_len {
            max_len = chain_len;
            selected = start;
        }
    }

    println!("{}", selected); // 837799
}
