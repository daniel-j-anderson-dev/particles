/// row major
#[bon::builder]
pub fn serialize_index(row_index: usize, col_index: usize, width: usize) -> usize {
    row_index * width + col_index
}

/// [row, col]
#[bon::builder]
pub fn deserialize_index(index: usize, width: usize) -> [usize; 2] {
    [index / width, index % width]
}
