pub struct Matrix {
    pub data: Vec<u64>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn get(&self, row: usize, col: usize) -> u64 {
        self.data[row * self.cols + col]
    }

    pub fn add_mod(&self, other: &Matrix, q: u64) -> Matrix {
        assert_eq!((self.rows, self.cols), (other.rows, other.cols));

        let data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| (a + b) % q)
            .collect();

        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn mul_mod(&self, other: &Matrix, q: u64) -> Matrix {
        assert_eq!(self.cols, other.rows);

        let mut result_data = Vec::with_capacity(self.rows * other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum: u128 = 0;
                for k in 0..self.cols {
                    let a = self.get(i, k) as u128;
                    let b = other.get(k, j) as u128;
                    sum = (sum + a * b) % q as u128;
                }
                result_data.push(sum as u64);
            }
        }

        Matrix {
            data: result_data,
            rows: self.rows,
            cols: other.cols,
        }
    }

    pub fn get_row_sum(&self, x: usize) -> u64 {
        self.data.iter().skip(x * self.cols).take(self.cols).sum()
    }
}
