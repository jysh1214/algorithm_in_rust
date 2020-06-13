pub fn bubble_sort<T: Ord>(list: &mut [T]) {
    if list.len() == 0 {
        return;
    }
    for i in 0..(list.len() - 1) {
        for j in 0..(list.len() - i - 1) {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}
