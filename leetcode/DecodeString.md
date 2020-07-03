# [394. Decode String](https://leetcode.com/problems/decode-string/)

Given an encoded string, return its decoded string.

The encoding rule is: `k[encoded_string]`, where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.

Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like `3a` or `2[4]`.

**Example 1:**

```
Input: s = "3[a]2[bc]"
Output: "aaabcbc"
```

**Example 2:**

```
Input: s = "3[a2[c]]"
Output: "accaccacc"
```

**Example 3:**

```
Input: s = "2[abc]3[cd]ef"
Output: "abcabccdcdcdef"
```

**Example 4:**

```
Input: s = "abc3[cd]xyz"
Output: "abccdcdcdxyz"
```

## My Rust Solution

```rust
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, usize)> = Vec::new();
        let mut curNum: usize = 0;
        let mut curString: String = String::new();

        for l in s.chars() {
            if l == '[' {
                stack.push((curString, curNum));
                curString = String::new();
                curNum = 0;
            } else if l == ']' {
                let (mut string, nums) = stack.pop().unwrap();
                curString = curString.repeat(nums);
                string.push_str(curString.as_str());
                curString = string
            } else if l.is_digit(10) {
                curNum = curNum * 10 + (l.to_digit(10).unwrap() as usize);
            } else {
                curString.push(l)
            }
        }

        return curString;
    }
}
```
