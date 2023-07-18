/// 00073
///
/// `T-O(M * N)`
///
/// `S-O(M + N)`
///
/// https://leetcode.cn/problems/set-matrix-zeroes/
#[allow(unused)]
pub fn solution(matrix: &mut Vec<Vec<i32>>) {
    let len = matrix.len();
    let sub_len = matrix[0].len();

    let mut row_zero = vec![false; len];
    let mut col_zero = vec![false; sub_len];

    for row_idx in 0..len {
        for col_idx in 0..sub_len {
            if matrix[row_idx][col_idx] == 0 {
                row_zero[row_idx] = true;
                col_zero[col_idx] = true;
            }
        }
    }

    for i in 0..len {
        for j in 0..sub_len {
            if (row_zero[i] || col_zero[j]) {
                matrix[i][j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn case1() {
        let preset_ret = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];

        let mut preset_arg = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        solution(&mut preset_arg);

        assert_eq!(preset_arg, preset_ret);
    }

    #[test]
    fn case2() {
        let preset_ret = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];

        let mut preset_arg = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        solution(&mut preset_arg);

        assert_eq!(preset_arg, preset_ret);
    }
}
