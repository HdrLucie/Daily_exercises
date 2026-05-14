fn is_good(mut nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return false;
    }
    nums.sort();
    let nums_len = nums.len() - 1;

    // for (i, value) in nums[..nums_len].iter().enumerate() {
    // for (i, value) in nums.iter().enumerate().take(nums_len) {
    //     if i as i32 + 1 == *value {
    //         continue;
    //     }
    //     return false;
    // }
    // nums[nums_len] == nums[nums_len - 1]
    nums[nums_len] == nums[nums_len - 1]
        && nums
            .into_iter()
            .enumerate()
            .take(nums_len)
            .all(|(i, value)| i as i32 + 1 == value)
}

fn main() {
    let vec1 = vec![1, 2, 3];
    println!("{}", is_good(vec1));
}

// Tests are executed concurrently (it's magic!)

#[test]
fn case_1() {
    assert!(!is_good(vec![2, 1, 3]));
}

#[test]
fn case_2() {
    assert!(is_good(vec![1, 3, 3, 2]));
}

#[test]
fn case_3() {
    assert!(is_good(vec![1, 1]));
}

#[test]
fn case_4() {
    assert!(!is_good(vec![3, 4, 4, 1, 2, 1]));
}
