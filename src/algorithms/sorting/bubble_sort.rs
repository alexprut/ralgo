use crate::utils::swap;

pub fn bubble_sort(arr: &mut [isize]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                swap(arr, i, i - 1);
                swapped = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sorting::bubble_sort::bubble_sort;

    #[test]
    fn sort() {
        let mut arr = [5, 2, 1, 9, 0, 33, 3, 3, 0];
        let expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        bubble_sort(&mut arr);
        assert_eq!(expected, arr)
    }

    #[test]
    fn sort_inverted() {
        let mut arr = [33, 9, 5, 3, 3, 2, 1, 0, 0];
        let expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        bubble_sort(&mut arr);
        assert_eq!(expected, arr)
    }

    #[test]
    fn sort_sorted() {
        let mut expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        bubble_sort(&mut expected);
        assert_eq!(expected, expected)
    }

    #[test]
    fn sort_handle_edge_cases() {
        let mut expected = [];
        bubble_sort(&mut expected);
        assert_eq!(expected, expected);

        let mut _expected = [1];
        bubble_sort(&mut expected);
        assert_eq!(expected, expected)
    }
}