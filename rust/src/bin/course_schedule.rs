//! 207. Course Schedule
//! Medium | Depth-First Search | Breadth-First Search | Graph Theory | Topological Sort
//! https://leetcode.com/problems/course-schedule/
//!
//! There are a total of `numCourses` courses you have to take, labeled from `0`
//! to `numCourses - 1`. You are given an array `prerequisites` where
//! `prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that you
//! **must** take course `b<sub>i</sub>` first if you want to take course
//! `a<sub>i</sub>`.
//!
//! * For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.
//!
//! Return `true` if you can finish all courses. Otherwise, return `false`.
//!
//! **Example 1:**
//!
//! ```
//! Input: numCourses = 2, prerequisites = [[1,0]]
//! Output: true
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0. So it is possible.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
//! Output: false
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= numCourses <= 2000`
//! * `0 <= prerequisites.length <= 5000`
//! * `prerequisites[i].length == 2`
//! * `0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses`
//! * All the pairs prerequisites[i] are **unique**.

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut adj = vec![vec![]; n];
        let mut indeg = vec![0; n];

        for pre in prerequisites {
            adj[pre[1] as usize].push(pre[0] as usize);
            indeg[pre[0] as usize] += 1;
        }

        let mut queue: Vec<usize> = indeg
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == 0)
            .map(|(i, _)| i)
            .collect();

        let mut taken = 0;
        while let Some(i) = queue.pop() {
            taken += 1;
            for next in &adj[i] {
                indeg[*next] -= 1;
                if indeg[*next] == 0 {
                    queue.push(*next);
                }
            }
        }

        taken == num_courses
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, vec![vec![1,0]], true)]
    #[case(2, vec![vec![1,0],vec![0,1]], false)]
    fn cases(
        #[case] num_courses: i32,
        #[case] prerequisites: Vec<Vec<i32>>,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, Solution::can_finish(num_courses, prerequisites));
    }
}
