extern crate algorithm;

#[test]
fn int_increase_order_test() {
    let mut a = [3, 2, 5, 4, 1];
    algorithm::sorting::bubble_sort::bubble_sort(&mut a);
    assert_eq!(a, [1, 2, 3, 4, 5]);
}

#[test]
fn str_increase_order_test() {
    let mut a = ["a", "b", "abc", "asd"];
    algorithm::sorting::bubble_sort::bubble_sort(&mut a);
    assert_eq!(a, ["a", "abc", "asd", "b"]);
}

#[test]
#[should_panic]
fn panic_test() {
    let mut a = [3, 2, 5, 4, 1];
    algorithm::sorting::bubble_sort::bubble_sort(&mut a);
    assert_eq!(a, [3, 2, 5, 4, 1]);
}