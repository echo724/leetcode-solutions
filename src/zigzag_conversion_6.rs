/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

use core::num;
use std::{array, str::Chars};

struct Solution {}
// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut indices: Vec<u8> = vec![0; s.len()];
        let remainder = s.len() as i32 % (num_rows * 2 - 2);
        let quotient = s.len() as i32 / (num_rows * 2 - 2);
        let bytes = s.as_bytes().to_vec();

        let mut mapper = vec![0];
        for i in 1..num_rows {
            let mut col_num: i32 = 0;
            if i == 1 {
                col_num = quotient;
            } else {
                col_num = quotient * 2;
            }
            if i < remainder + 1 {
                col_num += 1;
            }
            // 6 - 4 = 2 / i =  1 -> 0
            if remainder > num_rows && remainder - num_rows >= num_rows - i {
                col_num += 1;
            }
            mapper.push(mapper[(i - 1) as usize] + col_num);
        }

        println!("mapper {:#?}", mapper);
        for i in 0..s.len() as i32 {
            let mut r = i % (num_rows * 2 - 2);
            let q = i / (num_rows * 2 - 2);
            let add = if r >= num_rows { 1 } else { 0 };
            r = if r >= num_rows {
                num_rows * 2 - 2 - r
            } else {
                r
            };
            let multiplier = if r == 0 || r == num_rows - 1 { 1 } else { 2 };

            let index = mapper[r as usize] + q * multiplier + add;
            println!(
                "i={},q={},r={},index={},m={},add={}",
                i, q, r, index, multiplier, add
            );
            indices[index as usize] = bytes[i as usize];
        }
        return String::from_utf8(indices).unwrap();
    }
}
// @lc code=end

#[test]
fn test1() {
    let s = "PAYPALISHIRING";
    let numRows = 3;

    assert_eq!(Solution::convert(s.to_string(), numRows), "PAHNAPLSIIGYIR");
}

#[test]
fn test2() {
    let s = "PAYPALISHIRING";
    let numRows = 4;

    assert_eq!(Solution::convert(s.to_string(), numRows), "PINALSIGYAHRPI");
}

#[test]
fn test3() {
    let s = "PAYPA";
    let numRows = 4;

    assert_eq!(Solution::convert(s.to_string(), numRows), "PAYAP");
}
