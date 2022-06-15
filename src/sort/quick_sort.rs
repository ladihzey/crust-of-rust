pub fn sort<T: Ord>(array: &mut [T]) -> &[T] {
    sort_slice(array);
    array
}

fn sort_slice<T: Ord>(slice: &mut [T]) {
    let len = slice.len();

    if len < 2 {
        return;
    }

    let partition_index = partition(slice);
    sort_slice(&mut slice[0 .. partition_index]);
    sort_slice(&mut slice[partition_index + 1 .. len])
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let end = slice.len() - 1;
    let mut index = 0;

    for i in 0..end {
        if slice[i] <= slice[end] {
            slice.swap(index, i);
            index += 1;
        }
    }

    slice.swap(index, end);
    index
}
