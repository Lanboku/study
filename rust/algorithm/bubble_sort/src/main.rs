fn main() {
    let mut arr: [i32; 5] = [5, 2, 3, 4, 1];
    for i in 0..arr.len() {
        for j in 0..arr.len() - i {
            if arr[j] < arr[j - 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}
