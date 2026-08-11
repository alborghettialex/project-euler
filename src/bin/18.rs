// # 18
// <p>By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is $23$.</p>
// <p class="monospace center"><span class="red"><b>3</b></span><br><span class="red"><b>7</b></span> 4<br>
// 2 <span class="red"><b>4</b></span> 6<br>
// 8 5 <span class="red"><b>9</b></span> 3</p>
// <p>That is, $3 + 7 + 4 + 9 = 23$.</p>
// <p>Find the maximum total from top to bottom of the triangle below:</p>
// <p class="monospace center narrow_device_shrink copy_to_clipboard">75<br>
// 95 64<br>
// 17 47 82<br>
// 18 35 87 10<br>
// 20 04 82 47 65<br>
// 19 01 23 75 03 34<br>
// 88 02 77 73 07 63 67<br>
// 99 65 04 28 06 16 70 92<br>
// 41 41 26 56 83 40 80 70 33<br>
// 41 48 72 33 47 32 37 16 94 29<br>
// 53 71 44 65 25 43 91 52 97 51 14<br>
// 70 11 33 28 77 73 17 78 39 68 17 57<br>
// 91 71 52 38 17 14 91 43 58 50 27 29 48<br>
// 63 66 04 68 89 53 67 30 73 16 69 87 40 31<br>
// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23</p>
// <p class="smallest"><b>NOTE:</b> As there are only $16384$ routes, it is possible to solve this problem by trying every route. However, <a href="problem=67">Problem 67</a>, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! (c;</p>
//
// Answer: 1074

fn get_pos_to_loop(pos: usize, prev_len: usize) -> Vec<usize> {
    if pos == 0 {
        return vec![0];
    }
    if pos == prev_len {
        return vec![pos - 1];
    }
    vec![pos - 1, pos]
}

fn main() {
    let mut triangle: Vec<Vec<i32>> = vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 4, 82, 47, 65],
        vec![19, 1, 23, 75, 3, 34],
        vec![88, 2, 77, 73, 7, 63, 67],
        vec![99, 65, 4, 28, 6, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
    ];

    for i in (1..triangle.len()).rev() {
        let prev_len = triangle[i - 1].len();
        let mut sub_sums = vec![0; prev_len];

        for (j, &val) in triangle[i].iter().enumerate() {
            let pos_to_loop = get_pos_to_loop(j, prev_len);

            for pos in pos_to_loop {
                let sum = triangle[i - 1][pos] + val;
                sub_sums[pos] = sub_sums[pos].max(sum);
            }
        }

        triangle[i - 1] = sub_sums;
    }

    let max_sum = triangle[0][0];
    println!("{}", max_sum);
}
