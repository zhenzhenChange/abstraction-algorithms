/// TODO
///
/// 00030
///
/// `T-O(Timeout)`
///
/// `S-O(N)`
///
/// https://leetcode.cn/problems/substring-with-concatenation-of-all-words/
///
/// https://leetcode.cn/problems/substring-with-concatenation-of-all-words/solution/xiang-xi-tong-su-de-si-lu-fen-xi-duo-jie-fa-by-w-6/
#[allow(unused)]
pub fn solution(s: String, words: Vec<String>) -> Vec<i32> {
    let len = s.len();
    let size = words[0].len();
    let str_size = size * words.len();

    // NOTE: 长度必须大于等于子串长度
    if len < str_size {
        return vec![];
    }

    let mut i = 0;
    let mut ret = vec![];
    let count_map = statistics(words);

    // NOTE: 遍历到最后一个子串的起始位置即可
    while i <= len - str_size {
        let sub_str = s.get(i..(i + str_size)).unwrap();

        // NOTE: 将子串分组成 size 大小的单词数组
        let sequence = sub_str.chars().collect::<Vec<char>>();
        let normalized = sequence
            .chunks(size)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>();

        // NOTE: 直接 eq 两个 map 结构是否相等
        if count_map.eq(&statistics(normalized)) {
            ret.push(i as i32);
        }

        i += 1;
    }

    ret
}

// NOTE: 用于统计单词出现的次数
fn statistics(words: Vec<String>) -> std::collections::HashMap<String, i32> {
    let mut count_map = std::collections::HashMap::new();

    for word in words {
        let count = match count_map.get(&word) {
            Some(cached) => cached + 1,
            None => 1,
        };

        count_map.insert(word, count);
    }

    count_map
}

#[cfg(test)]
mod tests {
    use super::solution;
    use crate::helpers::vec::as_string;

    #[test]
    fn case1() {
        let preset_arg0 = String::from("barfoothefoobarman");
        let preset_arg1 = as_string(vec!["foo", "bar"]);

        let mut preset_ret = vec![0, 9];
        let mut solution_ret = solution(preset_arg0, preset_arg1);

        preset_ret.sort();
        solution_ret.sort();

        assert_eq!(preset_ret, solution_ret);
    }

    #[test]
    fn case2() {
        let preset_arg0 = String::from("wordgoodgoodgoodbestword");
        let preset_arg1 = as_string(vec!["word", "good", "best", "word"]);

        let mut preset_ret: Vec<i32> = vec![];
        let mut solution_ret = solution(preset_arg0, preset_arg1);

        preset_ret.sort();
        solution_ret.sort();

        assert_eq!(preset_ret, solution_ret);
    }

    #[test]
    fn case3() {
        let preset_arg0 = String::from("barfoofoobarthefoobarman");
        let preset_arg1 = as_string(vec!["bar", "foo", "the"]);

        let mut preset_ret = vec![6, 9, 12];
        let mut solution_ret = solution(preset_arg0, preset_arg1);

        preset_ret.sort();
        solution_ret.sort();

        assert_eq!(preset_ret, solution_ret);
    }

    #[test]
    fn case4() {
        let preset_arg0 = String::from("a");
        let preset_arg1 = as_string(vec!["a", "a"]);

        let mut preset_ret: Vec<i32> = vec![];
        let mut solution_ret = solution(preset_arg0, preset_arg1);

        preset_ret.sort();
        solution_ret.sort();

        assert_eq!(preset_ret, solution_ret);
    }
}
