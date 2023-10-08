/// 00070
///
/// `T-O(N)`
///
/// `S-O(N)`
///
/// https://leetcode.cn/problems/climbing-stairs/
#[allow(unused)]
pub fn solution(n: i32) -> i32 {
    let mut dp = vec![0; n as usize + 1];

    dp[0] = 1;
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
        assert_eq!(solution(2), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(solution(3), 3);
    }

    #[test]
    fn case3() {
        assert_eq!(solution(44), 1134903170);
    }
}
