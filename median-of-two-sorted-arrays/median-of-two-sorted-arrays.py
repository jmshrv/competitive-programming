import sys


class Solution:
    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        # https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/2471/Very-concise-O(log(min(MN)))-iterative-solution-with-detailed-explanation
        # See commit history for my attempt, this question was very hard 🙃
        N1, N2 = len(nums1), len(nums2)

        if N1 < N2:
            return self.findMedianSortedArrays(nums2, nums1)

        lo, hi = 0, N2 * 2

        while lo <= hi:
            mid2 = (lo + hi) // 2
            mid1 = N1 + N2 - mid2

            L1 = -sys.maxsize - 1 if mid1 == 0 else nums1[int(mid1 - 1) // 2]
            L2 = -sys.maxsize - 1 if mid2 == 0 else nums2[int(mid2 - 1) // 2]
            R1 = sys.maxsize if mid1 == N1 * 2 else nums1[int(mid1) // 2]
            R2 = sys.maxsize if mid2 == N2 * 2 else nums2[int(mid2) // 2]

            if L1 > R2:
                lo = mid2 + 1
            elif L2 > R1:
                hi = mid2 - 1
            else:
                return (max(L1, L2) + min(R1, R2)) / 2


if __name__ == "__main__":
    solution = Solution()

    print(solution.findMedianSortedArrays([1, 3], [2]))
    print(solution.findMedianSortedArrays([1, 2], [3, 4]))
