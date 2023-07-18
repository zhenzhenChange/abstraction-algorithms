/// 00012
///
/// `T-O(1)`
///
/// `S-O(1)`
///
/// https://leetcode.cn/problems/integer-to-roman/
#[allow(unused)]
pub fn solution(num: i32) -> String {
    let map = vec![
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    let mut int = num;
    let mut ret = String::new();
    let mut iter = map.into_iter();

    while let Some((k, v)) = iter.next() {
        if int >= v {
            // NOTE: Rust 的整数除法会截断小数部分
            ret.push_str(&k.repeat((int / v) as usize));
            int %= v;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        assert_eq!(solution(3), "III");
    }

    #[test]
    fn case2() {
        assert_eq!(solution(4), "IV");
    }

    #[test]
    fn case3() {
        assert_eq!(solution(9), "IX");
    }

    #[test]
    fn case4() {
        assert_eq!(solution(58), "LVIII");
    }

    #[test]
    fn case5() {
        assert_eq!(solution(1994), "MCMXCIV");
    }
}
