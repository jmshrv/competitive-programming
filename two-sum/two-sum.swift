class Solution {
    func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
//        O(n²) solution - simply iterate over the array twice
//        for i in 0...(nums.count - 2) {
//            for j in (i + 1)...(nums.count - 1) {
//                if nums[i] + nums[j] == target {
//                    return [i, j]
//                }
//            }
//        }
        
//        O(n) solution - x + y = ans, so x - ans = y. We make a dictionary that maps numbers to their list position. If the dictionary has the key (target - numbers[i]), then the corresponding value is our other number.
//        Swift solution based off https://leetcode.com/problems/two-sum/discuss/3/Accepted-Java-O(n)-Solution
        var numbers = [Int: Int]()
        
        for i in 0...nums.count - 1 {
            let potentialResult = numbers[target - nums[i]]
            if potentialResult != nil {
                return [potentialResult!, i]
            }
            numbers[nums[i]] = i
        }
        
//         The question states that there is always 1 solution, but Swift needs a return value just in case
        return [0, 0]
    }
}

