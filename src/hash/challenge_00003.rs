/// 00003
///
/// `T-O(N) = N => s.len`
///
/// `S-O(|Σ|) = |Σ| => ASCII 128`
///
/// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
#[allow(unused)]
pub fn solution(s: String) -> i32 {
    let mut ret = 0;
    let mut slow = 0;
    let mut fast = 0;
    let mut hashmap = std::collections::HashMap::with_capacity(s.len());

    for c in s.chars() {
        if let Some(i) = hashmap.get(&c) {
            slow = std::cmp::max(slow, i + 1);
        }

        hashmap.insert(c, fast as i32);
        fast += 1;

        ret = std::cmp::max(ret, fast - slow);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        let ret = solution(String::from("abcabcbb"));

        assert_eq!(ret, 3);
    }

    #[test]
    fn case2() {
        let ret = solution(String::from("bbbbb"));

        assert_eq!(ret, 1);
    }

    #[test]
    fn case3() {
        let ret = solution(String::from("pwwkew"));

        assert_eq!(ret, 3);
    }

    #[test]
    fn case4() {
        let ret = solution(String::from("abba"));

        assert_eq!(ret, 2);
    }

    #[test]
    fn case5() {
        let ret = solution(String::from(""));

        assert_eq!(ret, 0);
    }
}
