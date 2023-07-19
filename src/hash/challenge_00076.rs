/// 00076
///
/// `T-O(N)`
///
/// `S-O(N)`
///
/// https://leetcode.cn/problems/minimum-window-substring/
///
/// https://leetcode.cn/problems/minimum-window-substring/solution/zui-xiao-fu-gai-zi-chuan-by-leetcode-solution/594897
#[allow(unused)]
pub fn solution(s: String, t: String) -> String {
    let s_len = s.len();
    let t_len = t.len();

    if s.is_empty() || t.is_empty() || s_len < t_len {
        return String::new();
    }

    let mut map = std::collections::HashMap::new();
    let mut dyn_map = std::collections::HashMap::new();

    t.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

    let mut slow = 0;
    let mut kinds = 0;
    let mut range_ret = (0, s_len);

    for (fast, fast_char) in s.chars().enumerate() {
        if map.contains_key(&fast_char) {
            *dyn_map.entry(fast_char).or_insert(0) += 1;

            // NOTE: 所需的字符<个数>已满足，对字符<种类>进行递增
            dyn_map
                .get(&fast_char)
                .eq(&map.get(&fast_char))
                .then(|| kinds += 1);
        }

        // NOTE: 所需的字符<种类>已满足
        while kinds == map.len() {
            // NOTE: 根据长度更新最小子串的切片范围
            if fast - slow < range_ret.1 {
                range_ret = (slow, fast - slow);
            }

            let slow_char = s.as_bytes()[slow] as char;

            if map.contains_key(&slow_char) {
                // NOTE: 当前的字符<个数>已满足，但需先对字符<种类>进行缩减
                dyn_map
                    .get(&slow_char)
                    .eq(&map.get(&slow_char))
                    .then(|| kinds -= 1);

                *dyn_map.entry(slow_char).or_insert(0) -= 1;
            }

            slow += 1;
        }
    }

    s.get(range_ret.0..=range_ret.0 + range_ret.1)
        .map_or(String::new(), |sub_str| sub_str.to_string())
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        let preset_arg0 = String::from("ADOBECODEBANC");
        let preset_arg1 = String::from("ABC");

        assert_eq!(solution(preset_arg0, preset_arg1), String::from("BANC"));
    }

    #[test]
    fn case2() {
        let preset_arg0 = String::from("a");
        let preset_arg1 = String::from("a");

        assert_eq!(solution(preset_arg0, preset_arg1), String::from("a"));
    }

    #[test]
    fn case3() {
        let preset_arg0 = String::from("a");
        let preset_arg1 = String::from("aa");

        assert_eq!(solution(preset_arg0, preset_arg1), String::new());
    }

    #[test]
    fn case4() {
        let preset_arg0 = String::from("a");
        let preset_arg1 = String::from("b");

        assert_eq!(solution(preset_arg0, preset_arg1), String::new());
    }
}
