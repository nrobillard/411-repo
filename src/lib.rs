#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug> Array2<T> {
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

    pub fn iter_row_major<'a>(&'a self) -> impl Iterator<Item = (T, usize, usize)> + 'a {
        self.data.iter().enumerate().flat_map(move |(i, row)| {
            row.iter().enumerate().map(move |(j, &ref elem)| (elem.clone(), i, j))
        })
    }

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
        for (elem, i, j) in self.iter_row_major() {
            println!("Element: {:?}, Row: {}, Column: {}", elem, i, j);
        }
    }

    pub fn print_col_major(&self) {
        for (elem, i, j) in self.iter_col_major() {
            println!("Element: {:?}, Row: {}, Column: {}", elem, i, j);
        }
    }
}

// Any additional functions or methods can be added outside the impl block
// Need to make sure that the input is valid for the given dimensions