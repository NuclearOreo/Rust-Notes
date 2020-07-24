# [1477. Find Two Non-overlapping Sub-arrays Each With Target Sum](https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/)

Given an array of integers `arr` and an integer `target`.

You have to find two non-overlapping **sub-arrays** of `arr` each with sum equal `target`. There can be multiple answers so you have to find an answer where the sum of the lengths of the two sub-arrays is **minimum**.

Return the _minimum sum of the lengths_ of the two required sub-arrays, or return `-1` if you cannot find such two sub-arrays.

**Example 1:**

```
Input: arr = [3,2,2,4,3], target = 3
Output: 2
Explanation: Only two sub-arrays have sum = 3 ([3] and [3]). The sum of their lengths is 2.
```

**Example 2:**

```
Input: arr = [7,3,4,7], target = 7
Output: 2
Explanation: Although we have three non-overlapping sub-arrays of sum = 7 ([7], [3,4] and [7]), but we will choose the first and third sub-arrays as the sum of their lengths is 2.
```

**Example 3:**

```
Input: arr = [4,3,2,6,2,3,4], target = 6
Output: -1
Explanation: We have only one sub-array of sum = 6.
```

**Example 4:**

```
Input: arr = [5,5,4,4,5], target = 3
Output: -1
Explanation: We cannot find a sub-array of sum = 3.
```

**Example 5:**

```
Input: arr = [3,1,1,1,5,1,2,1], target = 3
Output: 3
Explanation: Note that sub-arrays [1,2] and [2,1] cannot be an answer because they overlap.
```

**Constraints:**

- `1 <= arr.length <= 10^5`
- `1 <= arr.length <= 10^5`
- `1 <= target <= 10^8`

## My Rust Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let (mut sum, mut res, mut lsize) = (0, std::i32::MAX, std::i32::MAX);
        let mut prefixSum: HashMap<i32, i32> = HashMap::new();

        prefixSum.insert(0, -1);

        for (i, val) in arr.iter().enumerate() {
            sum += *val;
            prefixSum.insert(sum, i as i32);
        }

        sum = 0;
        for (i, val) in arr.iter().enumerate() {
            sum += *val;
            let index = i as i32;

            if prefixSum.contains_key(&(sum - target)) {
                lsize = lsize.min(index - *prefixSum.get(&(sum - target)).unwrap());
            }

            if prefixSum.contains_key(&(sum + target)) && lsize != std::i32::MAX {
                let rsize = *prefixSum.get(&(sum + target)).unwrap() - index;
                res = res.min(rsize + lsize);
            }
        }

        return if res == std::i32::MAX { -1 } else { res }
    }
}
```
