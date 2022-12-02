use crate::Solution;
#[allow(unused_imports)]
use inpt::{Inpt, inpt};

#[derive(Inpt)]
#[inpt(regex = r"(\d+)x(\d+)x(\d+)")]
struct Present {
    l: u64,
    w: u64,
    h: u64,
}

impl Present {
    pub fn new(i: &'static str) -> Self {
        let mut i = i.split("x");
        Self {
            l: i.next().unwrap().parse().unwrap(),
            w: i.next().unwrap().parse().unwrap(),
            h: i.next().unwrap().parse().unwrap(),
        }
    }

    pub fn required_paper(self) -> u64 {
        2 * self.l * self.w
        + 2 * self.w * self.h
        + 2 * self.h * self.l
        + std::cmp::min(
            std::cmp::min(self.l * self.w,
                          self.w * self.h),
            self.h * self.l)
    }

    pub fn required_ribbon(self) -> u64 {
        2 * (self.l
             + self.w
             + self.h
             - std::cmp::max(std::cmp::max(self.l, self.w), self.h))
        + self.l * self.w * self.h
    }
}

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/02.input")
    };

    if part == 1 {
        Solution::U64(
            input
                .lines()
                // .map(|l| inpt::<Present>(l).unwrap())
                .map(|l| Present::new(l))
                .map(|p| p.required_paper())
                .sum())
    } else {
        Solution::U64(
            input
                .lines()
                // .map(|l| inpt::<Present>(l).unwrap())
                .map(|l| Present::new(l))
                .map(|p| p.required_ribbon())
                .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r"2x3x4"), Solution::U64(58));
        assert_eq!(solve(1, r"1x1x10"), Solution::U64(43));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U64(1598415));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"2x3x4"), Solution::U64(34));
        assert_eq!(solve(2, r"1x1x10"), Solution::U64(14));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::U64(3812909));
    }
}
