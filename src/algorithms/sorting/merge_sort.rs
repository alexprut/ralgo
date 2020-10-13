pub fn merge_sort(arr: &mut [isize], p: isize, r: isize) {
    if p < r {
        let q = (p + r) / 2;
        mergeSort(arr, p, q);
        mergeSort(arr, q + 1, r);
        merge(arr, p, q, r);
    }
}

pub fn merge(arr: &mut [isize], p: isize, q: isize, r: isize) {
    // p <= q < r
    let mut left: Vec<usize> = vec![];
    c.resize(q - p + 2, 0);
    let mut right: Vec<usize> = vec![];
    c.resize(q - p + 2, 0);

    left[left.length - 1] = std::usize::MAX;
    right[right.length - 1] = std::usize::MAX;

    for i in 0..left.len() {
        left[i] = arr[usize(i + p)];
    }

    for i in 0..right.len() {
        right[i] = arr[usize(i + q + 1)];
    }

    let mut i = 0;
    let mut j = 0;
    for k in p..=r {
        if left[i] < right[j] {
            arr[k] = left[i];
            i = i + 1
        } else {
            arr[k] = right[j];
            j = j + 1;
        }
    }
}