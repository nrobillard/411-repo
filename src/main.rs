use array2::Array2;

fn main() {
    // Example data
    let data_row_major = vec![1, 2, 3, 4, 5, 6];
    let height = 2;
    let width = 3;

    // Create Array2 from row-major data
    let array_row_major = Array2::from_row_major(data_row_major.clone(), height, width);

    // Print the row-major representation
    println!("Row-Major Order:");
    array_row_major.print_row_major();


    // Create Array2 from column-major data
    let array_col_major = Array2::from_col_major(data_row_major, height, width);

    // Print the column-major representation
    println!("Column-Major Order:");
    array_col_major.print_col_major();


    /*
    // Test iter_row_major
    println!("Iterating in Row-Major Order:");
    for (elem, row, col) in array_row_major.iter_row_major() {
        println!("Element: {:?}, Row: {}, Column: {}", elem, row, col);
    }

    // Test find
    let target_row = 1;
    let target_col = 2;
    let found_element = array_row_major.find(target_row, target_col);
    println!(
        "Element at row {} and column {}: {:?}",
        target_row, target_col, found_element
    );


    // Test iter_col_major
    println!("Iterating in Column-Major Order:");
    for (elem, row, col) in array_col_major.iter_col_major() {
        println!("Element: {:?}, Row: {}, Column: {}", elem, row, col);
    }

    */

}