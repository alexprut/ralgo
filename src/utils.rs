pub fn swap(arr: &mut [isize], i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}


pub fn max(arr: &[usize]) -> usize {
    let mut max = std::usize::MIN;
    for elem in arr {
        max = max.max(*elem)
    }
    return max;
}

pub fn get_digit_at_index(number: isize, digit_index: isize) -> isize {
    return number / 10.pow(u32(digit_index - 1)) % 10;
}

#[cfg(test)]
mod tests {
    use crate::utils::{swap, max};

    #[test]
    fn should_swap() {
        let mut xs = [4, 3, 1, 6, 5];
        swap(&mut xs, 0, 1);
        assert_eq!(3, xs[0]);
        assert_eq!(4, xs[1]);
    }

    #[test]
    fn should_find_max() {
        let xs = [1, 2, 0, 9, 5];
        assert_eq!(9, max(&xs));
    }
}