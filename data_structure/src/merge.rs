
/// 给你两个按 非递减顺序 排列的整数数组nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
///
/// 请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
///
/// 注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。为了应对这种情况，nums1 的初始长度为 m + n，其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。
///
///
///
/// ### 示例 1：
///
/// 输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
/// 输出：[1,2,2,3,5,6]
/// 解释：需要合并 [1,2,3] 和 [2,5,6] 。
/// 合并结果是 [1,2,2,3,5,6] ，其中斜体加粗标注的为 nums1 中的元素。
/// ### 示例 2：
///
/// 输入：nums1 = [1], m = 1, nums2 = [], n = 0
/// 输出：[1]
/// 解释：需要合并 [1] 和 [] 。
/// 合并结果是 [1] 。
/// 示例 3：
///
/// 输入：nums1 = [0], m = 0, nums2 = [1], n = 1
/// 输出：[1]
/// 解释：需要合并的数组是 [] 和 [1] 。
/// 合并结果是 [1] 。
/// 注意，因为 m = 0 ，所以 nums1 中没有元素。nums1 中仅存的 0 仅仅是为了确保合并结果可以顺利存放到 nums1 中。
///
///
/// 提示：
///
/// nums1.length == m + n
/// nums2.length == n
/// 0 <= m, n <= 200
/// 1 <= m + n <= 200
/// -109 <= nums1[i], nums2[j] <= 109
///
///
/// 进阶：你可以设计实现一个时间复杂度为 O(m + n) 的算法解决此问题吗？
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode.cn/problems/merge-sorted-array
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let n = n as usize;
    let mut idx = (nums1.len() - 1) as usize;
    let mut offset = 0;
    if idx >= n {
        offset = (idx - n) as usize;
    }
    let mut n2 = nums2.pop();
    while n2 != None {
        let x = n2.unwrap();
        if x > nums1[offset] {
            n2 = nums2.pop();
            nums1[idx] = x;
        } else {
            nums1[idx] = nums1[offset];
            if offset != 0 {
                offset -= 1;
            }
        }
        if idx != 0 {
            idx -= 1;
        }
    }
}

#[test]
fn merge_test() {
    let mut nums1 = vec![2,0];
    let mut nums2 = vec![1];
    merge2(&mut nums1, 1, &mut nums2, 1);
    assert_eq!(nums1, vec![1, 2]);

    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    merge2(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(nums1, vec![1]);

    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge2(&mut nums1, 6, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
