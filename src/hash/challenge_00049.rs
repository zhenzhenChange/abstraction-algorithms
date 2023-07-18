/// 00049
///
/// `T-O(N * K * logK)`
///
/// `S-O(N * K)`
///
/// https://leetcode.cn/problems/group-anagrams/
#[allow(unused)]
pub fn solution(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();

    for string in strs {
        // NOTE: 排序字符
        let mut chars = string.chars().collect::<Vec<_>>();
        chars.sort();

        // NOTE: 以排序重组后的字符串作为 key 存储当前字符串
        let sorted = chars.iter().collect::<String>();
        map.entry(sorted).or_insert_with(Vec::new).push(string);
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::solution;
    use crate::helpers::vec::{as_string, sort_2d};

    #[test]
    fn case1() {
        let preset_arg = as_string(vec!["eat", "tea", "tan", "ate", "nat", "bat"]);
        let preset_ret = vec![
            as_string(vec!["bat"]),
            as_string(vec!["nat", "tan"]),
            as_string(vec!["ate", "eat", "tea"]),
        ];

        assert_eq!(sort_2d(preset_ret), sort_2d(solution(preset_arg)));
    }

    #[test]
    fn case2() {
        let preset_arg = as_string(vec![""]);
        let preset_ret = vec![as_string(vec![""])];

        assert_eq!(sort_2d(preset_ret), sort_2d(solution(preset_arg)));
    }

    #[test]
    fn case3() {
        let preset_arg = as_string(vec!["a"]);
        let preset_ret = vec![as_string(vec!["a"])];

        assert_eq!(sort_2d(preset_ret), sort_2d(solution(preset_arg)));
    }
}
