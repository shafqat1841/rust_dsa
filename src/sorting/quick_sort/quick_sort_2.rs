fn swap(arr: &mut [i32], low: usize, high: usize) {
    let temp = arr[low];
    arr[low] = arr[high];
    arr[high] = temp;
}
fn partition(arr: &mut [i32]) -> usize {

    let pivot = arr[0];
    let mut low = 1;
    let mut high = arr.len() - 1;

    loop {
        if low >= high {
            break;
        }
        loop {
            if arr[low] > pivot || low >= high {
                break;
            }
            low += 1;
        }

        loop {
            if arr[high] <= pivot ||  high <= 0 {
                break;
            }
            high -= 1;
        }



        if low < high {
            swap(arr, low, high);
            low += 1;
            high -= 1;
        }
    }

    if arr[0] > arr[high] {
        swap(arr, 0, high);
    }

    high
}

pub fn quick_sort(arr: &mut [i32]) {
    let length = arr.len();

    if length != 0 {
        let pivot_index = partition(arr);
        quick_sort(&mut arr[0..pivot_index]);
        quick_sort(&mut arr[pivot_index + 1..length]);
    }
}
