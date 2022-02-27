#[allow(dead_code)]
pub fn simple_search(nums: Vec<i32>, target: i32) -> i32 {
    let maybe_num = nums.iter().position(|&x| x == target);
    match maybe_num {
        Some(n) => n as i32,
        None => -1,
    }
}

#[allow(dead_code)]
pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let pivot = ((left + right) / 2) as i32;
        if target == nums[pivot as usize] {
            return pivot as i32;
        }
        if target < nums[pivot as usize] {
            right = pivot - 1
        } else {
            left = pivot + 1
        }
    }
    -1
}
