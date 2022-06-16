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

fn slow_partition<T: Ord>(arr: &mut [T], start: usize, end: usize) -> isize {
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

fn fast_partition<T: Ord>(arr: &mut [T], start: isize, end: isize) -> isize {
    let pivot = end as usize;
    let mut store_index = start - 1;
    let mut last_index = end;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}
