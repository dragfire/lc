/*
 * @lc app=leetcode id=200 lang=rust
 *
 * [200] Number of Islands
 *
 * https://leetcode.com/problems/number-of-islands/description/
 *
 * algorithms
 * Medium (46.44%)
 * Likes:    5610
 * Dislikes: 193
 * Total Accepted:    729.1K
 * Total Submissions: 1.6M
 * Testcase Example:  '[["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]'
 *
 * Given a 2d grid map of '1's (land) and '0's (water), count the number of
 * islands. An island is surrounded by water and is formed by connecting
 * adjacent lands horizontally or vertically. You may assume all four edges of
 * the grid are all surrounded by water.
 *
 * Example 1:
 *
 *
 * Input:
 * 11110
 * 11010
 * 11000
 * 00000
 *
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input:
 * 11000
 * 11000
 * 00100
 * 00011
 *
 * Output: 3
 *
 */

// @lc code=start
use std::collections::LinkedList;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let visited: Vec<bool> = Vec::with_capacity(m * n);
        let ans = 0;

        let mut Q: LinkedList<(usize, usize)> = LinkedList::new();
        Q.push_back((0, 0));

        while Q.len() != 0 {
            let dirs = vec![[0, 1], [1, 0]];
        }

        ans
    }
}

struct Solution;

fn main() {
    let grid_vec = vec!["11110", "11010", "11000", "00000"];
    let grid: Vec<Vec<char>> = grid_vec.into_iter().map(|s| s.chars().collect()).collect();
    assert_eq!(Solution::num_islands(grid), 1);
}
// @lc code=end
