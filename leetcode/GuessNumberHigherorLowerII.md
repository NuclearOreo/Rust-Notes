# [375. Guess Number Higher or Lower II](https://leetcode.com/problems/guess-number-higher-or-lower-ii/)

We are playing the Guess Game. The game is as follows:

I pick a number from **1** to **n**. You have to guess which number I picked.

Every time you guess wrong, I'll tell you whether the number I picked is higher or lower.

However, when you guess a particular number x, and you guess wrong, you pay **\$x**. You win the game when you guess the number I picked.

**Example:**

```
n = 10, I pick 8.

First round:  You guess 5, I tell you that it's higher. You pay $5.
Second round: You guess 7, I tell you that it's higher. You pay $7.
Third round:  You guess 9, I tell you that it's lower. You pay $9.

Game over. 8 is the number I picked.

You end up paying $5 + $7 + $9 = $21.
```

Given a particular **n ≥ 1**, find out how much money you need to have to guarantee a **win**.

## My Rust Solution

```rust
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize + 1;
        let mut dp = vec![ vec![0; n ]; n ];

        return Self::dfs(1, n - 1, &mut dp);
    }

    pub fn dfs(start: usize, end: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if start >= end {
            return 0;
        }

        if dp[start][end] != 0{
            return dp[start][end];
        }

        let mut res = i32::max_value();
        for i in start..(end + 1) {
            let amt = i as i32;
            let tmp = amt + Self::dfs(i + 1, end, dp).max(Self::dfs(start, i - 1, dp));
            res = res.min(tmp);
        }

        dp[start][end] = res;

        return res;
    }
}
```
