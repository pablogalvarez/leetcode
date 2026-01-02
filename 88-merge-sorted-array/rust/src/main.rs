/*
+------------------------+
| 88. Merge sorted array |
+------------------------+

You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first
m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

Example 1:
    Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
    Output: [1,2,2,3,5,6]
    Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
    The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.

Example 2:
    Input: nums1 = [1], m = 1, nums2 = [], n = 0
    Output: [1]
    Explanation: The arrays we are merging are [1] and [].
    The result of the merge is [1].

Example 3:
    Input: nums1 = [0], m = 0, nums2 = [1], n = 1
    Output: [1]
    Explanation: The arrays we are merging are [] and [1].
    The result of the merge is [1].
    Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
*/

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }
        if m == 0 {
            *nums1 = std::mem::take(nums2);
            return;
        }
        let mut i: i32 = m - 1; // before zeros in nums1
        let mut j: i32 = n - 1; // end of nums2
        let mut k: i32 = m + n - 1; // end of nums1
        while j >= 0 {
            if i >= 0 && nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_m_is_zero() {
        let mut nums1 = vec![0, 0, 0, 0];
        let m: i32 = 0;
        let mut nums2 = vec![2, 4, 6, 7];
        let n: i32 = 4;
    }

    #[test]
    fn test_n_is_zero() {
        let mut nums1 = vec![1, 2, 2];
        let m: i32 = 3;
        let mut nums2: Vec<i32> = vec![];
        let n: i32 = 0;
    }

    #[test]
    fn test_merge_1() {
        let mut nums1 = vec![5, 6, 7, 0, 0, 0];
        let m: i32 = 3;
        let mut nums2 = vec![1, 2, 2];
        let n: i32 = 3;
    }

    #[test]
    fn test_merge_2() {
        let mut nums1 = vec![1, 2, 3, 0, 0];
        let m = 3;
        let mut nums2 = vec![10, 11];
        let n: i32 = 2;
    }

    #[test]
    fn test_merge_full() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m: i32 = 3;
        let mut nums2 = vec![2, 5, 6];
        let n: i32 = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
