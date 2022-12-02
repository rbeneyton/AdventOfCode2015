use crate::Solution;
use rustc_hash::FxHashSet;

pub fn solve(part: u8, input: &'static str) -> Solution {
    #![allow(unused)]
    let input = if input.len() > 0 {
        input
    } else {
        include_str!("../../data/03.input")
    };

    if part == 1 {
        let mut set = FxHashSet::default();
        set.reserve(input.len());
        let (mut x, mut y) = (0i16, 0i16);
        set.insert((x, y));
        for i in input.chars() {
            match i {
                '>' => x += 1,
                '<' => x -= 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => panic!(),
            }
            set.insert((x, y));
        }
        Solution::U32(set.len() as u32)
    } else {
        let mut set = FxHashSet::default();
        set.reserve(input.len());
        #[derive(Default, Clone, Copy)]
        struct Pos {
            x: i16,
            y: i16,
        }
        let mut poss = [Pos::default(); 2];
        set.insert((0, 0));
        for (idx, i) in input.chars().enumerate() {
            let p = &mut poss[idx % 2];
            match i {
                '>' => p.x += 1,
                '<' => p.x -= 1,
                '^' => p.y += 1,
                'v' => p.y -= 1,
                _ => panic!(),
            }
            set.insert((p.x, p.y));
        }
        Solution::U32(set.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(solve(1, r">"), Solution::U32(2));
        assert_eq!(solve(1, r"^>v<"), Solution::U32(4));
        assert_eq!(solve(1, r"^v^v^v^v^v"), Solution::U32(2));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve(1, ""), Solution::U32(2592));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(solve(2, r"^v"), Solution::U32(3));
        assert_eq!(solve(2, r"^>v<"), Solution::U32(3));
        assert_eq!(solve(2, r"^v^v^v^v^v"), Solution::U32(11));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(2, ""), Solution::I64(2360));
    }
}
