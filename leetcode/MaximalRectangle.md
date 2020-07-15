# [85. Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle/)

Given a 2D binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.

**Example:**

```
Input:
[
  ["1","0","1","0","0"],
  ["1","0","1","1","1"],
  ["1","1","1","1","1"],
  ["1","0","0","1","0"]
]
Output: 6
```

# My Rust Solution

```rust
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }

        let mut res = 0;
        let mut histogram: Vec<i32> = vec![ 0; (matrix[0].len() + 2) ];

        for row in matrix.iter() {
            for (i, val) in row.iter().enumerate() {
                if *val == '0' {
                    histogram[i + 1] = val.to_digit(10).unwrap() as i32;
                } else {
                    histogram[i + 1] += val.to_digit(10).unwrap() as i32;
                }
            }
            res = res.max(Self::maxInHistogram(&histogram))
        }

        return res;
    }

    pub fn maxInHistogram(hist: &Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut stack: Vec<usize> = vec![];

        for (i, val) in hist.iter().enumerate() {
            while stack.len() > 0 && hist[*stack.iter().last().unwrap()] > *val {
                let j = stack.pop().unwrap();
                let width = (i - stack[stack.len() - 1] - 1) as i32;
                res = res.max(hist[j] * width);
            }
            stack.push(i);
        }

        return res;
    }
}
```
