fn main() {
    let unsorted: [i32; 5] = [-2, 45, 0, 11, -9];
    println!("{:?}", bubble_sort(unsorted));
}

// Bubble Sort Algorithm for 5 numbers
fn bubble_sort (unsorted_list: [i32; 5]) -> [i32; 5] {
    let mut temp_list: [i32; 5] = unsorted_list;

    if temp_list.len() == 5 {
        for _i in 0..temp_list.len() - 1 {   
            for j in 0..temp_list.len() - 1 {
                // swap
                if temp_list[j] > temp_list[j + 1] {
                    temp_list.swap(j, j+1);
                }
            }
        }
        return temp_list;
    }
    return unsorted_list;
}