use crate::algorithms::sorting::counting_sort::counting_sort_on_digit;

pub fn radix_sort(arr: &mut [isize], max_digits: usize) {
    for i in 0..=max_digits {
        counting_sort_on_digit(arr, i);
    }
}