/*
 * @lc app=leetcode id=1971 lang=rust
 *
 * [1971] Find if Path Exists in Graph
 */

use crate::Solution;

// @lc code=start
use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let graph: HashMap<i32, Vec<i32>> = edges.iter().fold(HashMap::new(), |mut acc, edge| {
            acc.entry(edge[0]).or_default().push(edge[1]);
            acc.entry(edge[1]).or_default().push(edge[0]);
            acc
        });
        let mut visited = vec![false; n as usize];
        Self::dfs(&graph, source, destination, &mut visited)
    }

    fn dfs(
        graph: &HashMap<i32, Vec<i32>>,
        source: i32,
        destination: i32,
        visited: &mut [bool],
    ) -> bool {
        visited[source as usize] = true;

        if source == destination {
            return true;
        }

        if let Some(neighbours) = graph.get(&source) {
            for &neighbour in neighbours {
                if !visited[neighbour as usize] && Self::dfs(graph, neighbour, destination, visited)
                {
                    return true;
                }
            }
        }

        false
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1971() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;
        assert!(Solution::valid_path(n, edges, source, destination))
    }

    #[test]
    fn test_2_1971() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let source = 0;
        let destination = 5;
        assert!(!Solution::valid_path(n, edges, source, destination))
    }
}
