mod binary_search;
mod first_bad_version;

fn main() {
    let result = binary_search::simple_search(vec![1, 2, 3, 4], 2);
    let result2 = binary_search::binary_search(vec![-1, 0, 3, 5, 9, 12], 9);
    println!("{}", result);
    println!("{}", result2);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search::simple_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(
            binary_search::simple_search(vec![-1, 0, 3, 5, 9, 12], 2),
            -1
        );
        assert_eq!(binary_search::binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(
            binary_search::binary_search(vec![-1, 0, 3, 5, 9, 12], 2),
            -1
        );
    }
}
