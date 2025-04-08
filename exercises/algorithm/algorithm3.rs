/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T>(array: &mut [T])
where
    T: PartialOrd, // Required for comparisons
{
    let len = array.len();
    if len <= 1 {
        return; // No sorting needed for empty or single-element arrays
    }

    for i in 0..len {
        // Flag to optimize: if no swaps occur, the array is already sorted
        let mut swapped = false;

        // Last i elements are already in place
        for j in 0..(len - i - 1) {
            if array[j] > array[j + 1] {
                // Swap elements
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        // If no swapping occurred, array is sorted
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}

fn main() {
    let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
    println!("Before sorting: {:?}", vec);
    sort(&mut vec);
    println!("After sorting: {:?}", vec);
}