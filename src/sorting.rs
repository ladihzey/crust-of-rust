mod bubble_sort;
mod selection_sort;
mod insertion_sort;
mod quick_sort;

pub use bubble_sort::sort as bubble_sort;
pub use selection_sort::sort as selection_sort;
pub use insertion_sort::sort as insertion_sort;
pub use quick_sort::sort as quick_sort;

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    type SortFunction = fn(&mut [i32]);

    #[test_case(bubble_sort)]
    #[test_case(selection_sort)]
    #[test_case(insertion_sort)]
    #[test_case(quick_sort)]
    fn should_tolerate_empty_array(sort: SortFunction) {
        let mut array = [];
        sort(&mut array);
        assert_eq!(array, []);
    }

    #[test_case(bubble_sort)]
    #[test_case(selection_sort)]
    #[test_case(insertion_sort)]
    #[test_case(quick_sort)]
    fn should_tolerate_single_item_array(sort: SortFunction) {
        let mut array = [1];
        sort(&mut array);
        assert_eq!(array, [1]);
    }

    #[test_case(bubble_sort)]
    #[test_case(selection_sort)]
    #[test_case(insertion_sort)]
    #[test_case(quick_sort)]
    fn should_sort_array(sort: SortFunction) {
        let mut array = [123, 0, -100, 12, 25, 257, 1];
        sort(&mut array);
        assert_eq!(array, [-100, 0, 1, 12, 25, 123, 257]);
    }
}
