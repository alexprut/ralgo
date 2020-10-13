pub fn quick_sort(arr: &mut [isize], start: isize, end: isize) {
    if start < end {
        let pivot = partition(arr, start, end);
        quick_sort(arr, start, pivot - 1);
        quick_sort(arr, pivot + 1, end);
    }
}

fn partition(arr: &mut [isize], start: isize, end: isize) -> isize {
    let x = arr[end]; // Pivot element
    let mut i = start - 1;
    for j in start..end {
        if arr[j] < x {
            i = i + 1;
            Utils.swap(arr, i, j);
        }
    }
    Utils.swap(arr, i + 1, end);
    return i + 1;
}