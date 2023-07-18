/// 00041
///
/// `T-O(N)`
///
/// `S-O(1)`
///
/// https://leetcode.cn/problems/first-missing-positive/
#[allow(unused)]
pub fn solution(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();

    for idx in 0..len {
        while (nums[idx] > 0
            && (nums[idx] as usize) <= len
            && nums[(nums[idx] - 1) as usize] != nums[idx])
        {
            let next_idx = (nums[idx] - 1) as usize;
            nums.swap(idx, next_idx);
        }
    }

    let ret = |idx| idx as i32 + 1;
    (0..len)
        .find_map(|idx| (nums[idx] != ret(idx)).then(|| ret(idx)))
        .unwrap_or_else(|| ret(len))
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        assert_eq!(solution(vec![1, 2, 0]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(solution(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(solution(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn case4() {
        assert_eq!(solution(vec![1]), 2);
    }
}
