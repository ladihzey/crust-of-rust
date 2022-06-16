pub fn sort<T: Ord>(arr: &mut[T]) {
    sort_slice(arr, 0, arr.len() as isize - 1);
}

fn sort_slice<T: Ord>(arr: &mut [T], start: isize, end: isize) {
    if start < end {
        let pivot_index = fast_partition(arr, start, end);
        sort_slice(arr, start, pivot_index - 1);
        sort_slice(arr, pivot_index + 1, end);
    }
}

fn partition<T: Ord>(arr: &mut [T], start: usize, end: usize) -> isize {
    let mut index = start;

    for i in start..end {
        if arr[i] <= arr[end] {
            arr.swap(index, i);
            index += 1;
        }
    }

    arr.swap(index, end);
    index as isize
}
