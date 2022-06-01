pub fn sort<T: Ord>(array: &mut [T]) -> &[T] {
    for i in 0..array.len() {
        let mut min = i;
        for j in i + 1..array.len() {
            if array[min] > array[j] {
                min = j;
            }
        }
        array.swap(i, min);
    }

    array
}
