pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let pivot: i32 = (left + (right - left) / 2) as i32;
        if pivot < 0 {
            return 0;
        }
        if target == nums[pivot as usize] {
            return pivot as i32;
        }
        if target < nums[pivot as usize] {
            right = (pivot - 1) as usize
        } else {
            left = (pivot + 1) as usize
        }
    }
    left as i32
}
