use crate::utils::swap;

pub fn insertion_sort(arr: &mut [isize]) {
    for i in 0..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            swap(arr, j, j - 1);
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sorting::insertion_sort::insertion_sort;

    #[test]
    fn sort() {
        let mut arr = [5, 2, 1, 9, 0, 33, 3, 3, 0];
        let expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        insertion_sort(&mut arr);
        assert_eq!(expected, arr)
    }

    #[test]
    fn sort_inverted() {
        let mut arr = [33, 9, 5, 3, 3, 2, 1, 0, 0];
        let expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        insertion_sort(&mut arr);
        assert_eq!(expected, arr)
    }

    #[test]
    fn sort_sorted() {
        let mut expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        insertion_sort(&mut expected);
        assert_eq!(expected, expected)
    }

    #[test]
    fn sort_handle_edge_cases() {
        let mut expected = [];
        insertion_sort(&mut expected);
        assert_eq!(expected, expected);

        let mut _expected = [1];
        insertion_sort(&mut expected);
        assert_eq!(expected, expected)
    }
}