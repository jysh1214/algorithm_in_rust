pub fn merge_sort<T: Ord + Copy>(list: &mut [T]) {
    let len = list.len();
    if len == 1 {
        return;
    } else {
        let mid = len / 2;
        merge_sort(&mut list[0..mid]);
        merge_sort(&mut list[mid..len]);
        merge(list, 0, mid, len);
    }
}

fn merge<T: Ord + Copy>(list: &mut [T], l: usize, m: usize, r: usize) {
    let mut left: Vec<T> = vec![];
    let mut right: Vec<T> = vec![];

    for i in l..m {
        left.push(list[i]);
    }

    for i in m..r {
        right.push(list[i]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            list[k] = left[i];
            i += 1;
        } else {
            list[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        list[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        list[k] = right[j];
        j += 1;
        k += 1;
    }
}
