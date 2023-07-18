/// 00036
///
/// `T-O(1)`
///
/// `S-O(1)`
///
/// https://leetcode.cn/problems/valid-sudoku/
#[allow(unused)]
pub fn solution(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut squares = vec![HashSet::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            let value = board[i][j];
            if value == '.' {
                continue;
            }

            let index = i / 3 * 3 + j / 3;
            if rows[i].insert(value) && cols[j].insert(value) && squares[index].insert(value) {
                continue;
            }

            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        let preset_arg = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(solution(preset_arg), true);
    }

    #[test]
    fn case2() {
        let preset_arg = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(solution(preset_arg), false);
    }

    #[test]
    fn case3() {
        let preset_arg = vec![
            vec!['.', '.', '.', '8', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '6', '.', '.', '.', '.', '3', '.', '.'],
            vec!['7', '.', '.', '9', '6', '4', '1', '.', '.'],
            vec!['6', '.', '9', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '5', '.'],
            vec!['.', '.', '9', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '5'],
            vec!['.', '.', '1', '.', '.', '.', '.', '2', '.'],
        ];

        assert_eq!(solution(preset_arg), false);
    }
}
