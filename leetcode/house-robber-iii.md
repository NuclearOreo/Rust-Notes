# [337. House Robber III](https://leetcode.com/problems/house-robber-iii/)

The thief has found himself a new place for his thievery again. There is only one entrance to this area, called the "root." Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that "all houses in this place forms a binary tree". It will automatically contact the police if two directly-linked houses were broken into on the same night.

Determine the maximum amount of money the thief can rob tonight without alerting the police.

**Example 1:**

```
Input: [3,2,3,null,3,null,1]

     3
    / \
   2   3
    \   \
     3   1

Output: 7
Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
```

**Example 2:**

```
Input: [3,4,5,1,3,null,1]

     3
    / \
   4   5
  / \   \
 1   3   1

Output: 9
Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
```

# My Rust Solution

```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return *Self::dfs(root).iter().max().unwrap();
    }

    pub fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(n) = node {
            let (l, r) = (Self::dfs(n.borrow().left.clone()), Self::dfs(n.borrow().right.clone()));

            let rob = n.borrow().val + l[0] + r[0];
            let notRob = l[0].max(l[1]) + r[0].max(r[1]);

            return vec![notRob, rob];
        } else {
            return vec![0, 0];
        }
    }
}
```
