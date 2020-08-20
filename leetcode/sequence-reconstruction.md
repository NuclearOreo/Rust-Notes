# [444. Sequence Reconstruction](https://leetcode.com/problems/sequence-reconstruction/)

Check whether the original sequence `org` can be uniquely reconstructed from the sequences in `seqs`. The `org` sequence is a permutation of the integers from 1 to n, with 1 ≤ n ≤ 104. Reconstruction means building a shortest common supersequence of the sequences in `seqs` (i.e., a shortest sequence so that all sequences in seqs are subsequences of it). Determine whether there is only one sequence that can be reconstructed from `seqs` and it is the `org` sequence.

**Example 1:**

```
Input: org = [1,2,3], seqs = [[1,2],[1,3]]
Output: false
Explanation: [1,2,3] is not the only one sequence that can be reconstructed, because [1,3,2] is also a valid sequence that can be reconstructed.
```

**Example 2:**

```
Input: org = [1,2,3], seqs = [[1,2]]
Output: false
Explanation: The reconstructed sequence can only be [1,2].
```

**Example 3:**

```
Input: org = [1,2,3], seqs = [[1,2],[1,3],[2,3]]
Output: true
Explanation: The sequences [1,2], [1,3], and [2,3] can uniquely reconstruct the original sequence [1,2,3].
```

**Example 4:**

```
Input: org = [4,1,5,2,6,3], seqs = [[5,2,6,3],[4,1,5,2]]
Output: true
```

**Constraints:**

- `1 <= n <= 10^4`
- `org` is a permutation of {1,2,...,n}.
- `1 <= segs[i].length <= 10^5`
- `seqs[i][j]` fits in a 32-bit signed integer.

## My Rust Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegrees: HashMap<i32, i32> = HashMap::new();

        for seq in seqs.iter() {
            for num in seq.iter() {
                graph.insert(*num, Vec::new());
                indegrees.insert(*num, 0);
            }
        }

        for seq in seqs.iter() {
            for i in 0..seq.len()-1 {
                let mut neighbors = graph.get(&seq[i]).unwrap().to_vec();
                neighbors.push(seq[i+1]);
                graph.insert(seq[i], neighbors);
                indegrees.insert(seq[i+1], indegrees.get(&seq[i+1]).unwrap() + 1);
            }
        }

        let mut queue: Vec<i32> = Vec::new();
        for (node, count) in indegrees.iter() {
            if *count == 0 {
                queue.push(*node);
            }
        }

        let mut res: Vec<i32> = Vec::new();
        while queue.len() > 0 {
            if queue.len() > 1 {
                return false;
            }

            let node = queue.remove(0);
            res.push(node);

            let neighbors = graph.get(&node).unwrap();

            for neighbor in neighbors.iter() {
                let indegree = *indegrees.get(neighbor).unwrap() - 1;
                indegrees.insert(*neighbor, indegree);
                if indegree == 0 {
                    queue.push(*neighbor);
                }
            }

        }

        return res.len() == indegrees.len() && Self::vec_compare(&res, &org);
    }

    fn vec_compare(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }

        return true;
    }
}
```
