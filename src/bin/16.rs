// # 16
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000
//
// Answer: 1366

fn main() {
    let mut digits = vec![1];

    for _ in 0..1000 {
        let mut carry = 0;
        for d in digits.iter_mut() {
            let val = *d * 2 + carry;
            *d = val % 10;
            carry = val / 10;
        }
        if carry > 0 {
            digits.push(carry);
        }
    }

    let sum: u32 = digits.iter().sum();
    println!("{}", sum); // 1366
}
