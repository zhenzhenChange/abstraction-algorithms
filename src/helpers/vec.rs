pub fn compare() {}

pub fn as_string(str_vec: Vec<&str>) -> Vec<String> {
    str_vec
        .iter()
        .map(|&str| str.to_string())
        .collect::<Vec<String>>()
}
