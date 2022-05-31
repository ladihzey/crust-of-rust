pub mod bubble_sort;
pub mod selection_sort;

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::bubble_sort;
    use super::selection_sort;

    type SortFunction = fn(&mut [i32]) -> &[i32];

    #[test_case(bubble_sort::sort)]
    #[test_case(selection_sort::sort)]
    fn should_tolerate_empty_array(sort: SortFunction) {
        let mut initial_arr: [i32; 0] = [];
        let result = sort(&mut initial_arr);
        assert_eq!(result, []);
    }

    #[test_case(bubble_sort::sort)]
    #[test_case(selection_sort::sort)]
    fn should_tolerate_single_item_array(sort: SortFunction) {
        let mut initial_arr = [1];
        let result = sort(&mut initial_arr);
        assert_eq!(result, [1]);
    }

    #[test_case(bubble_sort::sort)]
    #[test_case(selection_sort::sort)]
    fn should_sort_array(sort: SortFunction) {
        let mut initial_arr = [123, 0, -100, 12, 25, 257, 1];
        let result = sort(&mut initial_arr);
        assert_eq!(result, [-100, 0, 1, 12, 25, 123, 257]);
    }
}