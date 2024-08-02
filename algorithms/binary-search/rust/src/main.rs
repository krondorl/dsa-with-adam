/*!
 * Data Structures and Algorithms with Adam
 *
 * Copyright (c) 2024 Adam Burucs
 *
 * MIT Licensed
 */

fn binary_search<'a>(search_array: &'a [usize], element: &'a usize) -> Result<usize, &'a str> {
    let mut low = 0;
    let mut high = search_array.len() - 1;
    while low <= high {
        let mid = low + ((high - low) / 2);
        if search_array[mid] == *element {
            return Ok(mid);
        }
        if search_array[mid] < *element {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    Err("No such element in array.")
}

fn print_result<'a>(search_array: &'a [usize], element: &'a usize) {
    let result = binary_search(search_array, element);
    println!("Array: {:?}", search_array);
    println!("Element: {:?}", element);
    match result {
        Ok(idx) => println!("Element is in the array at index: {:?}", idx),
        Err(e) => println!("{e}"),
    }
}

fn main() {
    println!("Binary Search");
    println!();
    let arr: &[usize] = &[1, 3, 5, 7, 9, 11];
    let mut element = 3;
    print_result(arr, &element);
    element = 555;
    print_result(arr, &element);
}
