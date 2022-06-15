pub fn sort<T: Ord>(array: &mut[T]) {
    sort_slice(array, 0, array.len() as isize - 1);
}

fn sort_slice<T: Ord>(array: &mut [T], start: isize, end: isize) {
    if start < end {
        let pivot_index = fast_partition(array, start, end);
        sort_slice(array, start, pivot_index - 1);
        sort_slice(array, pivot_index + 1, end);
    }
}

fn slow_partition<T: Ord>(array: &mut [T], start: usize, end: usize) -> isize {
    let mut index = start;

    for i in start..end {
        if array[i] <= array[end] {
            array.swap(index, i);
            index += 1;
        }
    }

    array.swap(index, end);
    index as isize
}

fn fast_partition<T: Ord>(array: &mut [T], start: isize, end: isize) -> isize {
    let pivot = end as usize;
    let mut store_index = start - 1;
    let mut last_index = end;

    loop {
        store_index += 1;
        while array[store_index as usize] < array[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && array[last_index as usize] > array[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            array.swap(store_index as usize, last_index as usize);
        }
    }
    array.swap(store_index as usize, pivot as usize);
    store_index
}
