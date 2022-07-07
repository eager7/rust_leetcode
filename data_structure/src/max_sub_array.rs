use std::cmp::Ordering;

/// 给你一个整数数组 nums ，请你找出一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
///
/// 子数组 是数组中的一个连续部分。
///
///
/// 示例 1：
///
/// 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
/// 输出：6
/// 解释：连续子数组[4,-1,2,1] 的和最大，为6 。
/// 示例 2：
///
/// 输入：nums = [1]
/// 输出：1
/// 示例 3：
///
/// 输入：nums = [5,4,-1,7,8]
/// 输出：23
///
/// 提示：
///
/// 1 <= nums.length <= 105
/// -104 <= nums[i] <= 104
///
/// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut previous = nums[0];

    nums.iter().skip(1).for_each(|&n| {
        previous = n.max(previous + n);
        max = max.max(previous);
    });
    return max;
}

#[test]
fn test_max_sub_array() {
    // assert_eq!(6, max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    // assert_eq!(1, max_sub_array(vec![1]));
    assert_eq!(2, max_sub_array(vec![-3, 1, -4, 2]));
}
