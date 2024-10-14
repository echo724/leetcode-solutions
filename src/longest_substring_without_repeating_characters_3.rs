/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start

// abcd a
struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars = s.as_bytes().to_vec();
        let mut min_index = 0;
        let mut max = 0;
        for (i, v) in chars.iter().enumerate() {
            if let Some((x, v)) = chars[min_index..i]
                .iter()
                .enumerate()
                .find(|(x, value)| value == &v)
            {
                if ((i - min_index) > max) {
                    max = i - min_index;
                }
                min_index = x + min_index + 1;
            }
        }

        if s.len() - min_index > max {
            return (s.len() - min_index) as i32;
        }
        return max as i32;
    }
}
// @lc code=end

#[test]
fn test1() {
    let s = "abcabcbb";
    assert_eq!(Solution::length_of_longest_substring(s.to_string()), 3);
}

#[test]
fn test2() {
    let s = "bbbbb";
    assert_eq!(Solution::length_of_longest_substring(s.to_string()), 1);
}

#[test]
fn test3() {
    let s = "pwwkew";
    assert_eq!(Solution::length_of_longest_substring(s.to_string()), 3);
}

#[test]
fn test4() {
    let s = " ";
    assert_eq!(Solution::length_of_longest_substring(s.to_string()), 1)
}
