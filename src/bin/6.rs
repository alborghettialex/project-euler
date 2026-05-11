// # 6
// The sum of the squares of the first ten natural numbers is,
// 1^2 + 2^2 + ... + 10^2 = 385.
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025.
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    let mut sum_of_squares = 0;
    let mut squares_of_sum = 0;

    for i in 1..=100 {
        sum_of_squares += i32::pow(i, 2);
        squares_of_sum += i;
    }

    squares_of_sum = i32::pow(squares_of_sum, 2);
    println!("{}", squares_of_sum - sum_of_squares) // 25164150
}
