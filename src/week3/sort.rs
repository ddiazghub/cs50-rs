use std::cmp::Ordering;
use std::collections::VecDeque;
use rand::Rng;
use std::time::{Instant, Duration};

/// An indexable data type that can be sorted.
pub trait Sortable<T> {
    fn selection_sort(&mut self);
    fn bubble_sort(&mut self);
    fn merge(sortable: &mut Self, start: usize, end: usize);
    fn merge_sort(&mut self);
}

impl<T: Ord + Clone> Sortable<T> for [T] {
    fn selection_sort(&mut self) {
        let length = self.len();

        for i in 0..length {
            let mut min = i;

            for j in i..length {
                if self[j] < self[min] {
                    min = j;
                }
            }

            self.swap(i, min);
        }
    }

    fn bubble_sort(&mut self) {
        let mut end = false;
        let n = self.len() - 1;

        for i in 0..n {
            if end { break };
            end = true;

            for j in (i..n).rev() {
                if self[j + 1] < self[j] {
                    self.swap(j, j + 1);
                    end = false;
                }
            }
        }
    }

    fn merge(sortable: &mut Self, start: usize, end: usize) {
        if start < end - 1 {
            let mid = start + (end - start) / 2;
            Sortable::merge(sortable, start, mid);
            Sortable::merge(sortable, mid, end);

            let mut empty: VecDeque<usize> = VecDeque::new();
            let mut queue: VecDeque<T> = VecDeque::new();
            let mut pointer2 = mid;

            for min_ptr in start..end {
                match (queue.get(0), pointer2 < end) {
                    (Some(element), true) => if sortable[pointer2] < *element {
                        if empty.len() > 0 && min_ptr == *empty.get(0).unwrap() {
                            empty.pop_front();
                        } else {
                            empty.push_back(pointer2);
                            queue.push_back(sortable[min_ptr].clone());
                        }

                        sortable.swap(min_ptr, pointer2);
                        pointer2 += 1;
                    } else {
                        let last_empty = empty.get(0);

                        if (last_empty != None && min_ptr == *last_empty.unwrap()) {
                            empty.pop_front();
                        } else {
                            queue.push_back(sortable[min_ptr].clone());
                        }

                        sortable[min_ptr] = queue.pop_front().unwrap().clone();
                        empty.push_back(pointer2);
                    },
                    (Some(_), false) => {
                        let last_empty = empty.get(0);

                        if (last_empty != None && min_ptr == *last_empty.unwrap()) {
                            empty.pop_front();
                        } else {
                            queue.push_back(sortable[min_ptr].clone());
                        }

                        sortable[min_ptr] = queue.get(0).unwrap().clone();
                        queue.pop_front();
                    },
                    (None, true) if min_ptr < mid => if sortable[pointer2] < sortable[min_ptr] {
                        queue.push_back(sortable[min_ptr].clone());
                        sortable.swap(min_ptr, pointer2);
                        empty.push_back(pointer2);
                        pointer2 += 1;
                    },
                    (None, _) => ()
                };
            }
        };
    }

    fn merge_sort(&mut self) {
        Sortable::merge(self, 0, self.len());
    }
}

pub fn main() {
    // Loads test data.
    let mut array1: [i32; 10000] = (0..10000).collect::<Vec<i32>>().try_into().unwrap();

    //  Clones test data for each algorithm.
    rand::thread_rng().fill(&mut array1[..]);
    let mut array2 = array1.clone();
    let mut array3 = array1.clone();
    let mut array4 = array1.to_vec();
    let mut array5 = array1.clone();

    // Benchmarks each algorithm.
    let mut start = Instant::now();
    array1.selection_sort();
    println!("Selection Sort: {}s", start.elapsed().as_secs_f64());

    start = Instant::now();
    array2.bubble_sort();
    println!("Bubble Sort: {}s", start.elapsed().as_secs_f64());

    start = Instant::now();
    array3.merge_sort();
    println!("Merge Sort 1: {}s", start.elapsed().as_secs_f64());

    start = Instant::now();
    merge(&mut array4);
    println!("Merge Sort 2: {}s", start.elapsed().as_secs_f64());

    start = Instant::now();
    quicksort(&mut array5);
    println!("Quicksort: {}s", start.elapsed().as_secs_f64());
}

