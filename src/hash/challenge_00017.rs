#![allow(unused)]

/// 00017
///
/// `T-O(3^M * 4^N) = M => 234568 + N => 79`
///
/// `S-O(3^M * 4^N) = M => 234568 + N => 79`
///
/// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
///
/// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/solution/hui-su-dui-lie-tu-jie-by-ml-zimingmeng/
pub fn solution(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let map = std::collections::HashMap::from([
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z']),
    ]);

    let mut ret = std::collections::VecDeque::from(vec![String::new()]);

    for num in digits.chars() {
        let sequence = map.get(&num).unwrap();

        for _ in ret.clone() {
            let head = ret.pop_front().unwrap();

            for c in sequence {
                ret.push_back(format!("{head}{}", c.to_string()));
            }
        }
    }

    ret.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::solution;

    fn format(vec_str: Vec<&str>) -> Vec<String> {
        vec_str
            .into_iter()
            .map(|str| String::from(str))
            .collect::<Vec<String>>()
    }

    #[test]
    fn case1() {
        let mut ret = solution(String::from("23"));
        let mut preset_ret = format(vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);

        ret.sort();
        preset_ret.sort();

        assert_eq!(ret, preset_ret);
    }

    #[test]
    fn case2() {
        let mut ret = solution(String::from(""));
        let mut preset_ret = format(vec![]);

        ret.sort();
        preset_ret.sort();

        assert_eq!(ret, preset_ret);
    }

    #[test]
    fn case3() {
        let mut ret = solution(String::from("2"));
        let mut preset_ret = format(vec!["a", "b", "c"]);

        ret.sort();
        preset_ret.sort();

        assert_eq!(ret, preset_ret);
    }
}
