fn main() {
    let result = simple_search(vec![1, 2, 3, 4], 2);
    let result2 = binary_search(vec![-1, 0, 3, 5, 9, 12], 9);
    println!("{}", result);
    println!("{}", result2);
}

fn simple_search(nums: Vec<i32>, target: i32) -> i32 {
    let maybe_num = nums.iter().position(|&x| x == target);
    match maybe_num {
        Some(n) => n as i32,
        None => -1,
    }
}

fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
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

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_binary_search() {
        assert_eq!(simple_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(simple_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
