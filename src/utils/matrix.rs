struct Matrix {
    data: Vec<i32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn get(&self, row: usize, column: usize) -> i32 {
        self.data[row * self.cols + column]
    }

    pub fn add(&mut self, matrix: Matrix) {
        todo!()
    }
}
