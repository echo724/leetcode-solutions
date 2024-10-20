/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

struct Solution {}
// @lc code=start
use std::str::{CharIndices, FromStr};
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut input = x;
        let mut is_negative = false;
        if x < 0 {
            is_negative = true;
            input = x.abs();
        }
        let max = if is_negative {
            i32::MIN.to_string()[1..i32::MIN.to_string().len()].to_string()
        } else {
            i32::MAX.to_string()
        };
        let str = input.to_string();
        let reversed = str.char_indices().rev();
        let removed_zeros_reversed = reversed
            .skip_while(|(_, v)| *v == '0')
            .map(|(_, v)| v)
            .into_iter()
            .collect::<String>();

        if removed_zeros_reversed.len() > max.len() {
            return 0;
        }
        println!("original {}, max {}", removed_zeros_reversed ,max);
        if removed_zeros_reversed.len() == max.len() {
            let r = removed_zeros_reversed.clone().into_bytes();
            let m = max.clone().into_bytes();
            for i in 0..max.len() {
                if r[i] > m[i] {
                    println!("original {}, max {}", r[i] ,m[i]);
                    return 0;
                } else if r[i] < m[i]{
                    break
                }
            }
        }

        let result = i32::from_str(&removed_zeros_reversed).unwrap();
        if is_negative {
            -result
        } else {
            result
        }
    }
}
// @lc code=end

#[test]
fn test1() {
    let x = 123;
    let output = 321;
    assert_eq!(Solution::reverse(x), output);
}

#[test]
fn test2() {
    let x = -123;
    let output = -321;
    assert_eq!(Solution::reverse(x), output);
}

#[test]
fn test3() {
    let x = 120;
    let output = 21;
    assert_eq!(Solution::reverse(x), output);
}

#[test]
fn test4() {
    let x = -1563847412;
    let output = 0;
    assert_eq!(Solution::reverse(x), output);
}


#[test]
fn test5() {
    let x = -2147483412;
    let output = -2143847412;
    assert_eq!(Solution::reverse(x), output);
}

#[test]
fn test6() {
    let x = 1534236469;
    let output = 0;
    assert_eq!(Solution::reverse(x), output);
}
