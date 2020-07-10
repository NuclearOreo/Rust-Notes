# [Stone Game II](https://leetcode.com/problems/stone-game-ii/)

Alex and Lee continue their games with piles of stones. There are a number of piles **arranged in a row**, and each pile has a positive integer number of stones `piles[i]`. The objective of the game is to end with the most stones.

Alex and Lee take turns, with Alex starting first. Initially, `M = 1`.

On each player's turn, that player can take **all the stones** in the **first** `X` remaining piles, where `1 <= X <= 2M`. Then, we set `M = max(M, X)`.

The game continues until all the stones have been taken.

Assuming Alex and Lee play optimally, return the maximum number of stones Alex can get.

**Example 1:**

```
Input: piles = [2,7,9,4,4]
Output: 10
Explanation:  If Alex takes one pile at the beginning, Lee takes two piles,
then Alex takes 2 piles again. Alex can get 2 + 4 + 4 = 10 piles in total.
If Alex takes two piles at the beginning, then Lee can take all three piles left.
In this case, Alex get 2 + 7 = 9 piles in total. So we return 10 since it's larger.
```

**Constraints:**

- `1 <= piles.length <= 100`
- `1 <= piles[i] <= 10 ^ 4`

## My Rust Solution

```rust
use std::collections::HashMap;
use std::cmp::min;
use std::cmp::max;

impl Solution {
    pub fn stone_game_ii(mut piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut dp: HashMap<(usize, usize), i32> = HashMap::new();

        for i in 1..n {
            piles[i] += piles[i - 1];
        }

        return Self::dfs(&piles, 0, 1, n, &mut dp);
    }

    pub fn dfs(piles: &Vec<i32>, i: usize, m: usize, n: usize, dp: &mut HashMap<(usize, usize), i32>) -> i32 {
        if i >= n {
            return 0;
        }

        if dp.contains_key(&(i, m)) {
            return dp[&(i, m)];
        }

        let mut res = 0;

        for x in 1..min(2 * m + 1, n - i + 1) {
            let mut val1 = 0;

            if i > 0 {
                val1 = piles[i - 1];
            }

            let val2 = Self::dfs(piles, i + x, max(m, x), n, dp);

            res = max(res, piles[n - 1] - val1 - val2);
        }

        dp.insert((i, m), res);
        return res;
    }
}
```
