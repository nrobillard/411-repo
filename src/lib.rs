mod array2;
pub use crate::array2::Array2;


#[cfg(test)]
mod tests {
    use super::*;

        // Additional test cases for iter_col_major
        #[test]
        fn test_iter_row_major() {
            let data_cm = vec![
                1, 5, 9, 
                2, 6, 10,
                3, 7, 11,
                4, 8, 12,
            ];
            let data_rm = vec![
                1, 2, 3, 4,
                5, 6, 7, 8,
                9, 10, 11, 12,
            ];

            let width = 4;
            let height = 3;

            let array_cm = Array2::from_col_major(data_cm.clone(), height, width);
            let array_rm = Array2::from_row_major(data_rm.clone(), height, width);

            let mut iter_row_major_cm = array_cm.iter_row_major();
            let mut iter_row_major_rm = array_rm.iter_row_major();

            // Test that the iterators return the same elements, regarless of constructor
            for i in 0..height {
                for j in 0..width{
                    eprintln!("Cell num {:?}", i * width + j + 1);
                    assert_eq!(iter_row_major_cm.next(), iter_row_major_rm.next());
                }
            } 

            // Test that the iterators return None after all elements have been iterated over
            assert_eq!(iter_row_major_cm.next(), None);
            assert_eq!(iter_row_major_rm.next(), None);
        
        }

        #[test]
        fn test_iter_col_major() {
            let data_cm = vec![
                1, 5, 9, 
                2, 6, 10,
                3, 7, 11,
                4, 8, 12,
            ];
            let data_rm = vec![
                1, 2, 3, 4,
                5, 6, 7, 8,
                9, 10, 11, 12,
            ];

            let width = 4;
            let height = 3;

            let array_cm = Array2::from_col_major(data_cm.clone(), height, width);
            let array_rm = Array2::from_row_major(data_rm.clone(), height, width);

            let mut iter_col_major_cm = array_cm.iter_col_major();
            let mut iter_col_major_rm = array_rm.iter_col_major();

            // Test that the iterators return the same elements, regarless of constructor
            for i in 0..height {
                for j in 0..width{
                    let (data_cm, i_cm, j_cm) = iter_col_major_cm.next().unwrap();
                    let (data_rm, i_rm, j_rm) = iter_col_major_rm.next().unwrap();
                    eprintln!("Cell num {:?}, Data from cm {:?}, Data from rm {:?}", i * width + j + 1, data_cm, data_rm);
                    assert_eq!(data_cm, data_rm);
                    assert_eq!(i_cm, i_rm);
                    assert_eq!(j_cm, j_rm);
                }
            }

            // Test that the iterators return None after all elements have been iterated over
            assert_eq!(iter_col_major_cm.next(), None);
            assert_eq!(iter_col_major_rm.next(), None);

        }

}