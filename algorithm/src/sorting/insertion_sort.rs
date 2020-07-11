pub fn insertion_sort<T: Ord>(list: &mut [T]) {
    for i in 0..list.len() {
        let mut j = i;
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j, j - 1);
            j -= 1;
        }
    }
}
