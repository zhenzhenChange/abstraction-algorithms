#![allow(unused)]

/// 00013
///
/// `T-O(N) = N => s.len`
///
/// `S-O(1)`
///
/// https://leetcode.cn/problems/roman-to-integer/
pub fn solution(s: String) -> i32 {
    let map = std::collections::HashMap::from([
        ('M', 1000),
        ('D', 500),
        ('C', 100),
        ('L', 50),
        ('X', 10),
        ('V', 5),
        ('I', 1),
    ]);

    let mut i = 0;
    let mut ret = 0;

    let bytes = s.as_bytes();
    let bytes_len = bytes.len();

    while i < bytes_len {
        let curr_num = map.get(&(*bytes.get(i).unwrap() as char)).unwrap();
        let next_num = match bytes.get(i + 1) {
            Some(c) => map.get(&(*c as char)).unwrap(),
            None => &0,
        };

        // NOTE: 左边的值 >= 右边的值 --> 累加
        if curr_num >= next_num {
            ret += curr_num;
        } else {
            ret -= curr_num;
        }

        i += 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        assert_eq!(solution(String::from("III")), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(solution(String::from("IV")), 4);
    }

    #[test]
    fn case3() {
        assert_eq!(solution(String::from("IX")), 9);
    }

    #[test]
    fn case4() {
        assert_eq!(solution(String::from("LVIII")), 58);
    }

    #[test]
    fn case5() {
        assert_eq!(solution(String::from("MCMXCIV")), 1994);
    }
}
