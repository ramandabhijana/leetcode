/// Longest Consecutive Sequence
/// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
/// You must write an algorithm that runs in O(n) time.
///
/// Example 1:
///     Input: nums = [100,4,200,1,3,2]
///     Output: 4
///     Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
///
/// Example 2:
///     Input: nums = [0,3,7,2,5,8,4,6,0,1]
///     Output: 9
///
/// Constraints:
///     0 <= nums.length <= 105
///     -109 <= nums[i] <= 109
///

struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let nums: HashSet<i32> = nums.into_iter().collect();

        let mut result = 0;

        for n in nums.clone() {
            let left_neighbor = n.saturating_sub(1);

            if !nums.contains(&left_neighbor) {
                let right_neighbor = n + 1;

                if !nums.contains(&right_neighbor) {
                    result = result.max(1);
                    continue;
                }

                let mut seq_len = 1;
                while nums.contains(&(n + seq_len)) {
                    seq_len += 1;
                }
                result = result.max(seq_len);
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
        let nums: Vec<i32> = [100, 4, 200, 1, 3, 2].into();
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1].into();
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }
}
