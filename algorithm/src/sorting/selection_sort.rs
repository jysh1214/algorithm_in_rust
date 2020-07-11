pub fn selection_sort<T: Ord>(list: &mut [T]) {
    for i in 0..list.len() {
        let mut min = i;
        for j in (i+1)..list.len() {
            if list[j] < list[min] {
                min = j;
            }
        }
        list.swap(i, min);
    }
}