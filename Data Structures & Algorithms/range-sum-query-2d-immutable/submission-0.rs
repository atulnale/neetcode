struct NumMatrix {
    pref: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        for i in 0..matrix.len() {
            for j in 1..matrix.get(0).unwrap().len() {
                matrix[i][j] += matrix[i][j - 1];
            }
        }
        for i in 1..matrix.len() {
            for j in 0..matrix.get(0).unwrap().len() {
                matrix[i][j] += matrix[i - 1][j];
            }
        }
        Self { pref: matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sub_count = 0;
        let mut sum = self.pref[row2 as usize][col2 as usize];
        if row1 > 0 {
            sub_count += 1;
            sum -= self.pref[(row1 - 1) as usize][col2 as usize];
        }
        if col1 > 0 {
            sub_count += 1;
            sum -= self.pref[row2 as usize][(col1 - 1) as usize];
        }
        if sub_count == 2 {
            sum += self.pref[(row1 - 1) as usize][(col1 - 1) as usize];
        }
        sum
    }
}
