# [Maximum Sum of Two Non-Overlapping Subarrays](https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/)

Given an array `A` of non-negative integers, return the maximum sum of elements in two non-overlapping (contiguous) subarrays, which have lengths `L` and `M`. (For clarification, the `L`-length subarray could occur before or after the `M`-length subarray.)

Formally, return the largest V for which `V = (A[i] + A[i+1] + ... + A[i+L-1]) + (A[j] + A[j+1] + ... + A[j+M-1])` and either:

- `0 <= i < i + L - 1 < j < j + M - 1 < A.length,` **or**
- `0 <= j < j + M - 1 < i < i + L - 1 < A.length`.

**Example 1:**

```
Input: A = [0,6,5,2,2,5,1,9,4], L = 1, M = 2
Output: 20
Explanation: One choice of subarrays is [9] with length 1, and [6,5] with length 2.
```

**Example 2:**

```
Input: A = [3,8,1,3,2,1,8,9,0], L = 3, M = 2
Output: 29
Explanation: One choice of subarrays is [3,8,1] with length 3, and [8,9] with length 2.
```

**Example 3:**

```
Input: A = [2,1,5,6,0,9,5,0,3,8], L = 4, M = 3
Output: 31
Explanation: One choice of subarrays is [5,6,0,9] with length 4, and [3,8] with length 3.
```

**Note:**

1. `L >= 1`
2. `M >= 1`
3. `L + M <= A.length <= 1000`
4. `0 <= A[i] <= 1000`

## Rust Solution

```rust
impl Solution {
    pub fn max_sum_two_no_overlap(mut a: Vec<i32>, l: i32, m: i32) -> i32 {
        let L: usize = l as usize;
        let M: usize = m as usize;

        for i in 1..a.len() {
            a[i] += a[i - 1];
        }

        let (mut res, mut Lmax, mut Mmax) = (a[L + M - 1], a[L - 1], a[M - 1]);

        for i in (L + M)..a.len() {
            Lmax = Lmax.max(a[i - M] - a[i - L - M]);
            Mmax = Mmax.max(a[i - L] - a[i - L - M]);
            res = res.max(Lmax + a[i] - a[i - M]).max(Mmax + a[i] - a[i - L]);
        }

        return res;
    }
}
```