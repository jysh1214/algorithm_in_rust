extern crate algorithm;

#[test]
fn int_increase_order_test() {
    let mut a = [3, 2, 5, 4, 1];
    algorithm::sorting::selection_sort::selection_sort(&mut a);
    assert_eq!(a, [1, 2, 3, 4, 5]);
}

#[test]
fn str_increase_order_test() {
    let mut a = ["a", "b", "abc", "asd"];
    algorithm::sorting::selection_sort::selection_sort(&mut a);
    assert_eq!(a, ["a", "abc", "asd", "b"]);
}

#[test]
fn one_size_list_test() {
    let mut a = [0];
    algorithm::sorting::selection_sort::selection_sort(&mut a);
    assert_eq!(a, [0]);
}

#[test]
fn empty_list_test() {
    let mut a: Vec<i32> = vec![];
    algorithm::sorting::selection_sort::selection_sort(&mut a);
    assert_eq!(a, []);
}

#[test]
#[should_panic]
fn panic_test() {
    let mut a = [3, 2, 5, 4, 1];
    algorithm::sorting::selection_sort::selection_sort(&mut a);
    assert_eq!(a, [3, 2, 5, 4, 1]);
}
