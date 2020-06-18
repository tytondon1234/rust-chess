pub fn create() -> Vec<(char, u32)> {
    let cols: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let rows: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut locations: Vec<(char, u32)> = Vec::new();
    for col in cols.iter() {
        for row in rows.iter() {
            locations.push((*col, *row));
        }
    }
    locations
}
