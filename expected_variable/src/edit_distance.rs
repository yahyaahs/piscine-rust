pub fn edit_distance(source: &str, target: &str) -> usize {
    let l1 = source.len();
    let l2 = target.len();
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    // if l1 ==0 || l2 ==0 {
    //     return 0;
    // }

    let mut matrix = vec![vec![0; l2 + 1]; l1 + 1];

    for i in 0..=l1 {
        for j in 0..=l2 {
            matrix[i][j] = if i == 0 {
                j
            } else if j == 0 {
                i
            } else if source_chars[i - 1] == target_chars[j - 1] {
                matrix[i - 1][j - 1]
            } else {
                1 + matrix[i - 1][j - 1].min(matrix[i][j - 1]).min(matrix[i - 1][j])
            };
        }
    }

    matrix[l1][l2]
}