fn sorted_squares(arr: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; arr.len()];
    let mut left = 0;
    let mut right = arr.len() - 1;

    let mut i: usize = arr.len() - 1;
    while left <= right {
        if arr[left].abs() > arr[right].abs() {
            result[i] = arr[left] * arr[left];
            left += 1;
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            result[i] = arr[right] * arr[right];
            right -= 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    result
}

fn main() {
    let arr = vec![-7, -3, 2, 3, 11];
    let result: Vec<i32> = sorted_squares(arr);

    println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }
}
