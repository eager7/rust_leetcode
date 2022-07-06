use std::collections::HashMap;

/// 给你一个整数数组 nums 。如果任一值在数组中出现 至少两次 ，返回 true ；如果数组中每个元素互不相同，返回 false 。
/// 示例 1：
/// 输入：nums = [1,2,3,1]
/// 输出：true
/// 示例 2：
/// 输入：nums = [1,2,3,4]
/// 输出：false
/// 示例3：
/// 输入：nums = [1,1,1,3,3,4,3,2,4,2]
/// 输出：true
///
/// 提示：
/// 1 <= nums.length <= 105
/// -109 <= nums[i] <= 109
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut m = HashMap::new();
    for n in nums.iter() {
        match m.get(n) {
            Some(&x) => return x,
            None => m.insert(n, true),
        };
    }
    return false;
}

#[test]
fn contains_duplicate_test() {
    assert_eq!(true, contains_duplicate(vec![1, 2, 3, 3]))
}
