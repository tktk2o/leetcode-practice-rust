#[allow(dead_code)]
fn isBadVersion(versions: i32) -> bool {
    if versions >= 1702766719 {
        true
    } else {
        false
    }
}

#[allow(dead_code)]
fn first_bad_version(n: i32) -> i32 {
    let (mut left, mut right) = (1, n);
    while left < right {
        let pivot = left + (right - left) / 2;
        if isBadVersion(pivot) {
            right = pivot
        } else {
            left = pivot + 1
        }
    }
    left
}
