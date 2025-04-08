fn reverse_array(arr: Vec<i32>) -> Vec<i32> {
    arr.into_iter().rev().collect()
}

fn main() {
    let arr = vec![1, 4, 3, 2];
    let reversed_arr = reverse_array(arr);
    println!("{:?}", reversed_arr); //  [2, 3, 4, 1]
}
