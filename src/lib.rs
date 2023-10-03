use std::fmt::Display;

pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl<T: Clone +  Display> Array2<T> {
    /// Creates an Array2, given an input vector that is in row-major order
    /// # Arguments
    /// * data: Data to be stored in the Array2 in row-major order
    /// * height: number of rows of the Array2
    /// * width: number of columns of the Array2
    /// # Returns
    /// * An Array2
    pub fn from_row_major(data: Vec<T>, height: usize, width: usize) -> Self {
        assert_eq!(height * width, data.len(), "Invalid dimensions");
        let mut rows = Vec::with_capacity(height);
        for chunk in data.chunks(width) {
            rows.push(chunk.to_vec());
        }
        Self {
            width,
            height,
            data: rows,
        }
    }

    /// Creates an Array2, given an input vector that is in column-major order
    /// # Arguments
    /// * data: Data to be stored in the Array2 in column-major order
    /// * height: number of rows of the Array2
    /// * width: number of columns of the Array2
    /// # Returns
    /// * An Array2
    pub fn from_col_major(data: Vec<T>, height: usize, width: usize) -> Self {
        assert_eq!(height * width, data.len(), "Invalid dimensions");
        let mut columns = vec![Vec::with_capacity(height); width];
        for (i, item) in data.into_iter().enumerate() {
            columns[i % width].push(item);
        }

        Self {
            width,
            height,
            data: columns,
        }
    }
    /// Iterates over the Array2 in row-major order
    /// # Arguments
    /// None
    /// Although the function does not take any arguments, it borrows the Array2
    /// <'a>: This is a lifetime parameter, indicating that the returned iterator has the same lifetime as the reference to the Array2
    ///
    /// # Returns
    /// an iterator over the Array2 in row-major order
    /// The type of this iterator is a tuple containing three elements:
    /// an element, a row index , and a column index .
    pub fn iter_row_major<'a>(&'a self) -> impl Iterator<Item = (T, usize, usize)> + 'a {
        self.data.iter().enumerate().flat_map(move |(i, row)| {
            row.iter().enumerate().map(move |(j, &ref elem)| (elem.clone(), i, j))
        })
    }

    /// Iterates over the Array2 in col-major order
    /// # Arguments
    /// None
    /// Although the function does not take any arguments, it borrows the Array2
    ///
    /// # Returns
    /// an iterator over the Array2 in row-major order
    /// The type of this iterator is a tuple containing three elements:
    /// an element, a row index , and a column index .

    pub fn iter_col_major(&self) -> impl Iterator<Item = (T, usize, usize)> + '_ {
        (0..self.width+1).flat_map(move |j| {
            (0..self.height+1).filter_map(move |i| {
                if i < self.data.len() && j < self.data[i].len() {
                    Some((self.data[i][j].clone(), i, j))
                } else {
                    None
                }
            })
        })
    }


    pub fn find(&self, row: usize, column: usize) -> T {
        self.data[row][column].clone()
    }

    pub fn print_row_major(&self) {
        for (elem, _, _) in self.iter_row_major() {
            println!("Element: {}", elem);
        }
    }

    pub fn print_col_major(&self) {
        for (elem, _, _) in self.iter_col_major() {
            println!("Element {}", elem);
        }
    }
}

// Any additional functions or methods can be added outside the impl block
// Need to make sure that the input is valid for the given dimensions