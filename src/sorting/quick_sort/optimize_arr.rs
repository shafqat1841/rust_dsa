pub fn optimize(arr: &mut [i32]){
    let low: usize = 0;
        let high: usize = arr.len() - 1;

        let mid = (high) / 2;

        if arr[low] > arr[mid] {
            let temp = arr[low];
            arr[low] = arr[mid];
            arr[mid] = temp;
        }

        if arr[low] > arr[high] {
            let temp = arr[low];
            arr[low] = arr[high];
            arr[high] = temp;
        }

        if arr[mid] > arr[high] {
            let temp = arr[mid];
            arr[mid] = arr[high];
            arr[high] = temp;
        }

        let temp = arr[low];
        arr[low] = arr[mid];
        arr[mid] = temp;
}
