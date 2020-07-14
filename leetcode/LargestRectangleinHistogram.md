# [Largest Rectangle in Histogram](https://leetcode.com/problems/largest-rectangle-in-histogram/)

Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.

![Histo 1](https://assets.leetcode.com/uploads/2018/10/12/histogram.png)
<sub>Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].</sub>

![Histo 2](https://assets.leetcode.com/uploads/2018/10/12/histogram_area.png)
<sub>The largest rectangle is shown in the shaded area, which has area = 10 unit.</sub>

**Example:**

```
Input: [2,1,5,6,2,3]
Output: 10
```

## My Rust Solution

```rust
impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut stack: Vec<usize> = Vec::new();
        heights.push(0);
        heights.insert(0, 0);

        for (i, h) in heights.iter().enumerate() {
            while stack.len() > 0 && heights[*stack.iter().last().unwrap()] > *h {
                let j = stack.pop().unwrap();
                let index = i as i32;
                let width = stack[stack.len() - 1] as i32;
                let size = (index - width - 1) * heights[j];
                res = res.max(size);
            }
            stack.push(i);
        }

        return res;
    }
}
```
