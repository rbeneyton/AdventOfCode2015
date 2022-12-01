use crate::Solution;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/01.input")
    };

    if part == 1 {
        let mut floor = 0;
        for c in input.chars() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!(),
            }
        }
        Solution::I64(floor)
    } else {
        let mut floor = 0;
        for (idx, c) in input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => panic!(),
            }
            if floor == -1 {
                return Solution::I64(idx as i64 + 1);
            }
        }
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::I64(74));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::I64(1795));
    }
}
