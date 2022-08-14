class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        if len(nums1) == 1 and len(nums2) == 1:
            return (nums1[0] + nums2[0]) / 2

        if len(nums1) == 0:
            return self.median(nums2)
        if len(nums2) == 0:
            return self.median(nums1)

        median1 = self.median(nums1)
        median2 = self.median(nums2)

        if nums2[0] < nums1[0] and nums2[-1] > nums1[-1]:
            return self.median(nums1)
        if nums1[0] < nums2[0] and nums1[-1] > nums2[-1]:
            return self.median(nums2)

        if median1 == median2:
            return median1

        if median1 > median2:
            split1 = (
                nums1[: len(nums1) // 2 + 1]
                if len(nums1) % 2 == 1
                else nums1[: len(nums1) // 2]
            )

            split2 = (
                nums2[len(nums2) // 2 + 1 :]
                if len(nums2) % 2 == 1
                else nums2[: len(nums2) // 2]
            )

            return self.findMedianSortedArrays(split1, split2)

        split1 = (
            nums1[len(nums1) // 2 + 1 :]
            if len(nums1) % 2 == 1
            else nums1[len(nums1) // 2 :]
        )

        split2 = (
            nums2[: len(nums2) // 2 + 1]
            if len(nums2) % 2 == 1
            else nums2[: len(nums2) // 2]
        )

        return self.findMedianSortedArrays(split1, split2)

    def median(self, nums: list[int]) -> float:
        length = len(nums)
        if length % 2 == 1:
            return nums[length // 2]

        return (nums[length // 2] + nums[length // 2 - 1]) / 2


if __name__ == "__main__":
    solution = Solution()

    print(solution.findMedianSortedArrays([1, 3], [2]))
    print(solution.findMedianSortedArrays([1, 2], [3, 4]))
    print(solution.findMedianSortedArrays([], [1]))
    print(solution.findMedianSortedArrays([1, 2], [-1, 3]))
    print(solution.findMedianSortedArrays([1, 2], [1, 2, 3]))
