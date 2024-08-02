/*!
 * Data Structures and Algorithms with Adam
 *
 * Copyright (c) 2024 Adam Burucs
 *
 * MIT Licensed
 */

function binarySearch(searchArray: number[], element: number): number {
  let low = 0;
  let high = searchArray.length - 1;

  while (low <= high) {
    let mid = low + Math.round((high - low) / 2);
    if (searchArray[mid] === element) {
      return mid;
    }
    if (searchArray[mid] < element) {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return -1;
}

function printResult(searchArray: number[], element: number) {
  let result = binarySearch(searchArray, element);
  console.log("Array: ", searchArray);
  console.log("Element: ", element);
  if (result === -1) {
    console.log("Element is not in the given array.");
  } else {
    console.log("Element is in the array at index: ", result);
  }
}

function main() {
  console.log("Binary Search");
  console.log();
  let arr = [1, 3, 5, 7, 9, 11];
  let element = 3;
  printResult(arr, element);
  element = 555;
  printResult(arr, element);
}

main();
