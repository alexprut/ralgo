use crate::utils::max;

pub fn counting_sort(arr: &mut [usize]) {
    let max = max(arr);
    let mut c: Vec<usize> = vec![];
    c.resize(max + 1, 0);
    let mut result = vec![];
    result.resize(arr.len(), 0);

    for index in 0..arr.len() {
        c[arr[index]] = c[arr[index]] + 1;
    }

    for i in 1..c.len() {
        c[i] += c[i - 1];
    }
    for j in (0..arr.len()).rev() {
        result[c[arr[j]] - 1] = arr[j];
        c[arr[j]] = c[arr[j]] - 1;
    }
    for i in 0..result.len() {
        arr[i] = result[i];
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::sorting::counting_sort::counting_sort;

    #[test]
    fn sort() {
        let mut arr = [5, 2, 1, 9, 0, 33, 3, 3, 0];
        let expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        counting_sort(&mut arr);
        assert_eq!(expected, arr)
    }

    #[test]
    fn sort_inverted() {
        let mut arr = [33, 9, 5, 3, 3, 2, 1, 0, 0];
        let expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        counting_sort(&mut arr);
        assert_eq!(expected, arr)
    }

    #[test]
    fn sort_sorted() {
        let mut expected = [0, 0, 1, 2, 3, 3, 5, 9, 33];

        counting_sort(&mut expected);
        assert_eq!(expected, expected)
    }

    #[test]
    fn sort_handle_edge_cases() {
        let mut expected = [];
        counting_sort(&mut expected);
        assert_eq!(expected, expected);

        let mut _expected = [1];
        counting_sort(&mut expected);
        assert_eq!(expected, expected)
    }
}