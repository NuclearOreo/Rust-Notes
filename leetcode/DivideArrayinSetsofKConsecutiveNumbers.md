# [1296. Divide Array in Sets of K Consecutive Numbers](https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/)

Given an array of integers `nums` and a positive integer `k`, find whether it's possible to divide this array into sets of `k` consecutive numbers
Return `True` if its possible otherwise return `False`.

**Example 1:**

```
Input: nums = [1,2,3,3,4,4,5,6], k = 4
Output: true
Explanation: Array can be divided into [1,2,3,4] and [3,4,5,6].
```

**Example 2:**

```
Input: nums = [3,2,1,2,3,4,3,4,5,9,10,11], k = 3
Output: true
Explanation: Array can be divided into [1,2,3] , [2,3,4] , [3,4,5] and [9,10,11].
```

**Example 3:**

```
Input: nums = [3,3,2,2,1,1], k = 3
Output: true
```

**Example 4:**

```
Input: nums = [1,2,3,4], k = 3
Output: false
Explanation: Each array should be divided in subarrays of size 3.
```

**Constraints:**

- `1 <= nums.length <= 10^5`
- `1 <= nums[i] <= 10^9`
- `1 <= k <= nums.length`

**Note:** This question is the same as 846: https://leetcode.com/problems/hand-of-straights/

## My Rust Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        let mut count: HashMap<i32, i32> = HashMap::new();

        for num in nums.iter() {
            if count.contains_key(num) {
                count.insert(*num, *count.get(num).unwrap() + 1);
            } else {
                count.insert(*num, 1);
            }
        }

        nums.sort();

        for num in nums.iter() {
            if *count.get(num).unwrap() == 0 {
                continue;
            }

            let mut curNum = *num;
            for i in 0..k {
                if !count.contains_key(&curNum) || *count.get(&curNum).unwrap() == 0 {
                    return false;
                }

                count.insert(curNum, *count.get(&curNum).unwrap() - 1);
                curNum += 1;
            }
        }

        return true;
    }
}
```
