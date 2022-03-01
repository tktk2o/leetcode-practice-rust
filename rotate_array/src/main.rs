fn rotate(nums: &mut Vec<i32>, k: i32) {
    let length = nums.len();
    let rotation = k as usize % length;
    *nums = [&nums[length - rotation..length], &nums[..length - rotation]].concat();
}

fn main() {
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut arr, 3);
    println!("{:?}", arr);
}

#[cfg(test)]
mod tests {
    use crate::rotate;
    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![-1, -100, 3, 99];
        rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100])
    }
}
