/// 00049
///
/// `T-O()`
///
/// `S-O()`
///
/// https://leetcode.cn/problems/group-anagrams/
#[allow(unused)]
pub fn solution(strs: Vec<String>) -> Vec<Vec<String>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::solution;
    use crate::helpers::vec::as_string;

    #[test]
    fn case1() {
        let preset_arg = as_string(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);
        let preset_ret = vec![
            as_string(vec!["bat"]),
            as_string(vec!["nat", "tan"]),
            as_string(vec!["ate", "eat", "tea"]),
        ];

        assert_eq!(solution(preset_arg), preset_ret);
    }

    #[test]
    fn case2() {
        let preset_arg = as_string(vec![""]);
        let preset_ret = vec![as_string(vec![""])];

        assert_eq!(solution(preset_arg), preset_ret);
    }

    #[test]
    fn case3() {
        let preset_arg = as_string(vec!["a"]);
        let preset_ret = vec![as_string(vec!["a"])];

        assert_eq!(solution(preset_arg), preset_ret);
    }
}
