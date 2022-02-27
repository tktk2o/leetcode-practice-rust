mod binary_search;
mod first_bad_version;
mod search_insert_position;

fn main() {
    let result = search_insert_position::search_insert(vec![1, 3, 5, 6], 2);
    println!("{}", result);
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
