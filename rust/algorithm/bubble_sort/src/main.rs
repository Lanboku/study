fn main() {
    let mut arr: [i32; 5] = [5, 2, 3, 4, 1];
    bubble_sort(&mut arr);
}

fn bubble_sort(array: &mut [i32]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                let tmp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = tmp;
            }
        }
    }

    println!("{:?}", array);
}
