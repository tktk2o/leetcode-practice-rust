fn main() {
    let result = binary_search(vec![1, 2, 3, 4], 2);
    println!("{}", result);
}

fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let maybe_num = nums.iter().position(|&x| x == target);
    match maybe_num {
        Some(n) => n as i32,
        None => -1,
    }
}
