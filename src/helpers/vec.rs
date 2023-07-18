pub fn sort_2d(mut matrix: Vec<Vec<String>>) -> Vec<Vec<String>> {
    matrix.sort_by_key(|v| v.len());
    matrix.iter_mut().for_each(|v| v.sort());

    matrix
}

pub fn as_string(str_vec: Vec<&str>) -> Vec<String> {
    str_vec
        .iter()
        .map(|&str| str.to_string())
        .collect::<Vec<String>>()
}
