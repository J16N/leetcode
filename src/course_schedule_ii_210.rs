/*
 * @lc app=leetcode id=210 lang=rust
 *
 * [210] Course Schedule II
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;
use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegree = vec![0; num_courses as usize];
        let graph: HashMap<i32, Vec<i32>> =
            prerequisites.iter().fold(HashMap::new(), |mut acc, v| {
                indegree[v[0] as usize] += 1;
                acc.entry(v[1]).or_default().push(v[0]);
                acc
            });
        let mut q: VecDeque<i32> = indegree
            .iter()
            .enumerate()
            .filter(|(_, &v)| v == 0)
            .map(|(i, _)| i as i32)
            .collect();
        let mut res = vec![];

        while !q.is_empty() {
            let course = q.pop_front().unwrap();
            res.push(course);

            if let Some(next_courses) = graph.get(&course) {
                for &next_course in next_courses {
                    indegree[next_course as usize] -= 1;
                    if indegree[next_course as usize] == 0 {
                        q.push_back(next_course);
                    }
                }
            }
        }

        if res.len() == num_courses as usize {
            res
        } else {
            vec![]
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_210() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let expected = vec![0, 1];
        assert_eq!(Solution::find_order(num_courses, prerequisites), expected);
    }

    #[test]
    fn test_2_210() {
        let num_courses = 4;
        let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let expected_1 = vec![0, 2, 1, 3];
        let expected_2 = vec![0, 1, 2, 3];
        let output = Solution::find_order(num_courses, prerequisites);
        assert!(output == expected_1 || output == expected_2);
    }

    #[test]
    fn test_3_210() {
        let num_courses = 3;
        let prerequisites = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
        let expected = vec![2, 1, 0];
        assert_eq!(Solution::find_order(num_courses, prerequisites), expected);
    }

    #[test]
    fn test_4_210() {
        let num_courses = 3;
        let prerequisites = vec![vec![1, 0], vec![1, 2], vec![0, 1]];
        let expected = vec![];
        assert_eq!(Solution::find_order(num_courses, prerequisites), expected);
    }

    #[test]
    fn test_5_210() {
        let num_courses = 4;
        let prerequisites = vec![vec![2, 0], vec![1, 0], vec![3, 1], vec![3, 2], vec![1, 3]];
        let expected = vec![];
        assert_eq!(Solution::find_order(num_courses, prerequisites), expected);
    }
}
