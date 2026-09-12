
fn partition(arr: &mut [i32]) -> usize {
    let pivot_value = arr[0];
    let mut left_low = 1;

    for i in 1..arr.len() {
        if arr[i] < pivot_value {
            let temp = arr[i];
            arr[i] = arr[left_low];
            arr[left_low] = temp;

            left_low += 1;
        }
    }

    let temp_left_low = arr[left_low - 1];
    arr[left_low - 1] = pivot_value;
    arr[0] = temp_left_low;

    left_low - 1
}

pub fn quick_sort(arr: &mut [i32]) {

    let lenght = arr.len();

    if lenght > 0 {
        let pivot = partition(arr);
        quick_sort(&mut arr[0..pivot]);
        quick_sort(&mut arr[pivot + 1..lenght]);
    }
}