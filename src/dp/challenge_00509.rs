/// 00509
///
/// `T-O(N)`
///
/// `S-O(N)`
///
/// https://leetcode.cn/problems/fibonacci-number/
#[allow(unused)]
pub fn solution(n: i32) -> i32 {
    if (n < 2) {
        return n;
    }

    let mut dp = vec![0; n as usize + 1];

    dp[0] = 0;
    dp[1] = 1;

    for i in 2..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n as usize]
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        assert_eq!(solution(2), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(solution(3), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(solution(4), 3);
    }

    #[test]
    fn case4() {
        assert_eq!(solution(0), 0);
    }
}
