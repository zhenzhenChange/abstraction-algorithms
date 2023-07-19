/// 00001
///
/// `T-O(N)`
///
/// `S-O(N)`
///
/// https://leetcode.cn/problems/two-sum/
#[allow(unused)]
pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::with_capacity(nums.len());

    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;

        if let Some(&cached) = map.get(&diff) {
            return vec![i as i32, cached];
        }

        map.insert(num, i as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        let mut ret = solution(vec![2, 7, 11, 15], 9);
        ret.sort();

        assert_eq!(ret, vec![0, 1]);
    }

    #[test]
    fn case2() {
        let mut ret = solution(vec![3, 2, 4], 6);
        ret.sort();

        assert_eq!(ret, vec![1, 2]);
    }

    #[test]
    fn case3() {
        let mut ret = solution(vec![3, 3], 6);
        ret.sort();

        assert_eq!(ret, vec![0, 1]);
    }
}
