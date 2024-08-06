/// Two Sum
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// Example 1:
///     Input: nums = [2,7,11,15], target = 9
///     Output: [0,1]
///     Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
///
/// Example 2:
///     Input: nums = [3,2,4], target = 6
///     Output: [1,2]
///
/// Example 3:
///     Input: nums = [3,3], target = 6
///     Output: [0,1]
///
/// Constraints:
///     2 <= nums.length <= 104
///     -10^9 <= nums[i] <= 109
///     -10^9 <= target <= 109
///     Only one valid answer exists.
///

struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let result = Vec::new();

        for (outer_index, &num) in nums.iter().enumerate() {
            for inner_index in outer_index + 1..nums.len() {
                let sum = num + nums[inner_index];
                if sum == target {
                    return [outer_index as i32, inner_index as i32].to_vec();
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = [2, 7, 11, 15].into();
        let target = 9i32;
        assert_eq!(Solution::two_sum(nums, target), [0, 1].to_vec());
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = [3, 2, 4].into();
        let target = 6i32;
        assert_eq!(Solution::two_sum(nums, target), [1, 2].to_vec());
    }

    #[test]
    fn test_case_3() {
        let nums: Vec<i32> = [3, 3].into();
        let target = 6i32;
        assert_eq!(Solution::two_sum(nums, target), [0, 1].to_vec());
    }
}