/// Sorts an array using quicksort.
///
/// # Arguments
/// * `array` - The array to sort.
pub fn quicksort<T: Ord + Clone>(array: &mut [T]) {
    quicksort_by(array, &|smaller, greater| smaller < greater);
}

/// Sorts an array using quicksort. With the specified comparator function.
///
/// # Arguments
/// * `array` - The array to sort.
/// * `is_smaller` - Function which specifies if the current element is smaller than the other.
pub fn quicksort_by<T: Clone, F: Fn(&T, &T) -> bool>(array: &mut [T], is_smaller: &F) {
    let length = array.len();

    if length > 1 {
        let pivot_position = quicksort_partition(array, is_smaller);
        quicksort_by(&mut array[0..pivot_position], is_smaller);
        quicksort_by(&mut array[pivot_position + 1..length], is_smaller);
    }
}

/// Partitions a quicksort array into 2 subarrays.
///
/// # Arguments
/// * `array` - The array to sort.
/// * `is_smaller` - Function which specifies if the current element is smaller than the other.
fn quicksort_partition<T: Clone, F: Fn(&T, &T) -> bool>(array: &mut [T], is_smaller: &F) -> usize {
    let length: usize = array.len();

    if length == 2 {
        if is_smaller(&array[1], &array[0]) {
            array.swap(0, 1);
        }

        return 1;
    }

    position_pivot(array, is_smaller);
    let pivot = array[length - 1].clone();
    let mut pivot_position: usize = 0;

    for i in 0..length - 1 {
        if is_smaller(&array[i], &pivot) {
            array.swap(i, pivot_position);
            pivot_position += 1;
        }
    }

    array.swap(pivot_position, length - 1);
    pivot_position
}

/// Selects a pivot and positions it at the end of the array.
///
/// # Arguments
/// * `array` - The array to sort.
/// * `is_smaller` - Function which specifies if the current element is smaller than the other.
fn position_pivot<T: Clone, F: Fn(&T, &T) -> bool>(array: &mut [T], is_smaller: &F) {
    let length: usize = array.len();
    let mid: usize = length / 2;

    let pivot_position = if is_smaller(&array[mid], &array[0]) ^ is_smaller(&array[length - 1], &array[0]) {
        0
    } else if is_smaller(&array[0], &array[mid]) ^ is_smaller(&array[length - 1], &array[mid]) {
        mid
    } else {
        length - 1
    };

    array.swap(pivot_position, length - 1);
}

/// Recursively sorts an array using the merge sort algorithm.
///
/// # Arguments
/// * `sortable` - The array to sort.
fn merge<T: Ord + Clone>(sortable: &mut Vec<T>) {
    let length = sortable.len();

    if length > 1 {
        let mid = length / 2;
        let mut half1 = (&sortable[0..mid]).to_vec();
        let mut half2 = (&sortable[mid..length]).to_vec();

        merge(&mut half1);
        merge(&mut half2);

        let (mut ptr1, mut ptr2) = (0, 0);

        for min_ptr in 0..length {
            if ptr1 >= half1.len() {
                sortable[min_ptr] = half2[ptr2].clone();
                ptr2 += 1;
            } else if ptr2 >= half2.len() {
                sortable[min_ptr] = half1[ptr1].clone();
                ptr1 += 1;
            }else if half1[ptr1] < half2[ptr2] {
                sortable[min_ptr] = half1[ptr1].clone();
                ptr1 += 1;
            } else {
                sortable[min_ptr] = half2[ptr2].clone();
                ptr2 += 1;
            }
        }
    };
}