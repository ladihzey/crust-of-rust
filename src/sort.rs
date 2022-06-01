mod bubble_sort;
mod selection_sort;
mod insertion_sort;

pub use bubble_sort::sort as bubble_sort;
pub use selection_sort::sort as selection_sort;
pub use insertion_sort::sort as insertion_sort;

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    type SortFunction = fn(&mut [i32]) -> &[i32];

    #[test_case(bubble_sort)]
    #[test_case(selection_sort)]
    #[test_case(insertion_sort)]
    fn should_tolerate_empty_array(sort: SortFunction) {
        assert_eq!(sort(&mut []), []);
    }

    #[test_case(bubble_sort)]
    #[test_case(selection_sort)]
    #[test_case(insertion_sort)]
    fn should_tolerate_single_item_array(sort: SortFunction) {
        assert_eq!(sort(&mut [1]), [1]);
    }

    #[test_case(bubble_sort)]
    #[test_case(selection_sort)]
    #[test_case(insertion_sort)]
    fn should_sort_array(sort: SortFunction) {
        assert_eq!(
            sort(&mut [123, 0, -100, 12, 25, 257, 1]),
            [-100, 0, 1, 12, 25, 123, 257]
        );
    }
}
