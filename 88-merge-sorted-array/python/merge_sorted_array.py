from typing import List


class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        i = m - 1
        j = n - 1
        k = m + n - 1
        while j >= 0:
            if i >= 0 and nums1[i] > nums2[j]:
                nums1[k] = nums1[i]
                i -= 1
            else:
                nums1[k] = nums2[j]
                j -= 1
            k -= 1


solution: Solution = Solution()

# test1
nums1 = [0, 0, 0]
m = 0
nums2 = [1, 2, 3]
n = 3
solution.merge(nums1, m, nums2, n)
assert nums1 == [1, 2, 3]

# test2
nums1 = [1, 2]
m = 2
nums2 = []
n = 0
solution.merge(nums1, m, nums2, n)
assert nums1 == [1, 2]

# test3
nums1 = [5, 6, 7, 0, 0, 0]
m = 3
nums2 = [1, 2, 3]
n = 3
solution.merge(nums1, m, nums2, n)
assert nums1 == [1, 2, 3, 5, 6, 7]

# test4
nums1 = [1, 2, 3, 0, 0, 0]
m = 3
nums2 = [2, 5, 6]
n = 3
solution.merge(nums1, m, nums2, n)
assert nums1 == [1, 2, 2, 3, 5, 6]
