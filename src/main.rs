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

#[cfg(test)]
mod tests {
    use crate::binary_search;
    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
