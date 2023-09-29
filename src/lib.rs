#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl<T: Clone> Array2<T> {

    /// Creates an Array2, given an input vector that is in row-major order
    /// 
    /// # Arguments
    /// * data: Data to be stored in the Array2 in row-major order
    /// * height: number of rows of the Array2
    /// * width: number of columns of the Array2
    /// 
    /// # Returns
    /// * An Array2
    pub fn from_row_major(data: Vec<T>, height: usize, width: usize) -> Self {
        todo!();
    }

    /// Creates an Array2, given an input vector that is in column-major order
    /// 
    /// # Arguments
    /// * data: Data to be stored in the Array2 in column-major order
    /// * height: number of rows of the Array2
    /// * width: number of columns of the Array2
    /// 
    /// # Returns
    /// * An Array2
    pub fn from_col_major(data: Vec<T>, height: usize, width: usize) -> Self {
        todo!();
    }

    /// Iterates over the Array2 in row-major order
    /// 
    /// # Arguments
    /// None
    /// 
    /// # Returns
    /// * An option containing a tuple of the element, row, and column
    /// 
    pub fn iter_row_major(&self) -> Option<(T, usize, usize)> {
        todo!();
    }

    /// Iterates over the Array2 in column-major order
    /// 
    /// # Arguments
    /// None
    /// 
    /// # Returns
    /// * An option containing a tuple of the element, row, and column
    /// 
    pub fn iter_col_major(&self) -> Option<(T, usize, usize)> {
        todo!();
    }

    /// Returns an element of an Array2
    /// 
    /// # Arguments
    /// * row: row of the element to be returned
    /// * column: column of the element to be returned
    /// 
    /// # Returns
    /// * An element of the Array2
    pub fn find(&self, row: usize, column: usize) -> T {
        todo!();
    }

    /// Prints out the elements of an Array2 in row-major order
    /// 
    /// # Arguments 
    pub fn print(&self) -> () {
        todo!();
    }

}


/*

- Leftovers from the creation of the lib.rs file via "cargo new --lib array2"
- Will probably be deleted once further instruction about where to put the Array2
  implementation is given (Ayman said lib.rs is typcally for tests, and that the
  implementation is typically in a separate file)

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
