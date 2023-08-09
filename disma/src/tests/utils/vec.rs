pub fn assert_contains_exactly_in_any_order<T: PartialEq<T>>(vec: &[T], expected: &Vec<T>) {
    for expected_item in expected {
        assert!(vec.contains(expected_item));
    }
}
