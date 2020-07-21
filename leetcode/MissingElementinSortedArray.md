# [1060. Missing Element in Sorted Array](https://leetcode.com/problems/missing-element-in-sorted-array/)

Given a sorted array `A` of **unique** numbers, find the `K-th` missing number starting from the leftmost number of the array.

**Example 1:**

```
Input: A = [4,7,9,10], K = 1
Output: 5
Explanation:
The first missing number is 5.
```

**Example 2:**

```
Input: A = [4,7,9,10], K = 3
Output: 8
Explanation:
The missing numbers are [5,6,8,...], hence the third missing number is 8.
```

**Example 3:**

```
Input: A = [1,2,4], K = 3
Output: 6
Explanation:
The missing numbers are [3,5,6,7,...], hence the third missing number is 6.
```

**Note:**

1. `1 <= A.length <= 50000`
2. `1 <= A[i] <= 1e7`
3. `1 <= K <= 1e8`

# My Rust Solution

```rust
impl Solution {
    pub fn missing_element(nums: Vec<i32>, mut k: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut missingNumbers = nums[nums.len() - 1] - nums[0] - (nums.len() as i32) + 1;

        if missingNumbers < k {
            return nums[nums.len() - 1] + k - missingNumbers;
        }

        while left < right - 1 {
            let mid = (left + right) / 2;
            missingNumbers = nums[mid] - nums[left] - (mid - left) as i32;

            if missingNumbers >= k {
                right = mid;
            } else {
                k -= missingNumbers;
                left = mid;
            }
        }

        return nums[left] + k;
    }
}
```
