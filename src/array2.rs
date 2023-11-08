use core::fmt::Debug;

pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl<T: Clone + Debug> Array2<T> {

    pub fn set_array_zero( value: T, height: usize, width: usize) -> Self {
        let mut data = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(value.clone());
            }
            data.push(row);
        }
        Self {
            width,
            height,
            data,
        }
    }
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
        let mut rows = vec![Vec::with_capacity(width); height];
        for (i, item) in data.into_iter().enumerate() {
            rows[i % height].push(item);
        }
        Self {
            width,
            height,
            data: rows,
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
    pub fn iter_row_major<'a>(&'a self) -> impl Iterator<Item=(T, usize, usize)> + 'a {
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
    /// an iterator over the Array2 in col-major order
    /// The type of this iterator is a tuple containing three elements:
    /// an element, a row index , and a column index .

    pub fn iter_col_major(&self) -> impl Iterator<Item=(T, usize, usize)> + '_ {
        (0..self.width).flat_map(move |j| {
            (0..self.height).filter_map(move |i| {
                if i < self.data.len() && j < self.data[i].len() {
                    Some((self.data[i][j].clone(), i, j))
                } else {
                    None
                }
            })
        })
    }

    /// Finds and returns the element at the specified row and column indices.
    /// # Arguments
    /// * `row`: Row index
    /// * `column`: Column index
    ///
    /// # Returns
    /// * A result conatining the element at the specified row and column indices.
    /// * A result containing an error if the row or column indices are out of bounds.
    pub fn get(&self, row: usize, column: usize) -> Result<T, String> {
        if row >= self.height || column >= self.width {
            Err("Index out of bounds".to_string())
        }
        else {
            Ok(self.data[row][column].clone())
        }
    }

    /// Inserts an element at the specified row and column indices.
    /// # Arguments
    /// * `row`: Row index
    /// * `column`: Column index
    /// 
    /// # Returns
    /// * A result containing nothing if the element was successfully inserted.
    /// * A result containing an error if the row or column indices are out of bounds.
    pub fn insert(&mut self, row: usize, column: usize, value: T) -> Result<(), String> {
        if row >= self.height || column >= self.width {
            Err("Index out of bounds".to_string())
        }
        else {
            self.data[row][column] = value;
            Ok(())
        }
    }


    /// Returns the height of the Array2.
    /// # Returns
    /// The height of the Array2.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the width of the Array2.
    /// # Returns
    /// The width of the Array2.
    pub fn width(&self) -> usize {
        self.width
    }



}