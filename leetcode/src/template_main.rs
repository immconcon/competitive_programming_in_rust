impl Solution {
    fn sort_example(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums
    }
}

struct Solution;

fn main() {
    test::test();
}

mod test {
    use super::*;

    pub fn test() {
        check(&[1, 2, 3], &[1, 2, 3]);
        check(&[1, 2, 3, 7, 8, 9], &[9, 1, 8, 2, 7, 3]);
    }

    #[track_caller]
    fn check(expected: &[i32], v: &[i32]) {
        assert_eq!(expected.to_vec(), Solution::sort_example(v.into()));
    }
}
